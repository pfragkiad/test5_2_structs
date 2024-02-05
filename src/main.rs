#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle { //implementation blocks
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool 
    {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    
    let r1 : Rectangle = Rectangle{height:10, width:dbg!(20+10)};

    println!("The rectangle is equal to {:#?} and its area equals {}",r1, r1.area());

    dbg!(&r1);

}
