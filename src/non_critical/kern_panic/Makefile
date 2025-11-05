obj-m += linux_kmod.o

KDIR ?= /lib/modules/$(shell uname -r)/build
PWD := $(dir $(abspath $(firstword $(MAKEFILE_LIST))))

all:
	$(MAKE) -C $(KDIR) M=$(PWD) modules

clean:
	$(MAKE) -C $(KDIR) M=$(PWD) clean

.PHONY: all clean

