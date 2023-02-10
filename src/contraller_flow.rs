pub fn else_flow() {
    let a = true;
    if a {
        println!("this is a")
    } else {
        println!("this is b")
    }
}
pub fn else_flow1() {
    let a = true;
    let m = if a { "a" } else { "b" };
    println!("this is {}", m)
}
pub fn loop_flow() {
    let mut count = 1;
    loop {
        if count > 10 {
            break;
        }
        println!("again");
        count += 1;
    }
}
pub fn while_flow() {
    let mut number = 3;
    while number > 1 {
        println!("again");
        number -= 1;
    }
}
pub fn for_flow() {
    let arr = [1, 2, 3, 4, 5];
    for v in arr.iter() {
        println!("this is {}", v)
    }
}
