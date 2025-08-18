

trait Mointer {
    fn name() -> String;
    
    fn mointer() -> String {
        println!("mointer {}", Self::name());
        Self::name()
    }
    
    fn new() -> Self;
}

pub struct Cat;

impl Mointer for Cat {
    fn name() -> String {
        String::from("Cat")
    }
    
    fn new() -> Self {
        Cat
    }
}

pub struct Dog;

impl Mointer for Dog {
    fn name() -> String {
        String::from("Dog")
    }
    
    fn new() -> Self {
        Dog
    }
}

fn main(){
    let cat=Cat::mointer();
    let dog=Dog::mointer();
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_mointer(){
        let cat=Cat::mointer();
        let dog=Dog::mointer();
        assert_eq!(cat,"Cat");
        assert_eq!(dog,"Dog");
    }
}