//
//  main.c
//  chap3-3
//
//  Created by 川島寛隆 on 2023/07/19.
//
// 条件変数

#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>

pthread_mutex_t mut = PTHREAD_MUTEX_INITIALIZER;
pthread_cond_t cond = PTHREAD_COND_INITIALIZER;

volatile bool ready = false;
char buf[256];  // スレッド間でデータを受け渡すため

// データ生成担当
void* producer(void* arg) {
    printf("producer: ");
    fgets(buf, sizeof(buf), stdin);
    
    pthread_mutex_lock(&mut);
    ready = true;
    
    if (pthread_cond_broadcast(&cond) != 0) {
        perror("pthread_cond_broadcast");
        exit(-1);
    }
    
    pthread_mutex_unlock(&mut);
    return NULL;
}

// データ消費担当
void* consumer(void* arg) {
    // 条件変数を読み込むためにまずロックを獲得。
    pthread_mutex_lock(&mut);
    while (!ready) {
        if (pthread_cond_wait(&cond, &mut) != 0) {
            perror("pthread_cond_wait");
            exit(-1);
        }
    }
    
    pthread_mutex_unlock(&mut);
    printf("consumer: %s\n", buf);
    return NULL;
}


int main(void) {
    pthread_t pr, cn;
    pthread_create(&pr, NULL, producer, NULL);
    pthread_create(&cn, NULL, consumer, NULL);
    
    pthread_join(pr, NULL);
    pthread_join(cn, NULL);
    
    pthread_mutex_destroy(&mut);
    
    if (pthread_cond_destroy(&cond) != 0) {
        perror("pthread_cond_destory");
        return -1;
    }
    
    return 0;
}
