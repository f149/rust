fn main() {
    println!("First Function!");
    another_function();
    get_numb_square(11);
    get_numb_summ(17, 5);
    block_exmpl();
    println!("Func Five = {}", five());
    println!("Func PlusOne = {}", plus_one(221));
}

fn another_function() {
    println!("Second Function!");
}

fn get_numb_square(x: i32) {
    let result = x*x;
    println!("Square = {}", result);
}

fn get_numb_summ(x: i32, y: i32){
    let result = x + y;
    println!("Summ = {}", result);
}

fn block_exmpl() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };
    println!("Value from block is {}", y);
}

fn five () -> i32 {
    22
}

fn plus_one(x: i32) -> i32{
    x + 1
}
