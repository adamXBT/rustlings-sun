// 本练习考查以下内容：
// - Arc (Atomic Reference Counting) 的使用
// - RwLock 的读写锁机制
// - Mutex 的互斥锁
// - 多线程环境下的数据共享
// - 并发安全的数据结构设计

use std::sync::{Arc, RwLock, Mutex};
use std::thread;
use std::time::Duration;

// TODO: 创建一个线程安全的数据结构
// 这个结构体应该能够在多个线程间安全地共享
struct SharedData {
    // 添加必要的字段
    // 提示：可以使用 Arc<RwLock<>> 或 Arc<Mutex<>>
    todo!("实现 SharedData 结构体")
}

impl SharedData {
    fn new() -> Arc<Self> {
        // 创建新的共享数据实例
        // 返回 Arc<Self> 以便在多个线程间共享
        todo!("实现 new 方法")
    }

    fn read_data(&self) -> String {
        // 读取数据
        // 提示：根据你选择的锁类型，实现相应的读取逻辑
        todo!("实现 read_data 方法")
    }

    fn write_data(&mut self, new_data: String) {
        // 写入数据
        // 提示：根据你选择的锁类型，实现相应的写入逻辑
        todo!("实现 write_data 方法")
    }

    fn get_data_count(&self) -> usize {
        // 获取数据的数量或长度
        todo!("实现 get_data_count 方法")
    }
}

// TODO: 创建一个函数来演示多线程读写
fn demonstrate_concurrent_access() {
    // 这个函数应该：
    // 1. 创建共享数据
    // 2. 启动多个读取线程
    // 3. 启动多个写入线程
    // 4. 等待所有线程完成
    // 5. 验证数据的完整性
    todo!("实现 demonstrate_concurrent_access 函数")
}

// TODO: 创建一个线程安全的计数器
struct ThreadSafeCounter {
    // 添加必要的字段来实现线程安全的计数器
    // 提示：可以使用 Arc<Mutex<usize>>
    todo!("实现 ThreadSafeCounter 结构体")
}

impl ThreadSafeCounter {
    fn new() -> Arc<Self> {
        // 创建新的计数器实例
        todo!("实现 new 方法")
    }

    fn increment(&self) {
        // 增加计数器的值
        todo!("实现 increment 方法")
    }

    fn decrement(&self) {
        // 减少计数器的值
        todo!("实现 decrement 方法")
    }

    fn get_value(&self) -> usize {
        // 获取当前计数器的值
        todo!("实现 get_value 方法")
    }
}

// TODO: 创建一个函数来测试计数器的并发安全性
fn test_counter_concurrency() {
    // 这个函数应该：
    // 1. 创建共享计数器
    // 2. 启动多个增加线程
    // 3. 启动多个减少线程
    // 4. 等待所有线程完成
    // 5. 验证最终结果
    todo!("实现 test_counter_concurrency 函数")
}

// TODO: 创建一个生产者-消费者模式
struct ProducerConsumer {
    // 添加必要的字段来实现生产者-消费者模式
    // 提示：可以使用 Arc<Mutex<Vec<String>>> 来存储数据
    // 可以使用 Arc<Mutex<bool>> 来控制生产/消费状态
    todo!("实现 ProducerConsumer 结构体")
}

impl ProducerConsumer {
    fn new() -> Arc<Self> {
        // 创建新的生产者-消费者实例
        todo!("实现 new 方法")
    }

    fn produce(&self, item: String) {
        // 生产一个项目
        todo!("实现 produce 方法")
    }

    fn consume(&self) -> Option<String> {
        // 消费一个项目
        // 如果没有项目可消费，返回 None
        todo!("实现 consume 方法")
    }

    fn is_empty(&self) -> bool {
        // 检查是否有项目可消费
        todo!("实现 is_empty 方法")
    }
}

// TODO: 创建一个函数来演示生产者-消费者模式
fn demonstrate_producer_consumer() {
    // 这个函数应该：
    // 1. 创建生产者-消费者实例
    // 2. 启动生产者线程
    // 3. 启动消费者线程
    // 4. 等待所有线程完成
    // 5. 验证所有项目都被正确处理
    todo!("实现 demonstrate_producer_consumer 函数")
}

fn main() {
    println!("并发原语练习");
    
    // 演示并发访问
    demonstrate_concurrent_access();
    
    // 测试计数器并发性
    test_counter_concurrency();
    
    // 演示生产者-消费者模式
    demonstrate_producer_consumer();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_data() {
        let shared_data = SharedData::new();
        
        // 测试基本功能
        shared_data.write_data("Test data".to_string());
        assert_eq!(shared_data.read_data(), "Test data");
        assert_eq!(shared_data.get_data_count(), 1);
    }

    #[test]
    fn test_counter() {
        let counter = ThreadSafeCounter::new();
        
        // 测试基本功能
        assert_eq!(counter.get_value(), 0);
        counter.increment();
        assert_eq!(counter.get_value(), 1);
        counter.decrement();
        assert_eq!(counter.get_value(), 0);
    }

    #[test]
    fn test_producer_consumer() {
        let pc = ProducerConsumer::new();
        
        // 测试基本功能
        assert!(pc.is_empty());
        pc.produce("Item 1".to_string());
        assert!(!pc.is_empty());
        
        let item = pc.consume();
        assert_eq!(item, Some("Item 1".to_string()));
        assert!(pc.is_empty());
    }

    #[test]
    fn test_concurrent_access() {
        // 这个测试需要运行实际的并发代码
        // 可以在这里添加并发测试
    }
}
