// 你可以使用 `use` 和 `as` 关键字将模块路径引入作用域并为它们取个新名称(别名)。

// 选项1：私有模块，公开子模块（当前解决方案）
mod delicious_snacks {
    // TODO: 在修复以下两条 `use` 语句后将它们添加到作用域。
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;

    pub mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    pub mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
}

// 测试：在根模块中直接访问 delicious_snacks
fn test_module_access() {
    // ✅ 可以访问，因为在同一模块内
    let fruit = delicious_snacks::fruit;
    let veggie = delicious_snacks::veggie;
    println!("Test access: {} and {}", fruit, veggie);
}

// 选项2：公开模块，私有子模块（这样不行）
// pub mod delicious_snacks_public {
//     mod fruits_private {  // 私有，外部无法访问
//         pub const PEAR: &str = "Pear";
//     }
// }

// 选项3：完全公开（也可以工作）
// pub mod delicious_snacks_fully_public {
//     pub mod fruits {
//         pub const PEAR: &str = "Pear";
//     }
//     pub mod veggies {
//         pub const CUCUMBER: &str = "Cucumber";
//     }
// }

fn main() {
    // 测试模块访问
    test_module_access();
    
    // 使用 use 语句导入
    use delicious_snacks::fruits::*;
    use delicious_snacks::veggies::*;
    println!(
        "favorite snacks: {} and {}",
        APPLE,
        CARROT,
    );
}
