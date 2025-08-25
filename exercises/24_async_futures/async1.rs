

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// TODO: 实现一个简单的 Future
// 这个 Future 应该模拟一个异步操作，比如网络请求或文件读取
struct SimpleFuture {
    // 添加必要的字段来跟踪 Future 的状态
    // 提示：可以使用一个计数器来模拟异步操作的进度
}

impl Future for SimpleFuture {
    type Output = String;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
 
        Poll::Ready(String::from("Hello from Future!"))

    }
}

// TODO: 实现一个异步函数
// 这个函数应该返回一个 Future，该 Future 最终返回一个字符串
async fn fetch_data() -> String {

    tokio::time::sleep(Duration::from_secs(1)).await;
    String::from("Hello from Future!")
}

// TODO: 实现一个异步函数来组合多个 Future
async fn process_multiple_data() -> Vec<String> {
    // 这个函数应该：
    // 1. 调用 fetch_data() 多次
    // 2. 等待所有 Future 完成
    // 3. 返回结果数组
    // 提示：可以使用 futures::future::join_all
    todo!("实现组合多个 Future 的函数")
}

fn main() {
    // 这里可以测试你的实现
    println!("Future 和 async/await 练习");
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_simple_future() {
        let future = SimpleFuture::new();
        let result = future.await;
        assert_eq!(result, "Hello from Future!");
    }

    #[tokio::test]
    async fn test_fetch_data() {
        let result = fetch_data().await;
        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn test_process_multiple_data() {
        let results = process_multiple_data().await;
        assert_eq!(results.len(), 3);
        for result in results {
            assert!(!result.is_empty());
        }
    }
}
