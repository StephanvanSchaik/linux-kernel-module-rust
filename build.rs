use flate2::bufread::GzDecoder;
use std::collections::HashSet;
use std::io::{BufRead, BufReader, Cursor, Read};
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

const INCLUDED_TYPES: &[&str] = &[
    "file_system_type",
    "mode_t",
    "umode_t",
    "ctl_table",
    "kprobe_instance",
    "class",
    "device",
    "proc_ops",
    "proc_dir_entry",
    "usb_interface",
    "usb_device_id",
    "usb_driver",
    "usbnet",
    "mm_struct",
    "task_struct",
    "vma_area_struct",
    "rw_semaphore",
    "spinlock_t",
    "pgd_t",
    "pgdval_t",
    "p4d_t",
    "p4dval_t",
    "pud_t",
    "pudval_t",
    "pmd_t",
    "pmdval_t",
    "pte_t",
    "pteval_t",
    "pid_t",
];
const INCLUDED_FUNCTIONS: &[&str] = &[
    "cdev_add",
    "cdev_init",
    "cdev_del",
    "register_filesystem",
    "unregister_filesystem",
    "krealloc",
    "kfree",
    "mount_nodev",
    "kill_litter_super",
    "register_sysctl",
    "unregister_sysctl_table",
    "access_ok",
    "_copy_to_user",
    "_copy_from_user",
    "alloc_chrdev_region",
    "unregister_chrdev_region",
    "wait_for_random_bytes",
    "get_random_bytes",
    "rng_is_initialized",
    "printk",
    "add_device_randomness",
    "__class_create",
    "class_destroy",
    "device_create",
    "device_destroy",
    "proc_mkdir_mode",
    "proc_create",
    "proc_remove",
    "find_vma",
    "down_read",
    "down_write",
    "up_read",
    "up_write",
    "register_kretprobe",
    "unregister_kretprobe",
    "usb_register_driver",
    "usb_deregister",
    "usbnet_disconnect",
    "usbnet_get_endpoints",
    "usbnet_probe",
    "find_vpid",
    "pid_task",
];
const INCLUDED_VARS: &[&str] = &[
    "EINVAL",
    "ENOMEM",
    "ESPIPE",
    "EFAULT",
    "EAGAIN",
    "ENOENT",
    "__this_module",
    "FS_REQUIRES_DEV",
    "FS_BINARY_MOUNTDATA",
    "FS_HAS_SUBTYPE",
    "FS_USERNS_MOUNT",
    "FS_RENAME_DOES_D_MOVE",
    "BINDINGS_GFP_KERNEL",
    "KERN_INFO",
    "VERIFY_WRITE",
    "LINUX_VERSION_CODE",
    "SEEK_SET",
    "SEEK_CUR",
    "SEEK_END",
    "O_NONBLOCK",
    "phys_base",
    "page_offset_base",
    "current_task",
];
const OPAQUE_TYPES: &[&str] = &[
    // These need to be opaque because they're both packed and aligned, which rustc
    // doesn't support yet. See https://github.com/rust-lang/rust/issues/59154
    // and https://github.com/rust-lang/rust-bindgen/issues/1538
    "desc_struct",
    "xregs_state",
    "u64_stats_sync",
    "lock_class_key",
    "dev_archdata",
    "mod_arch_specific",
];
const CONFIG_NAMES: &[&str] = &[
    "CONFIG_PAGE_TABLE_ISOLATION",
];

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const ARCH_TYPES: &[&str] = &[
    "cpuinfo_x86",
];

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const ARCH_VARS: &[&str] = &[
    "boot_cpu_data",
    "NCAPINTS",
    "NBUGINTS",
    "X86_FEATURE_LA57",
];

