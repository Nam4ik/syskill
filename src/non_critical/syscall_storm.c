#define _GNU_SOURCE 
#define MAX_THREADS 5

#include <sys/syscall.h>
#include <unistd.h>
#include <stdlib.h>
#include <pthread.h>
#include <stdio.h>

static pthread_t THRS[MAX_THREADS];

static void* syscall_storm_linux(void* arg) {
  int iterations = *(int*)arg; 
  while(1) {
    for(int i = 0; i < iterations; i++) { 
      syscall(rand() % 400, rand() / rand()); 
      printf("%d. Called rand(), from thread %d\n", i, (int)(intptr_t)arg); 
    }
  }
  return NULL; 
}

int init_syscall_storm(int threads, int iterations) { 
  if (threads > MAX_THREADS) { 
    printf("ERR: Max treads 5, %d required\n", threads);
    return -1; 
  }

  for(int i = 0; i < threads; i++) { 
    int *thread_arg = malloc(sizeof(int)); 
        *thread_arg = iterations;
    if (pthread_create(&THRS[i], NULL, syscall_storm_linux, thread_arg) != 0) { 
      printf("ERR: Could not create thread %d\n", i);
      free(thread_arg); 
      return -1; 
    } 
  }
  return 0;
}

void stop_syscall_storm() {
  for(int i = 0; i < MAX_THREADS; i++) {
    pthread_cancel(THRS[i]);
  }
}
//TODO: Fix this and create random shit-syscall chooser. Maximaly shitcoded!!! Its system-killer, not a default program
//TODO: Add support for other *NIXes 


// cc -O0 -ffunction-sections -fdata-sections -fPIC -gdwarf-4 -fno-omit-frame-pointer -m64 -I src/non_critical -I src/critical/classic -std=c11 -o /home/namilskyy/suicidekit/target/debug/build/suicidekit-d9b57cca9def6d72/out/170d28c25dafbf6f-random_sounds.o -c src/non_critical/random_sounds.c


