trait AppendBar {
    // 关联常量 - 有默认值
    const SUFFIX: &'static str = "bar";
    // 关联常量 - 必须实现
    const PREFIX: &'static str;
    // 关联类型
    type Output;
    
    fn append_bar(self) -> Self::Output;
}

// 为 String 实现 AppendBar
impl AppendBar for String {
    // 覆盖默认常量
    const SUFFIX: &'static str = "BAR";
    // 必须提供这个常量
    const PREFIX: &'static str = "Hello";
    // 指定输出类型
    type Output = String;
    
    fn append_bar(self) -> Self::Output {
        format!("{}{}{}", Self::PREFIX, self, Self::SUFFIX)
    }
}

// 为 &str 实现 AppendBar
impl AppendBar for &str {
    const PREFIX: &'static str = "Hi";
    type Output = String;
    
    fn append_bar(self) -> Self::Output {
        format!("{}{}{}", Self::PREFIX, self, Self::SUFFIX)
    }
}

// 为 Vec<String> 实现 AppendBar
impl AppendBar for Vec<String> {
    const PREFIX: &'static str = "";
    type Output = Vec<String>;
    
    fn append_bar(mut self) -> Self::Output {
        self.push("Bar".to_string());
        self
    }
}

fn main() {
    // 使用 String 实现
    let s1 = "World".to_string();
    let result1 = s1.append_bar();
    println!("String 结果: {}", result1);  // HelloWorldBAR
    
    // 使用 &str 实现
    let s2 = "Rust";
    let result2 = s2.append_bar();
    println!("&str 结果: {}", result2);    // HiRustbar
    
    // 直接访问常量
    println!("String 后缀: {}", String::SUFFIX);   // BAR
    println!("String 前缀: {}", String::PREFIX);  // Hello
    println!("&str 后缀: {}", <&str>::SUFFIX);    // bar
    println!("&str 前缀: {}", <&str>::PREFIX);   // Hi
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), "Bar");
        assert_eq!(foo.pop().unwrap(), "Foo");
    }
}
