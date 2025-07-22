// This is the src file for challenge 1, the creds and flag are placeholders
use std::io::{Write, stdin, stdout};

fn main() {
    // static username
    let username = "Test123";

    // static password
    let password = "Test123";

    // gather user input for username
    let mut input_username = String::new();
    print!("Please enter your username: ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut input_username)
        .expect("Did not enter string");

    // gather user input for password
    let mut input_password = String::new();
    print!("Please enter your password: ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut input_password)
        .expect("Did not enter string");

    // checking if username and password match (trim input)
    if input_username.trim() == username && input_password.trim() == password {
        println!("You did it well done!");
        println!("heres your flag >> FLAG{{Place_holder}");
    } else {
        println!("Invalid username or password.");
    }
}
