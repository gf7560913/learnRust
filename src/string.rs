pub fn get_string() {
    let a = String::from("hahahahha");
    let mut b = "hello".to_string();
    b.push_str(",world");
    println!("a is {},b is {:?}", a, b);
    let s1 = String::from("hi ");
    let s2 = String::from("brother");
    let s3 = s1 + &s2;
    println!("s3 is {}", s3);

    let o1 = "cn".to_string();
    let o2 = "hb".to_string();
    let o3 = "wh".to_string();
    let o4 = format!("{},{},{}", o1, o2, o3);
    println!("s4 is {}", o4);
}
//字符串分割
pub fn split_string() {
    let a = String::from("hahahahha");
    let b = &a[0..1];
    println!("b is {}", b);
}