fn handle_kernel_version_cfg(bindings_path: &PathBuf) {
    let f = BufReader::new(fs::File::open(bindings_path).unwrap());
    let mut version = None;
    for line in f.lines() {
        let line = line.unwrap();
        if let Some(type_and_value) = line.split("pub const LINUX_VERSION_CODE").nth(1) {
            if let Some(value) = type_and_value.split('=').nth(1) {
                let raw_version = value.split(';').next().unwrap();
                version = Some(raw_version.trim().parse::<u64>().unwrap());
                break;
            }
        }
    }
    let version = version.expect("Couldn't find kernel version");
    let (major, minor) = match version.to_be_bytes() {
        [0, 0, 0, 0, 0, major, minor, _patch] => (major, minor),
        _ => panic!("unable to parse LINUX_VERSION_CODE {:x}", version),
    };

    if major >= 6 {
        panic!("Please update build.rs with the last 5.x version");
        // Change this block to major >= 7, copy the below block for
        // major >= 6, fill in unimplemented!() for major >= 5
    }
    if major >= 5 {
        for x in 0..=if major > 5 { unimplemented!() } else { minor } {
            println!("cargo:rustc-cfg=kernel_5_{}_0_or_greater", x);
        }
    }
    if major >= 4 {
        // We don't currently support anything older than 4.4
        for x in 4..=if major > 4 { 20 } else { minor } {
            println!("cargo:rustc-cfg=kernel_4_{}_0_or_greater", x);
        }
    }
}

fn handle_kernel_symbols_cfg(symvers_path: &PathBuf) {
    let f = BufReader::new(fs::File::open(symvers_path).unwrap());
    for line in f.lines() {
        let line = line.unwrap();
        if let Some(symbol) = line.split_ascii_whitespace().nth(1) {
            if symbol == "setfl" {
                println!("cargo:rustc-cfg=kernel_aufs_setfl");
                break;
            }
        }
    }
}

fn parse_kernel_config<R: BufRead>(reader: R, whitelist: &[&str]) {
    let whitelist = whitelist
        .into_iter()
        .map(|s| *s)
        .collect::<HashSet<&str>>();
    let enabled = vec!["y", "Y", "m", "M"]
        .into_iter()
        .collect::<HashSet<&str>>();

    for line in reader.lines() {
        let line = line.unwrap();

        // Ignore comments.
        if line.starts_with("#") {
            continue;
        }

        // Split the line at the assignment.
        let split = line
            .splitn(2, "=")
            .map(|s| s.trim())
            .collect::<Vec<&str>>();

        // Too few elements.
        if split.len() < 2 {
            continue;
        }

        // The number of page table levels is numeric, so handle it like the kernel version.
        if split[0] == "CONFIG_PGTABLE_LEVELS" {
            let value = match split[1].parse::<usize>() {
                Ok(n) => n,
                _ => continue,
            };

            if value >= 5 {
                println!("cargo:rustc-cfg=pgtable_levels_5_or_greater");
            }

            if value >= 4 {
                println!("cargo:rustc-cfg=pgtable_levels_4_or_greater");
            }

            if value >= 3 {
                println!("cargo:rustc-cfg=pgtable_levels_3_or_greater");
            }

            continue;
        }

        // Do we care about this config option?
        if !whitelist.contains(split[0]) {
            continue;
        }

        // Has it been enabled?
        if !enabled.contains(split[1]) {
            continue;
        }

        // Expose it to Rust.
        let name = match split[0].splitn(2, "_").last() {
            Some(name) => name,
            _ => continue,
        };

        println!("cargo:rustc-cfg={}", name.to_lowercase());
    }
}

fn handle_kernel_config(path: &PathBuf, whitelist: &[&str]) {
    let f = BufReader::new(fs::File::open(path).unwrap());
    parse_kernel_config(f, whitelist);
}

fn handle_gzip_kernel_config(path: &PathBuf, whitelist: &[&str]) {
    // Decompress the file.
    let mut gz = GzDecoder::new(BufReader::new(fs::File::open(path).unwrap()));
    let mut s = String::new();
    gz.read_to_string(&mut s).unwrap();

    // Pass it as a BufRead.
    let f = BufReader::new(Cursor::new(s));

    parse_kernel_config(f, whitelist);
}

