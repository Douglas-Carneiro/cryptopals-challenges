fn main() {
    // let hex_str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let hex_str = "551be2082b1563c4ec2247140400124d4b6508041b5a472256093aea1847";
    println!("Size of hex_str: {}", hex_str.len());
    let decoded = base16::decode(&hex_str).unwrap();

    // Bytes representing alphabetic ascii characters, this range includes some special characters
    let cypher_bytes = (65..=122).collect::<Vec<u8>>();

    let original_msg = 2;
    let xor_cypher = 3;
    let encoded_msg = original_msg ^ xor_cypher;

    assert_eq!((encoded_msg ^ xor_cypher), original_msg);
    assert_eq!((original_msg ^ encoded_msg), xor_cypher);

    let mut max_score = 0;
    let mut key = 0;
    let mut decrypted_msg = String::new();

    for cypher in cypher_bytes {
        let message = decoded.iter().map(|b| b ^ cypher).collect::<Vec<u8>>();
        let score = message.iter().filter(|&b| { let c = *b as char; c.is_ascii_alphabetic()}).count();

        if score > max_score {
            max_score = score;
            key = cypher;
            println!("Message: {:?}", message.iter().filter(|&b| { let c = *b as char; c.is_ascii_alphabetic()}).collect::<Vec<&u8>>());
            decrypted_msg = String::from_utf8(message).unwrap_or("Problem creating string".to_string());
            println!("Decrypted message: {}", decrypted_msg);
        }
    }

    println!("The best performing key is: {}", key as char);
    println!("The message obtained using this key is: {decrypted_msg}");
    println!("With score: {max_score}");

    let test_msg: Vec<u8> = vec![105, 85, 104, 107, 90, 72, 88, 80, 73, 75, 116, 88, 75, 65, 110, 81, 87, 115, 65, 68, 88, 116, 98, 120, 73, 75];
    let test_msg_decoded = String::from_utf8(test_msg).unwrap_or("Problem creating string".to_string());
    println!("Test message: {}", test_msg_decoded);
}
