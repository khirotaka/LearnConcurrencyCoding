//
//  main.c
//  chap2-2
//
//  Created by 川島寛隆 on 2023/07/16.
//

#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

void* thread_fn(void* arg) {
    for (int i=0; i<5; ++i) {
        printf("i = %d\n", i);
        sleep(1);
    }
    
    return NULL;
}


int main(void) {
    // アトリビュートを初期化
    pthread_attr_t attr;
    if (pthread_attr_init(&attr) != 0) {
        perror("pthread_attr_init");
        return -1;
    }
    
    // デタッチスレッドに設定
    // スレッド終了時に自動的にスレッド用のリソースを開放してくれるようになる。
    if (pthread_attr_setdetachstate(&attr, PTHREAD_CREATE_DETACHED != 0)) {
        perror("pthread_attr_setdetachstate");
        return -1;
    }
    
    // アトリビュートを指定してスレッド生成
    pthread_t th;
    if (pthread_create(&th, &attr, thread_fn, NULL) != 0) {
        perror("pthread_create");
        return -1;
    }

    // アトリビュートを破棄
    if (pthread_attr_destroy(&attr) != 0) {
        perror("pthread_attr_destroy");
        return -1;
    }
    
    sleep(7);
    
    return 0;
}
