//
//  main.c
//  chap3-2
//
//  Created by 川島寛隆 on 2023/07/17.
//
// POSIX セマフォ

#include <pthread.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <semaphore.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>


#define NUM_THREADS 10
#define NUM_LOOP 10 // スレッド内のループ数

int count = 0;  // 各スレッド内で増減させる値

void* th(void* arg) {
    // スレッド内で名前付きセマフォを生成
    sem_t* s = sem_open("/mysemaphore", 0);
    if (s == SEM_FAILED) {
        perror("sem_open");
        exit(1);
    }
    
    for (int i=0; i<NUM_LOOP; ++i) {
        // セマフォ待機
        if (sem_wait(s) == -1) {
            perror("sem_wait");
            exit(1);
        }
        // カウンターをアトミックに増加
        __sync_fetch_and_add(&count, 1);
        printf("count = %d\n", count);
        
        // 10ms待機
        usleep(10000);
        // カウンターをアトミックに削減
        __sync_fetch_and_sub(&count, 1);
        
        if (sem_post(s) == -1) {
            perror("sem_post");
            exit(1);
        }
    }
    
    // セマフォを閉じる
    if (sem_close(s) == -1) {
        perror("sem_close");
    }
    
    return NULL;
}


int main(void) {
    // 名前付きセマフォを開く
    // 名前付きセマフォの名前はOS全体。これを使えば、メモリを共有しないプロセス間でも容易にセマフォを実現できる。
    // O_CREAT ... すでに同名のセマフォがあるなら、開くだけ、無いなら生成
    // 0660 はパーミッション。ファイルのアレと同じ考え方。OSプロセスの所有者とグループが読み書き可能
    // 3 はロックを同時に獲得可能なプロセスの数
    sem_t* s = sem_open("/mysemaphore", O_CREAT, 0660, 3);
    if (s == SEM_FAILED) {
        perror("sem_open");
        return 1;
    }
    
    pthread_t v[NUM_THREADS];
    for (int i=0; i < NUM_THREADS; ++i) {
        pthread_create(&v[i], NULL, th, NULL);
    }
    
    for (int i=0; i<NUM_THREADS; ++i) {
        pthread_join(v[i], NULL);
    }
    
    // セマフォを閉じる。閉じるだけ
    if (sem_close(s) == -1) {
        perror("sem_close");
    }
    
    // セマフォを閉じただけではリソースが残ってしまうので、完全に削除する。
    if (sem_unlink("/mysemaphore") == -1) {
        perror("sem_unlink");
    }
    
    return 0;
}
