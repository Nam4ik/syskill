#include <stdlib.h>
#include <unistd.h>
#include <stdio.h>

void random_data_linux(char sym_drive[16]) {
    char command[55]; 
    sprintf(command, "dd if=/dev/zero of=%s bs=1024 count=1024", sym_drive);
    system(command);
}
