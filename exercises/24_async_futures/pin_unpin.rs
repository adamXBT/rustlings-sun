// 本练习考查以下内容：
// - Pin 的概念和用途
// - Unpin trait 的作用
// - 自引用结构体的处理
// - Pin<Box<dyn Future>> 的使用

use std::pin::Pin;
use std::marker::PhantomPinned;
use std::future::Future;

// TODO: 创建一个自引用结构体
// 这个结构体应该包含一个指向自身的引用
struct SelfReferential {
    // 添加必要的字段
    // 提示：需要包含一个指向自身的引用，以及 PhantomPinned 标记
    todo!("实现自引用结构体")
}

// TODO: 为 SelfReferential 实现必要的方法
impl SelfReferential {
    fn new() -> Pin<Box<Self>> {
        // 这个方法应该：
        // 1. 创建一个 SelfReferential 实例
        // 2. 将其包装在 Pin<Box<>> 中
        // 3. 设置自引用
        todo!("实现 new 方法")
    }

    fn get_value(&self) -> i32 {
        // 返回一个值，用于测试
        42
    }
}

// TODO: 创建一个包含 Future 的结构体
// 这个结构体应该能够安全地存储和调用 Future
struct FutureHolder {
    // 添加必要的字段来存储 Future
    // 提示：可以使用 Pin<Box<dyn Future<Output = T>>>
    todo!("实现 FutureHolder 结构体")
}

impl FutureHolder {
    fn new<F>(future: F) -> Self 
    where
        F: Future<Output = String> + 'static,
    {
        // 这个方法应该：
        // 1. 接受一个 Future
        // 2. 将其包装在 Pin<Box<>> 中
        // 3. 存储在结构体中
        todo!("实现 new 方法")
    }

    // TODO: 实现一个方法来执行 Future
    // 这个方法应该返回 Future 的结果
    async fn execute(&self) -> String {
        todo!("实现 execute 方法")
    }
}

// TODO: 创建一个函数来演示 Pin 的使用
fn demonstrate_pin() -> Pin<Box<dyn Future<Output = String>>> {
    // 这个函数应该：
    // 1. 创建一个异步闭包
    // 2. 将其包装在 Pin<Box<>> 中
    // 3. 返回包装后的 Future
    todo!("实现 demonstrate_pin 函数")
}

fn main() {
    println!("Pin 和 Unpin 练习");
    
    // 测试自引用结构体
    let pinned_self_ref = SelfReferential::new();
    println!("SelfReferential value: {}", pinned_self_ref.get_value());
    
    // 测试 Future 持有者
    // 注意：这里需要异步运行时来测试
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[test]
    fn test_self_referential() {
        let pinned = SelfReferential::new();
        assert_eq!(pinned.get_value(), 42);
    }

    #[tokio::test]
    async fn test_future_holder() {
        let future = async { "Hello from Future!".to_string() };
        let holder = FutureHolder::new(future);
        let result = holder.execute().await;
        assert_eq!(result, "Hello from Future!");
    }

    #[tokio::test]
    async fn test_demonstrate_pin() {
        let pinned_future = demonstrate_pin();
        let result = pinned_future.await;
        assert!(!result.is_empty());
    }
}
