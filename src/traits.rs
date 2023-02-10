use std::fmt::{Debug, Display};

pub trait action {
    fn run(&self) -> String;
    fn walk(&self) -> String;
}

pub struct human {}
pub struct dog {}
impl action for human {
    fn run(&self) -> String {
        println!("human run!!!");
        let s = String::from("hello");
        s
    }
    fn walk(&self) -> String {
        println!("human walk!!!");
        let s = String::from("world");
        s
    }
}

impl action for dog {
    fn run(&self) -> String {
        println!("dog run!!!");
        let s = String::from("hello");
        s
    }
    fn walk(&self) -> String {
        println!("dog walk!!!");
        let s = String::from("world");
        s
    }
}

//trait as param
//impl trait 适用于简单实现
pub fn get_trait(item: impl action) {
    println!("which struct impl {}", item.walk())
}

//trait bound 语法，适用复杂情况
pub fn get_trait1<T: action>(item: T) {
    println!("which struct impl {}", item.walk());
}
//一个函数需要传入多个接口的实现
pub fn notify<T: action + Display>() {
    println!("which struct impl ");
}
//传入两个参数，两个参数分别是两个接口的实现
pub fn notify2<T: action + Display, U: Clone + Debug>() {
    println!("which struct impl ");
}

//使用where来约束
pub fn notify3<T, U>()
where
    T: action + Display,
    U: Clone + Debug,
{
    println!("which struct impl ");
}

//trait作为返回类型
pub fn notify4() -> impl action {
    //注意：使用trait作为返回类型的时候，只能确定返回一种类型
    let d = dog {};
    d
}
