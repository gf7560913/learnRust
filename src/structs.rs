#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}
pub fn area() {
    let rect = Rectangle {
        length: 30,
        width: 50,
    };
    let a = &rect.width * &rect.length;
    println!("area is {:?}", a);
    println!("area is {:?}", rect);
    println!("area is {:#?}", rect);
}
/*
let c = structs::Circle { Radius: 10 };
   let a = c.area();
   println!("area is {}", a)
   */
pub struct Circle {
    pub Radius: u32,
}

impl Circle {
    pub fn area(self) -> f32 {
        let r = f32::from(self.Radius as f32);
        3.14 * r * r
    }

    pub fn new(r: u32) -> Circle {
        Circle { Radius: r }
    }
}
