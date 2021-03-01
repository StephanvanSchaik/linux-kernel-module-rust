export KDIR ?= /lib/modules/$(shell uname -r)/build

CLANG ?= clang
ifeq ($(origin CC),default)
CC := ${CLANG}
endif

all: hello.ko

hello.ko:
	$(MAKE) -C $(KDIR) M=$(CURDIR) CC=$(CC) CONFIG_CC_IS_CLANG=y

clean:
	$(MAKE) -C $(KDIR) M=$(CURDIR) CC=$(CC) clean

install: hello.ko
	$(MAKE) INSTALL_MOD_DIR=kernel/drivers/hello -C $(KDIR) M=$(PWD) modules_install
	depmod -A
	sh -c "cat /etc/modules | grep -Fxq hello || echo hello >> /etc/modules"
