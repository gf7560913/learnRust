pub fn slice() {
    let word = String::from("hello world");
    let a = &word[0..5];
    let b = &word[6..];
    let c = &word[..];
    println!("a is {},b is {},c is {}", a, b, c);
}
