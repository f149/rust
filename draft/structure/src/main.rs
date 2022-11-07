struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
    
let user1 = User {
    email: String::from("user1@mail.com"),
    username: String::from("name_of_user1"),
    active: true,
    sign_in_count: 1,
}
