// 本练习考查以下内容：
// - Trait 对象的概念
// - 类型擦除的实现
// - Box<dyn Trait> 的使用
// - 动态分发 vs 静态分发

// TODO: 定义一个 trait 来表示可序列化的数据
trait Serializable {
    fn serialize(&self) -> String;
    fn deserialize(data: &str) -> Self;
}

// TODO: 为不同的类型实现 Serializable trait
// 实现 String 类型
impl Serializable for String {
    fn serialize(&self) -> String {
        // 简单的序列化：直接返回字符串
        todo!("实现 String 的序列化")
    }

    fn deserialize(data: &str) -> Self {
        // 简单的反序列化：直接返回字符串
        todo!("实现 String 的反序列化")
    }
}

// TODO: 实现 i32 类型
impl Serializable for i32 {
    fn serialize(&self) -> String {
        // 将数字转换为字符串
        todo!("实现 i32 的序列化")
    }

    fn deserialize(data: &str) -> Self {
        // 将字符串解析为数字
        todo!("实现 i32 的反序列化")
    }
}

// TODO: 创建一个结构体来存储不同类型的 Serializable 对象
struct DataContainer {
    // 添加必要的字段来存储 trait 对象
    // 提示：可以使用 Vec<Box<dyn Serializable>>
    todo!("实现 DataContainer 结构体")
}

impl DataContainer {
    fn new() -> Self {
        // 创建新的容器
        todo!("实现 new 方法")
    }

    fn add<T: Serializable + 'static>(&mut self, item: T) {
        // 添加一个可序列化的项目
        // 提示：需要将具体类型转换为 trait 对象
        todo!("实现 add 方法")
    }

    fn serialize_all(&self) -> Vec<String> {
        // 序列化所有项目
        todo!("实现 serialize_all 方法")
    }

    fn get_serialized_size(&self) -> usize {
        // 获取所有序列化后数据的总大小
        todo!("实现 get_serialized_size 方法")
    }
}

// TODO: 创建一个函数来演示 trait 对象的使用
fn demonstrate_trait_objects() -> Vec<Box<dyn Serializable>> {
    // 这个函数应该：
    // 1. 创建不同类型的 Serializable 对象
    // 2. 将它们包装在 Box<dyn Serializable> 中
    // 3. 返回 trait 对象数组
    todo!("实现 demonstrate_trait_objects 函数")
}

// TODO: 创建一个泛型函数来演示静态分发
fn process_static<T: Serializable>(item: &T) -> String {
    // 这个函数使用静态分发
    // 在编译时就知道具体的类型
    todo!("实现 process_static 函数")
}

// TODO: 创建一个函数来演示动态分发
fn process_dynamic(item: &dyn Serializable) -> String {
    // 这个函数使用动态分发
    // 在运行时才知道具体的类型
    todo!("实现 process_dynamic 函数")
}

fn main() {
    println!("Trait 对象和类型擦除练习");
    
    // 测试数据容器
    let mut container = DataContainer::new();
    container.add("Hello".to_string());
    container.add(42);
    
    let serialized = container.serialize_all();
    println!("Serialized data: {:?}", serialized);
    
    // 演示 trait 对象
    let trait_objects = demonstrate_trait_objects();
    for obj in trait_objects {
        println!("Serialized: {}", obj.serialize());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_serialization() {
        let data = "Hello World".to_string();
        let serialized = data.serialize();
        let deserialized = String::deserialize(&serialized);
        assert_eq!(data, deserialized);
    }

    #[test]
    fn test_i32_serialization() {
        let data = 42;
        let serialized = data.serialize();
        let deserialized = i32::deserialize(&serialized);
        assert_eq!(data, deserialized);
    }

    #[test]
    fn test_data_container() {
        let mut container = DataContainer::new();
        container.add("Test".to_string());
        container.add(123);
        
        let serialized = container.serialize_all();
        assert_eq!(serialized.len(), 2);
        assert!(serialized.contains(&"Test".to_string()));
        assert!(serialized.contains(&"123".to_string()));
    }

    #[test]
    fn test_trait_objects() {
        let objects = demonstrate_trait_objects();
        assert!(!objects.is_empty());
        
        for obj in objects {
            let serialized = obj.serialize();
            assert!(!serialized.is_empty());
        }
    }

    #[test]
    fn test_static_vs_dynamic() {
        let string_data = "Hello".to_string();
        let int_data = 42;
        
        let static_result = process_static(&string_data);
        let dynamic_result = process_dynamic(&string_data);
        
        assert_eq!(static_result, dynamic_result);
    }
}
