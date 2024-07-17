// Creating the User struct template
struct User {
    active: bool,
    email: String,
    username: String,
    signin_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main () {
    // this instatiates the User struct into the new_user_1 variable
    let new_user_1 = User {
        active: true,
        email: String::from("test@email.com"),
        username: String::from("Test123"),
        signin_count: 1,
    };
    struct User {
        active: bool,
        email: String,
        username: String,
        signin_count: u64,
    }

    let new_user_2 = User {
        username: String::from("Test321"),
        ..new_user_1
    };

    // println!("{}", new_user_1.email);   // this causes an error
                                        //  because the email field is moved from new_user_1 to new_user_2

}

