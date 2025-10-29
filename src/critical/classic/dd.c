#include <stdlib.h>
#include <unistd.h>
#include <stdio.h>

void random_data_linux(char sym_drive[16], bool random) {
    char command[55]; 
    if (random) {
        sprintf(command, "dd if=/dev/urandom of=%s bs=1024 count=1024", sym_drive);
    } else {
        sprintf(command, "dd if=/dev/zero of=%s bs=1024 count=1024", sym_drive);
    }
    system(command);
}
