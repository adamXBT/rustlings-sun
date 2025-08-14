
use std::{
    thread,
    time::{Duration, Instant},
};

fn main() {
    // 创建线程句柄容器
    let mut handles = Vec::new();
    
    // 创建10个线程
    for i in 0..10 {
        let handle = thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("Thread {i} done");
            start.elapsed().as_millis()
        });
        handles.push(handle);
    }

    // 等待所有线程完成并收集结果
    let mut results = Vec::new();
    for handle in handles {
        let result: u128 = handle.join().unwrap();  // 修正为 u128
        results.push(result);
    }

    // 验证所有线程都完成了
    if results.len() != 10 {
        panic!("Oh no! Some thread isn't done yet!");
    }

    // 打印每个线程的执行时间
    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("Thread {i} took {result}ms");
    }
}
