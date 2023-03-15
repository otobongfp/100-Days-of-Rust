
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u32,
}

fn main() {
    // --snip--
    let user1 = User {
        active: true,
        username: String::from("Bolt"),
        email: String::from("another@example.com"),
        sign_in_count: 45,
    };

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}