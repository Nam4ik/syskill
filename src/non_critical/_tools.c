#include <stdio.h>
#include <stdlib.h> 
#include <string.h>
#include <unistd.h>
#include <string.h>

int check_root() {
    if(getuid() == 0) {
        return 1;
    }
    return 0;
}

int check_pid(int pid) {
    if(getpid() == pid) {
        return 1;
    }
    return 0;
}


char* get_os_name() {
    FILE* file = fopen("/etc/os-release", "r");
    if (file == NULL) {
        perror("fopen");
        return NULL;
    }

    char line[256];
    while (fgets(line, sizeof(line), file)) {
        char* name = strstr(line, "NAME=");
        if (name != NULL) {
            name += 5; // skip "NAME="
            char* end = strchr(name, '\n');
            if (end != NULL) {
                *end = '\0';
            }
            return name;
        }
    }

    fclose(file);
    return NULL;
}