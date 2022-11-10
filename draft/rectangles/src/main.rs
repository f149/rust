fn main() {
    let width1  = 30;
    let height1 = 50;

    let rect1 = (22, 101);
    let rect2 = Rectangle {width: 111, height: 2};
    let rect3 = Rectangle {width: 30, height: 50};
    let rect4 = Rectangle {width: 10, height: 40};
    let rect5 = Rectangle {width: 60, height: 45};

    println!("The area_V1 is equal to {} square pixels.", area_v1(width1, height1));
    println!("The area_V2 is equal to {} square pixels.", area_v2(rect1));
    println!("The area_V3 is equal to {} square pixels.", area_v3(&rect2));
    println!("Rectangle_2 with debug is eval {:?}", rect2);
    println!("The area_method is equal to {} square pixels.", rect2.area());
    println!("Can rect3 include rect4? {}", rect3.can_hold(&rect4));
    println!("Can rect3 include rect5? {}", rect3.can_hold(&rect5));

}               

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn area_v1(width: u32, height: u32) -> u32 {
    width * height
}

fn area_v2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_v3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
