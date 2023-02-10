pub fn data_type() -> u8 {
    let x = 6;
    //x = 7; 不能对不可变的变量两次赋值，除非加mut
    x
}
pub fn constant_type() {
    //1.常量永远不变
    //２.声明常量用const关键字
    //3.常量可以在任何作用域声明
    //4.常量只能绑定到常量表达式，无法绑定到函数调用结果，或智能运行时才能算出的值
    //命名规范　MAX_NUMBER
    const MAX_NUMBER: i32 = 1000000;
    println!("number is {}", MAX_NUMBER);
}
pub fn shadow() {
    let str = "   ";
    let str = str.len();
    println!("length is {}", str)
}
