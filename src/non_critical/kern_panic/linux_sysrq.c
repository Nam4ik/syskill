#include <stdio.h> 
#include <stdlib.h>

void linux_sysrq_start() { 
    system("echo 1 > /proc/sys/kernel/sysrq"); 
}

void linux_sysrq_panic() {
    system("echo 1 > /proc/sysrq-trigger");
}