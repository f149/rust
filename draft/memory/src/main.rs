fn main() {
    let mut s = String::from("Hello")   ;
    println!("String contain: {}", s);
    println!("Let s.push_str...");
    
    s.push_str(", CCXXII!");
    println!("Now string contain: {}", s);

    let s2 = String::from("IM S2!");
    let s3 = s2;

    println!("{}, TEST! S2", s3);

    let main_str = String::from("THIS IS MAIN STRING");
    let cloned_str = main_str.clone();

    println!("Main string = {}", main_str);
    println!("Cloned string = {}", cloned_str);

    let ss1 = gives_ownership();
    let ss2 = String::from("MMCCXXII");
    let ss3 = takes_and_gives_back(ss2);


}

fn gives_ownership() -> String {
    let some_string = String::from("CCXXII");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn let_calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
