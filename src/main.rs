#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    
    let r1 : Rectangle = Rectangle{height:10,width:20};

    println!("The rectangle is equal to {:#?}",r1);
}
