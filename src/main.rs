#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle { //implementation blocks (we can have one or multiple impl blocks)
    fn area(&self) -> u32 { //associated functions
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool   //associated functions (METHOD)
    {
        self.width > other.width && self.height > other.height
    }

    fn square(size:u32) -> Self //associated function (NOT METHOD)
    {
        Self {
            width : size,
            height:size
        }
    }
}

fn main() {
    
    let r1 : Rectangle = Rectangle{height:10, width:dbg!(20+10)};

    println!("The rectangle is equal to {:#?} and its area equals {}",r1, r1.area());

    dbg!(&r1);

    //tried examples up until:
    //https://doc.rust-lang.org/book/ch05-03-method-syntax.html
    let r2 : Rectangle = Rectangle::square(100);
    dbg!(&r2);

}
