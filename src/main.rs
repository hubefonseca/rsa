extern crate rsa;

fn main() {
    let (a, b, c) = rsa::generate(128);

    println!("{} {} {}", a, b, c);
}
