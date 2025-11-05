#include <linux/module.h> 
#include <linux/kernel.h>
#include <linux/delay.h>

static int __init cr_init(void) {
    printk(KERN_INFO "Crash module loaded, your kernel will broke in \n");
    for(int i = 0; i < 10; i++) {
        ssleep(1);
        printk(KERN_INFO "%d\n", i);
    }
    *(int*)0 = 0xdeadbeef;
    return 0;
}

static void __exit cr_exit(void) {
    printk(KERN_INFO "Crash module unloaded((((\n");
    printk(KERN_INFO "It's not a bug, it's a feature. Were just wanted to write dump to 0 addr.\n");
}

module_init(cr_init);
module_exit(cr_exit);
MODULE_LICENSE("BSD-2-CLAUSE");