struct User {
    #[allow(dead_code)]
    active: bool,
    #[allow(dead_code)]
    username: String,
    email: String,
    #[allow(dead_code)]
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;

pub fn main() {
    let mut _user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
    _user1.email = String::from("anotheremail@example.com");
    let _user2 = User {
        email: String::from("another@example.com"),
        .._user1
    };
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
    let _subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
