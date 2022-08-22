/*fn char_to_binary(c: char) -> String {
  let hex_num = c.to_digit(16).expect("Not a valid hex character") as u8;

  let binary = format!("{hex_num:b}");

  // Add the missing left side zeros
  let binary_num = match 4 - binary.chars().count() {
      1 => format!("0{binary}"),
      2 => format!("00{binary}"),
      3 => format!("000{binary}"),
      _ => binary
  };

  binary_num
}

fn main() {
    let hex_str = b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    // let hex_str = "1aff";
    // let bytes = hex_str.as_bytes();

    let buffer: String = hex_str.iter().map(|c| char_to_binary(*c as char)).collect();

    println!("Buffer: {}", buffer);
    println!("Buffer size: {}", buffer.chars().count());
    println!("Buffer chars: {:?}", buffer.chars());

    let test = buffer.chars().take(6);
    let str_test = test.map(|c| c).collect::<String>();

    let test2 = String::from("ab");
    let chunk1 = test2.chars().take(2).map(|c| c).collect::<String>();
    println!("Chunk 1: {}", chunk1);

    let chunk2 = test2.chars().take(2).map(|c| c).collect::<String>();
    println!("Chunk 2: {}", chunk2);

    println!("Str_test: {}", str_test);
   // let test = test.
    // Elaborate on this...
    // while let mut iter = buffer.iter().take(6) {

    // }
    // let bytes = bytes[0];
    // let binary = format!("{bytes:b}");
    // println!("M as a binary: {}", binary);

    //let spaceless: String =
      //  spacey.chars().filter(|c| !c.is_whitespace()).collect();

    let c = 'a';
    let test = c.to_digit(16).expect("Not a valid hex character") as u8;

    let binary2 = format!("{test:b}");
    // Add the missing left side zeros
    let binary2 = match 4 - binary2.chars().count() {
        1 => format!("0{binary2}"),
        2 => format!("00{binary2}"),
        3 => format!("000{binary2}"),
        _ => binary2
    };
    println!("'{}' as a binary: {}", c, binary2);

    let intval = u8::from_str_radix(&binary2, 2).unwrap();
    println!("{} as int: {}", binary2, intval);

    let base64_c = intval as char;
    println!("\nC again: {:?}", base64_c);

  for i in (0..buffer.chars().count()-6).step_by(6) {
      let intval = u8::from_str_radix(&buffer[i..i+6], 2).unwrap();
      println!("{} as int: {}", &buffer[i..i+6], intval);
  }
}
*/

use base64::encode;

fn main() {
    let hex_str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let decoded = base16::decode(&hex_str).unwrap();
    println!("Byte vector: {:?}", decoded);

    let base64_encoded = encode(decoded);
    println!("Encoded vector: {:?}", base64_encoded);

    let base64_str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    assert_eq!(base64_encoded, base64_str);
}
