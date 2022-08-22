fn main() {
    let hex_str_1 = "1c0111001f010100061a024b53535009181c";
    let hex_str_2 = "686974207468652062756c6c277320657965";

    let decoded_1 = base16::decode(hex_str_1).unwrap();
    let decoded_2 = base16::decode(hex_str_2).unwrap();

    let xor: Vec<u8> = decoded_1.iter().zip(decoded_2.iter()).map(|(x, y)| x ^ y).collect();
    println!("XOR result: {:?}\n", xor);

    let result = base16::encode_config(&xor, base16::EncodeLower);
    println!("Encoded result: {}", result);

    assert_eq!(result, "746865206b696420646f6e277420706c6179");
}

// https://linuxhint.com/update_google_chrome_ubuntu/