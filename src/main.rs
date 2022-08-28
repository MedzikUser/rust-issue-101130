use crypto::decode;
use macros::encode;

fn main() {
    // encode text in compile time
    let text = encode!("Hello, World!");

    // decode and print the text in runtime
    println!("{:?}", decode(text));
    println!("Key: {:?}", crypto::KEY);
}
