//
//  main.c
//  chap2
//
//  Created by 川島寛隆 on 2023/07/16.
//

#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

#define NUM_THREADS 10  // 生成するスレッド数

void* thread_fn(void* arg) {
    // スレッド生成時に自動的に arg が渡される。
    int id = (int)arg;
    for (int i=0; i<5; ++i) {
        printf("id = %d, i = %d\n", id, i);
        sleep(1);
    }
    
    return "finish!";
}

int main(void) {
    pthread_t v[NUM_THREADS];
    
    for (int i=0; i<NUM_THREADS; ++i) {
        // pthread_create でスレッドを生成。戻り値が 0でないならエラー
        // 第3引数数で関数、第4引数で 第3引数で渡した関数の引数を渡している。
        if (pthread_create(&v[i], NULL, thread_fn, (void*)i) != 0) {
            perror("pthread create");
            return -1;
        }
    }
    
    for (int i=0; i<NUM_THREADS; ++i) {
        char* ptr;
        // pthread_joinでスレッド終了を待機
        // 第二引数が スレッド戻り値
        // pthread_joinしないとメモリリークしてしまうので、これは必須
        if (pthread_join(v[i], (void**)&ptr) == 0) {
            printf("message = %s\n", ptr);
        }
        else {
            perror("pthread_join");
            return -1;
        }
    }
    return 0;
}
