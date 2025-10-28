#define _GNU_SOURCE 
#define MAX_THREADS 5

#include <sys/syscall.h>
#include <unistd.h>
#include <stdlib.h>
#include <pthread.h>
#include <stdio.h>

pthread_t THRS[MAX_THREADS];


static void* syscall_storm_linux(void* iterations) {
  while(1) {
    for(int i = 0; 1 < iterations; i++) {
      syscall(rand() % 400, rand() / rand()); 
      printf("%d. Called rand(), from thread %d", i);
  }
}
}

int init(int threads, int iterations) { 
  if (threads > 5) {
    printf("ERR: Max treads 5, %d required", threads);
    return NULL; 
  }

  for(int i = 0; i < MAX_THREADS; i++) {
    int thread_args[i] = i;
    if (pthread_create(&THRS[i], NULL, syscall_storm_linux, thread_args) != 0) {
      printf("ERR: Could not create thread %d", i);
      return NULL;
    } 
  };
}

void stop() {
  for(int i = 0; i < MAX_THREADS; i++) {
    pthread_cancel(THRS[i]);
  }
}

//TODO: Fix this and create random shit-syscall chooser. Maximaly shitcoded!!! Its system-killer, not a default program
//TODO: Add support for other *NIXes 