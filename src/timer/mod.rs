use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;

// 시간단축 / 시간증가가 구현된 타이머
// Arc<Timer> 꼴로 들고 다녀야 함
pub struct Timer {
    seconds: Mutex<i32>
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            seconds: Mutex::new(0)
        }
    }

    pub async fn set(&self, seconds: i32) {
        *self.seconds.lock().await = seconds;
    }
    pub async fn increase(&self, seconds: i32) {
        *self.seconds.lock().await += seconds;
    }
    pub async fn decrease(&self, seconds: i32) {
        *self.seconds.lock().await -= seconds;
    }
    pub async fn run(&self, seconds: i32) {
        self.set(seconds).await;
        
        while *self.seconds.lock().await > 0 {
            *self.seconds.lock().await -= 1;

            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
}

#[tokio::test]
async fn test() {
    let timer = Arc::new(Timer::new());
    
    let timer_ = timer.clone();
    let handle = tokio::spawn(async move {
        timer_.run(10).await;
        println!("Timer finished!");
    });

    tokio::time::sleep(Duration::from_secs(3)).await;
    timer.increase(3).await;

    tokio::time::sleep(Duration::from_secs(3)).await;
    timer.decrease(8).await;

    handle.await.unwrap_or(());
}