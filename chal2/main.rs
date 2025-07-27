use std::io::{Write, stdin, stdout};

// raw byte arrays—no ASCII string literals here!
const U_BYTES: [u8; 14] = [
    // placeholder for: "username"
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
    0x00, 0x00,
];

const P_BYTES: [u8; 17] = [
    // placeholder for: "password"
    0x00, 0x00, 0x00, 0x00, 0x00, 
    0x00, 0x00, 
    0x00, 0x00, 0x00, 0x00, 
    0x00, 0x00, 0x00, 0x00, 
    0x00, 0x00,
];

const FLAG_XOR: [u8; 20] = [
    // placeholder for: XOR-encoded FLAG{example}
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
];

const XOR_KEY: u8 = 0x23; // example key, can be changed

fn decode(bytes: &[u8]) -> String {
    String::from_utf8(bytes.to_vec()).unwrap()
}

fn decode_flag(enc: &[u8], key: u8) -> String {
    let decoded: Vec<u8> = enc.iter().map(|b| b ^ key).collect();
    String::from_utf8(decoded).unwrap()
}

fn verify(u: &str, p: &str) -> bool {
    let expected_u = decode(&U_BYTES);
    let expected_p = decode(&P_BYTES);
    u == expected_u && p == expected_p
}

fn main() {
    let mut user_input_username = String::new();
    print!("Enter username: ");
    let _ = stdout().flush();
    stdin().read_line(&mut user_input_username).unwrap();

    let mut user_input_password = String::new();
    print!("Enter password: ");
    let _ = stdout().flush();
    stdin().read_line(&mut user_input_password).unwrap();

    if verify(user_input_username.trim(), user_input_password.trim()) {
        let flag = decode_flag(&FLAG_XOR, XOR_KEY);
        println!("Well done! Here's your flag => {}", flag);
    } else {
        println!("Access Den

