mod tests;

mod nes;
use nes::NES;

fn main() {
    let nes = NES::new();

    println!("{:?}", nes);
}
