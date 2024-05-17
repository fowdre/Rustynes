mod tests;

mod nes;
use nes::Nes;

fn main() {
    let mut nes = Nes::new();

    nes.cpu_tick();

    println!("{:?}", nes);
}
