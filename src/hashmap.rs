use std::collections::HashMap;

pub fn hash_map() {
    let mut map = HashMap::new();
    map.insert(String::from("hello"), 10);
    println!("map is {:?}", map)
}
//collect方法创建hashmap
pub fn vec_map() {
    let scores = vec![10, 50];
    let color = vec![String::from("red"), String::from("blue")];
    let map: HashMap<_, _> = color.iter().zip(scores.iter()).collect();
    println!("map is {:?}", map)
}
pub fn get_map() {
    let scores = vec![10, 50];
    let color = vec![String::from("red"), String::from("blue")];
    let map: HashMap<_, _> = color.iter().zip(scores.iter()).collect();
    let c = "red".to_string();
    let num = map.get(&c);
    match num {
        Some(s) => {
            println!("num is {}", s)
        }
        None => {
            println!("the key is not exist")
        }
    }
}
pub fn for_map() {
    let scores = vec![10, 50];
    let color = vec![String::from("red"), String::from("blue")];
    let map: HashMap<_, _> = color.iter().zip(scores.iter()).collect();
    for (k, v) in map {
        println!("key is {},value is {}", k, v)
    }
}
