#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    fn can_hold(&self, other: Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
fn main() {
    let r1 = Rectangle {
        width: 2,
        height: 3,
    };

    let r2 = Rectangle {
        width: r1.width * 2,
        height: r1.width * 2,
    };

    println!("Area of R1 : {}", r1.area());
    println!("Perimiter of R1 : {}", r1.perimeter());
    println!("Area of R2 : {}", r2.area());
    print!("R2 can hold R1 ? {}", r2.can_hold(r1))
}
