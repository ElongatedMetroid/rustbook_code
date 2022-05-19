struct User {
    username: String,
    email: String,
    sign_in_count: usize,
    active: bool,
}

fn main() {
    // if you would like change values of a struct member the whole struct has to be mutable, single elements cannot be mutable
    let mut user1 = User {
        username: String::from("Demarcus Baldwin"),
        email: String::from("demarcus.baldwin@nonamesoftware.com"),
        sign_in_count: 1,
        active: true,
    };

    user1.active = false;

    // to acess members of a structure you can user dot-notation
    println!("username {}, email {}, sign in count {}, active {}", user1.username, user1.email, user1.sign_in_count, user1.active);

    let user2 = build_user(String::from("Bob tims"), String::from("BobTims@nonamesoftware.com"));

    println!("username: {}", user2.username);

    // sometimes you will want to create a structure that creates a new instance of a struct but uses most of a different instances values
    // instead of doing this the long way by typing "active: user2.active" you can just do "..user2", shown below
    let user3 = User {
        username: String::from("Barbra Johnston"),
        email: String::from("qwerty@nonamesoftware.com"),
        // this
        ..user2
        // instead of
        // active: user1.active,
        // sign_in_count: user1.sign_in_count,
    };

    println!("username: {}, email: {}, active: {}, sign_in_count: {}", user3.username, user3.email, user3.active, user3.sign_in_count);

    // tuple structures
    // even though these both made up of the same i32 variables they are still DIFFERENT TYPES,
    // a function that takes in Color CANNOT take in point as an argument
    struct Color(i32, i32, i32);
    struct Orgin(i32, i32, i32);

    let black = Color(0, 0, 0);
    let orgin = Orgin(0, 0, 0);

    println!("Black = {}, {}, {}", black.0, black.1, black.2);
    println!("Orgin = {}, {}, {}", orgin.0, orgin.1, orgin.2);
}

fn build_user(username: String, email:String) -> User {
    User {
        // when you have variables with the same names as struct members you can use shorthand notation
        email, 
        username,
        // instead of
        // email: email,
        // username: username,
        active: true,
        sign_in_count: 5,
    }
}