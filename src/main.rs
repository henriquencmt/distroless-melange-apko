use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let number: u8 = rng.gen();
    println!("{}", number);
}
