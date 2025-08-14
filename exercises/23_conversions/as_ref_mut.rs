// `AsRef` 和 `AsMut` 允许进行低成本的引用到引用的转换。
// 分别在 https://doc.rust-lang.org/std/convert/trait.AsRef.html 
// 和 https://doc.rust-lang.org/std/convert/trait.AsMut.html 
// 阅读更多关于它们的内容。 

// 获取给定参数中的字节数(bytes, 而非字符数)
// (`.len()` 方法返回的是字符串中的字节数)。
// TODO: 适当地添加 `AsRef` 特征作为特征约束(trait bound)。
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().len()
}

// 获取给定参数中的字符数(而非字节数)。
// TODO: 适当地添加 `AsRef` 特性作为特征约束。
fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// 使用 `as_mut()` 并对一个数字进行求平方操作。
// TODO: 添加特征约束。
fn num_sq<T: AsMut<u32>>(arg: &mut T) {
    // TODO: 实现函数体
    *arg.as_mut() = *arg.as_mut() * *arg.as_mut();
}

fn main() {
    // 对比示例：展示 & 和 as_ref() 的区别
    
    // 1. 使用 &str 的函数（限制性强）
    fn limited_counter(arg: &str) -> usize {
        arg.len()
    }
    
    // 2. 使用 AsRef<str> 的函数（灵活性强）
    fn flexible_counter<T: AsRef<str>>(arg: T) -> usize {
        arg.as_ref().len()
    }
    
    let s1 = "Hello";                    // &str
    let s2 = String::from("Hello");      // String
    let s3 = Box::new("Hello".to_string()); // Box<String>
    
    // 使用 limited_counter（只能接受 &str）
    println!("limited_counter(&str): {}", limited_counter(s1));           // ✅ 可以
    // println!("limited_counter(String): {}", limited_counter(s2));      // ❌ 编译错误！
    // println!("limited_counter(Box): {}", limited_counter(s3));        // ❌ 编译错误！
    
    // 使用 flexible_counter（可以接受多种类型）
    println!("flexible_counter(&str): {}", flexible_counter(s1));         // ✅ 可以
    println!("flexible_counter(String): {}", flexible_counter(s2));       // ✅ 可以
    println!("flexible_counter(Box): {}", flexible_counter(s3.as_ref())); // ✅ 可以，通过 as_ref()
    
    // 甚至可以直接传递所有权
    println!("flexible_counter(owned): {}", flexible_counter("World"));  // ✅ 可以
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mut_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
