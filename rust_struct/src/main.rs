#[derive(Debug)]
struct Rectangle {
    height:u64,
    width:u64
}

impl Rectangle {
    fn area(&self) -> u64 {
        self.width * self.height
    }
}
fn main() {
    let rect1 = Rectangle{
        height:300,
        width:400
    };
   // let react1_area = area(&rect1);
    println!("{:?}",rect1);
}

// fn area(rect:&Rectangle)-> u64 {
//     rect.height * rect.width
// }
