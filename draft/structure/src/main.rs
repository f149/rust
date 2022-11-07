fn main() {
// NOT mutable struct
    let user1 = User {
        email: String::from("user1@mail.com"),
        username: String::from("name_of_user1"),
        active: true,
        sign_in_count: 1,
    };
    
//MUTABLE STRUCT
    let mut user2 = User {
        email: String::from("user2@mail.com"),
        username: String::from("name_of_user2"),
        active: true,
        sign_in_count: 1,
    };
    
    user2.email = String::from("another_email@mail.com");
    
    let user3 = User {
        email: String::from("user3@mail.com"),
        username: String::from("user3_name"),
        active: true,
        sign_in_count: 1,
    };
    
    let user4 = User {
        email: String::from("user4@email.com"),
        username: String::from("user4_name"),
        ..user3
    };
}


fn create_user_v1(email: String, username: String) -> User {
    User{
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
        }
}

fn create_user_v2(email: String, username: String) -> User {
    User{
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}


















