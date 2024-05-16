mod tests;

mod nes;
use nes::NES;

fn main() {
    let mut nes = NES::new();

    nes.cpu_tick();

    println!("{:?}", nes);
}
