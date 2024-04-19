use std::io;

fn caesar_cipher(text: &str, shift: u8) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() {b'a'} else {b'A'};
                (((c as u8 + shift - base) % 26) + base) as char
            } else {
                c
            }
        })
        .collect()
}
fn substitution_cipher(text: &str, key: &str) -> String {
    let mut result = String::new();
    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() {b'a'} else {b'A'};
            let index = (c as u8 - base) as usize;
            result.push_str(&key[index..index + 1]);
        } else {
            result.push(c);
        }
    }
    result
}
fn rot13(text: &str) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() {b'a'} else {b'A'};
                (((c as u8 - base + 13) % 26) + base) as char
            } else {
                c
            }
        })
        .collect()
}
fn reverse(text: &str) -> String {
    text.chars().rev().collect()
}
fn ascii_cipher(text: &str) -> String {
    text.chars().map(|c| (c as u8).to_string()).collect::<Vec<String>>().join(" ")

}


fn main() {
    println!("Enter a text:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading the line!");
    let text = input.trim();

    println!("Choose an entcrypt-methode:");
    println!("1.Caesar-Encrypting:");
    println!("2. Substitution-Encrypting:");
    println!("3. ROT13-Encrypting:");
    println!("4. Reverse-Encrypting:");
    println!("5. ASCII-Encrypting:");

    let mut method_input = String::new();
    io::stdin().read_line(&mut shift_input).expect("Error reading the line!");
    let method = method_input.trim().parse::<u32>().expect("Invalid Input");

    let entcrypted_text = match method {
        1 => {
            println!("Inset the shift:");
            let mut shift_input = String::new();
            io::stdin().read_line(&mut shift_input).expect("Error reading the line!");
            let shift = shift_input.trim();
            caesar_cipher(text, shift)
        }
        2 => {
            println!("Insert a key for substitution: (26 chars without space)");
            let mut key_input.trim();
            let key = key_input.trim();
            substitution_cipher(text, key)
        }
        3 => rot13(text),
        4 => reverse(text),
        5 => ascii_cipher(text),
        _ => {
            println!("Invalid Input!")
            caesar_cipher(text, 3)
        }
    };
    println!("Encrypted text: {}", entcrypted_text);
}
