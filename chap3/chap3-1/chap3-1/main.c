//
//  main.c
//  chap3-1
//
//  Created by 川島寛隆 on 2023/07/17.
//
// Pthreadsのミューテックス


#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>

// mutex用共有変数
pthread_mutex_t mut = PTHREAD_MUTEX_INITIALIZER;

void* some_func(void* arg) {
    if (pthread_mutex_lock(&mut) != 0) {
        perror("pthread_mutex_lock");
        exit(-1);
    }
    
    printf("mutex lock!\n");
    
    if (pthread_mutex_unlock(&mut) != 0) {
        perror("pthread_mutex_unlock");
        exit(-1);
    }
    
    return NULL;
}


int main(void) {
    pthread_t th1, th2;
    // スレッド生成 1
    if (pthread_create(&th1, NULL, some_func, NULL) != 0) {
        perror("pthread_create");
        return -1;
    }
    
    // スレッド生成 2
    if (pthread_create(&th2, NULL, some_func, NULL) != 0) {
        perror("pthread_create");
        return -1;
    }
    
    // スレッド1 待機
    if (pthread_join(th1, NULL) != 0) {
        perror("pthread_join");
        return -1;
    }
    // スレッド2 待機
    if (pthread_join(th2, NULL) != 0) {
        perror("pthread_join");
        return -1;
    }
    
    // ミューテックスオブジェクトを開放
    if (pthread_mutex_destroy(&mut) != 0) {
        perror("pthread_mutex_destory");
        return -1;
    }
    
    return 0;
}
