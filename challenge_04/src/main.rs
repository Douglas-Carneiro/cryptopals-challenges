use std::env;
use std::fs;

fn decrypt_message(filename: &str) {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    
    // Bytes representing alphabetic ascii characters, this range includes some special characters
    let cypher_bytes = (65..=122).collect::<Vec<u8>>();

    let mut max_score = 0;
    let mut key = 0;
    let mut decrypted_msg = String::new();
    let mut line_number = 0;

    for (i, line) in contents.lines().enumerate() {
        // let line: String = line.parse().unwrap();
        if line.len() == 60 {
            println!("Checking line {} with content: '{}' and size: {}", i+1, line, line.len());
            let line = line.trim();
            let decoded = base16::decode(line).unwrap();

            for cypher in cypher_bytes.iter() {
                let message = decoded.iter().map(|b| b ^ cypher).collect::<Vec<u8>>();
                let score = message.iter().filter(|&b| { let c = *b as char; c.is_ascii_alphabetic()}).count();
        
                if score > max_score {
                    line_number = i+1;
                    max_score = score;
                    key = *cypher;
                    decrypted_msg = String::from_utf8(message).unwrap_or("Problem creating string".to_string());
                    println!("Message: {}", decrypted_msg);
                }
            }
        }
    }

    println!("The best performing key is: {}, with score: {}", key as char, max_score);
    println!("The message obtained using this key is: {decrypted_msg}");
    println!("The message is on the line {} of the file", line_number);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    decrypt_message(filename);

    println!("> is alphabetic: {},  > as a number: {}", '>'.is_ascii_alphabetic(), '>' as u8);
    let test = "iUhkZHXPIKtXKAnQWsD>XtbxIK";
    let test_score = test.chars().filter(|c| c.is_ascii_alphabetic()).count();
    println!("test score: {}", test_score);
}
