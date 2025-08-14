// TODO: 通过添加一两个字符来修复编译器错误。
#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
}

macro_rules! add{
    ($a:expr, $b:expr) => {
        {
            $a + $b
        }
    };
    ($a:expr, $b:expr, $c:expr) => {
        {
            $a + $b + $c
        }
    };
}

// 使用标识符参数的宏示例
macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("这是函数: {}", stringify!($func_name));
        }
    };
}

fn main() {
    println!("{}", add!(1,2,3));
    println!("{}", add!(1,2));
    
    // 使用标识符参数创建函数
    create_function!(hello_world);
    create_function!(greet);
    
    // 调用生成的函数
    hello_world();
    greet();
}
