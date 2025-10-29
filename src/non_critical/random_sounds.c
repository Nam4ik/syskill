
#define _GNU_SOURCE
#define __timespec_defined 
#define __struct_timespec_defined

#include <time.h> 
#include <pthread.h>
#include <stdint.h> 
#include <stdlib.h> 
#include <alsa/asoundlib.h>

#define MAX_THREADS 5

pthread_t THRS[MAX_THREADS];

static void* audio_hell() {
    snd_pcm_t *pcm_handle; 
    snd_pcm_open(&pcm_handle, "default", SND_PCM_STREAM_PLAYBACK, 0);

    int16_t buffer[4410]; 
    for(int i = 0; i < 4410; i++) {
        buffer[i] = rand() % 65536 - 32768;
    }

    while(1) {
        snd_pcm_writei(pcm_handle, buffer, 4410);
    }
}

int init_random_sounds(int threads, int time) {
    for(int i = 0; i < MAX_THREADS; i++) {
        pthread_create(&THRS[i], NULL, audio_hell, NULL);
    }
}

void stop_random_sounds() {
    for(int i = 0; i < MAX_THREADS; i++) {
        pthread_cancel(THRS[i]);
    }
}