fn handle_kernel_config_cfg(whitelist: &[&str]) {
    let mut config_paths = vec![
        PathBuf::from("/proc/config"),
        PathBuf::from("/proc/config.gz"),
    ];

    if let Ok(uname) = Command::new("uname")
        .arg("-r")
        .output() {
        let version = std::str::from_utf8(&uname.stdout).unwrap();
        config_paths.push(PathBuf::from(format!("/boot/config-{}", version)));
    }

    config_paths.push(PathBuf::from(env::var("KDIR").unwrap()).join(".config"));

    for config_path in &config_paths {
        if !config_path.exists() {
            continue;
        }

        let extension = match config_path.extension() {
            Some(extension) => extension.to_str().unwrap(),
            _ => "",
        };

        if extension == "gz" {
            handle_gzip_kernel_config(&config_path, whitelist);
        } else {
            handle_kernel_config(&config_path, whitelist);
        }

        break;
    }
}

// Takes the CFLAGS from the kernel Makefile and changes all the include paths to be absolute
// instead of relative.
fn prepare_cflags(cflags: &str, kernel_dir: &str) -> Vec<String> {
    let cflag_parts = shlex::split(&cflags).unwrap();
    let mut cflag_iter = cflag_parts.iter();
    let mut kernel_args = vec![];
    while let Some(arg) = cflag_iter.next() {
        if arg.starts_with("-I") && !arg.starts_with("-I/") {
            kernel_args.push(format!("-I{}/{}", kernel_dir, &arg[2..]));
        } else if arg == "-include" {
            kernel_args.push(arg.to_string());
            let include_path = cflag_iter.next().unwrap();
            if include_path.starts_with('/') {
                kernel_args.push(include_path.to_string());
            } else {
                kernel_args.push(format!("{}/{}", kernel_dir, include_path));
            }
        } else {
            kernel_args.push(arg.to_string());
        }
    }
    kernel_args
}

fn main() {
    println!("cargo:rerun-if-env-changed=CC");
    println!("cargo:rerun-if-env-changed=KDIR");
    println!("cargo:rerun-if-env-changed=c_flags");

    let kernel_dir = env::var("KDIR").expect("Must be invoked from kernel makefile");
    let kernel_cflags = env::var("c_flags").expect("Add 'export c_flags' to Kbuild");
    let kbuild_cflags_module =
        env::var("KBUILD_CFLAGS_MODULE").expect("Must be invoked from kernel makefile");

    let cflags = format!("{} {}", kernel_cflags, kbuild_cflags_module);
    let kernel_args = prepare_cflags(&cflags, &kernel_dir);

    let target = env::var("TARGET").unwrap();

    let mut builder = bindgen::Builder::default()
        .use_core()
        .ctypes_prefix("c_types")
        .derive_default(true)
        .size_t_is_usize(true)
        .rustfmt_bindings(true);

    builder = builder.clang_arg(format!("--target={}", target));
    for arg in kernel_args.iter() {
        builder = builder.clang_arg(arg.clone());
    }

    println!("cargo:rerun-if-changed=src/bindings_helper.h");
    builder = builder.header("src/bindings_helper.h");

    for t in INCLUDED_TYPES {
        builder = builder.whitelist_type(t);
    }
    for f in INCLUDED_FUNCTIONS {
        builder = builder.whitelist_function(f);
    }
    for v in INCLUDED_VARS {
        builder = builder.whitelist_var(v);
    }
    for t in OPAQUE_TYPES {
        builder = builder.opaque_type(t);
    }
    builder = builder.constified_enum_module("pid_type");

    for t in ARCH_TYPES {
        builder = builder.whitelist_type(t);
    }
    for v in ARCH_VARS {
        builder = builder.whitelist_var(v);
    }

    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    handle_kernel_version_cfg(&out_path.join("bindings.rs"));
    handle_kernel_symbols_cfg(&PathBuf::from(&kernel_dir).join("Module.symvers"));
    handle_kernel_config_cfg(CONFIG_NAMES);

    let mut builder = cc::Build::new();
    builder.compiler(env::var("CC").unwrap_or_else(|_| "clang".to_string()));
    builder.target(&target);
    builder.warnings(false);
    println!("cargo:rerun-if-changed=src/helpers.c");
    builder.file("src/helpers.c");
    for arg in kernel_args.iter() {
        builder.flag(&arg);
    }
    builder.compile("helpers");
}
