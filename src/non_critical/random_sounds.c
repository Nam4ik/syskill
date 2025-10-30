#define _GNU_SOURCE 
#define __timespec_defined
#define __stru

#include <time.h>
#include <alsa/asoundlib.h>
#include <stdint.h> 
#include <stdlib.h> 
#include <pthread.h>

#define MAX_THREADS 5

static pthread_t THRS[MAX_THREADS];

static void* audio_hell(void* _arg) {
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
    (void)time;
    if (threads > MAX_THREADS) threads = MAX_THREADS;
    for(int i = 0; i < threads; i++) {
        pthread_create(&THRS[i], NULL, audio_hell, NULL);
    }
    return 0;
}

void stop_random_sounds() {
    for(int i = 0; i < MAX_THREADS; i++) {
        pthread_cancel(THRS[i]);
    }
}