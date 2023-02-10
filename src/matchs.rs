#[derive(Debug)]
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(Usstate),
}
#[derive(Debug)]
pub enum Usstate {
    Alabma,
    NewYork,
}
pub fn value_in_center(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state is {:?}", state);
            25
        }
    }
}
/*
let p = matchs::Coin::Quarter(matchs::Usstate::NewYork);
    let a = matchs::value_in_center(p);
    println!("a is {}", a)
*/
