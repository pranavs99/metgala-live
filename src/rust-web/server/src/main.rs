// struct Account {
//     username: String,
//     password: String
// }

// enum IpAddrKind {
//     V4,
//     V6
// }

// struct IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// fn okay(x: i32) -> i32 {
//     x + 1
// }

// fn first_word(s: &String) -> &str {
//     // move byte form of string -> bytes
//     let bytes = s.as_bytes();
//     // iterate through bytes
//     for (i, &item) in bytes.iter().enumerate() {
//         // upon seeing space, return slice of string until space
//         if item == b' ' {
//             return &s[..i];
//         }
//     }
//     // return string reference if nothing found
//     s
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter
// }

// fn getCoinValue(c: Coin) -> u8 {
//     match c {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

// struct Account {
//     username: String,
//     password: String,
// }

// impl Account {

// }

use std::collections::HashMap;

struct Account {
    username: String,
    displayName: String,
    password: String,
}

impl Account {
    // print username & display name of account
    fn printAccount(&self) {
        println!("username: {}", self.username);
        println!("display name: {}", self.displayName);
    }
    // change password field of Account struct
    fn changePassword(&mut self, newPassword: String) {
        self.password = newPassword;
    }
}



fn main() {
    let mut accounts: HashMap<String, Account> = HashMap::new();

    let mut a: Account = Account {
        username: String::from("admin"),
        displayName: String::from("Admin Istrator"),
        password: String::from("admin"),
    };
    accounts.insert(a.username, a);

    let mut p: Account = Account {
        username: String::from("pranav"),
        displayName: String::from("Pranav Singh"),
        password: String::from("test"),
    };
    accounts.insert(p.username, p);

    a.printAccount();
}