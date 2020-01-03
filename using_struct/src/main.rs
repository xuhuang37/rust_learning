fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    
    let mut user1 = User {
        username: String::from("alan"),
        email: String::from("someone@email.com"),
        sign_in_count: 1,
        active: true,
    };
    user1.email = String::from("anotherone@email.com");
    user1.username = String::from("anotherone@email.com");
    user1.active = false;
    user1.sign_in_count = 2;

    fn build_user(email: String, username: String) -> User {
        User {
            // username: username,
            // email: email,
            username,
            email,
            sign_in_count: 1,
            active: true,
        }
    }

    let user2 = User {
        username:String::from("sally"),
        email:String::from("anotherone@email.com"),
        ..user1
    };

    // struct User3 {
    //     username: &str,
    //     email: &str,
    //     sign_in_count: u64,
    //     active: bool, 
    // };

    // let user_lifetime = User3 {
    //     email: "someone@example.com",
    //     username: "someusername123",
    //     active: true,
    //     sign_in_count: 1,
    // };

}
