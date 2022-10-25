fn main() {
    let mut x = 5;
    println!("Value x = {}", x);
    x = 6;
    println!("Value x = {}", x);


    let tup2 = (22, 222, 2222);
    let (_x, y, _z) = tup2;
    println!("Value Y from tup2 = {}", y);


    let a = [2, 22, 222, 2222, 22222];
    let b: [i32; 3] = [2, 22, 222];
    let c = [3; 5];

    let index = 2;
    let element = b[index];
    
    println!("Element #{} = {}", index, element);

    another_function();
 }

fn another_function() {
    println!("AnotherFunction");
}
