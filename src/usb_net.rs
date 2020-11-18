use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use core::mem;

use crate::bindings;
use crate::c_types;
use crate::error::{Error, KernelResult};
use crate::types::CStr;
use crate::usb;

unsafe extern "C" fn bind_callback<T: DriverInfo>(
    dev: *mut bindings::usbnet,
    interface: *mut bindings::usb_interface,
) -> c_types::c_int {
    let dev = Device::from_raw(dev);
    let interface = usb::Interface::from_raw(interface);

    let driver_info = match T::bind(&dev, &interface) {
        Ok(driver_info) => Box::new(driver_info),
        Err(error) => return error.to_kernel_errno(),
    };

    (*dev.raw).data[0] = Box::into_raw(driver_info) as c_types::c_ulong;

    0
}

unsafe extern "C" fn unbind_callback<T: DriverInfo>(
    dev: *mut bindings::usbnet,
    interface: *mut bindings::usb_interface,
) {
    let dev = Device::from_raw(dev);
    let interface = usb::Interface::from_raw(interface);

    let driver_info = Box::from_raw((*dev.raw).data[0] as *mut T);

    driver_info.unbind(&dev, &interface);
}

pub struct Device {
    raw: *mut bindings::usbnet,
}

impl Device {
    pub(crate) fn from_raw(
        raw: *mut bindings::usbnet,
    ) -> Self {
        Self {
            raw,
        }
    }

    pub fn get_endpoints(
        &self,
        interface: &usb::Interface,
    ) -> KernelResult<()> {
        let result = unsafe {
            bindings::usbnet_get_endpoints(self.raw, interface.raw_mut())
        };

        if result < 0 {
            return Err(Error::from_kernel_errno(result));
        }

        Ok(())
    }
}

pub trait DriverInfo: Sync + Sized {
    fn bind(
        _dev: &Device,
        _interface: &usb::Interface,
    ) -> KernelResult<Self>;

    fn unbind(
        &self,
        _dev: &Device,
        _interface: &usb::Interface,
    ) {
    }
}

pub struct DriverBuilder {
    driver_info: Vec<Box<bindings::driver_info>>,
    products: Vec<bindings::usb_device_id>,
}

impl DriverBuilder {
    pub fn new() -> Self {
        Self {
            driver_info: vec![],
            products: vec![],
        }
    }

    pub fn add<T: DriverInfo>(
        mut self,
        vendor: u16,
        product: u16,
    ) -> Self {
        let driver_info = Box::new(bindings::driver_info {
            bind: Some(bind_callback::<T>),
            unbind: Some(unbind_callback::<T>),
            ..Default::default()
        });

        let product = bindings::usb_device_id {
            idVendor: vendor,
            idProduct: product,
            driver_info: &(*driver_info) as *const _ as c_types::c_ulong,
            ..Default::default()
        };

        self.driver_info.push(driver_info);
        self.products.push(product);

        self
    }

    pub fn register(
        mut self,
        name: CStr<'static>,
    ) -> KernelResult<Driver> {
        self.products.push(unsafe { mem::zeroed() });

        let table = self.products.into_boxed_slice();

        let mut driver = Box::new(bindings::usb_driver {
            name: name.as_ptr() as *const i8,
            id_table: table.as_ptr(),

            probe: Some(bindings::usbnet_probe),
            disconnect: Some(bindings::usbnet_disconnect),

            ..Default::default()
        });

        unsafe {
            bindings::usb_register_driver(
                driver.as_mut(),
                &mut bindings::__this_module,
                name.as_ptr() as *const i8,
            )
        };

        Ok(Driver {
            _driver_info: self.driver_info,
            _table: table,
            driver: driver,
        })
    }
}

pub struct Driver {
    _driver_info: Vec<Box<bindings::driver_info>>,
    _table: Box<[bindings::usb_device_id]>,
    driver: Box<bindings::usb_driver>,
}

impl Drop for Driver {
    fn drop(&mut self) {
        unsafe {
            bindings::usb_deregister(self.driver.as_mut());
        }
    }
}
