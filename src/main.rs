mod nes;
use nes::NES;

fn main() {
    let mut nes = NES::new();

    println!("{:?}", nes);

    nes.cpu_write(0x0000, 0x0001);
    nes.cpu_write(0x0001, 0x0002);
    nes.cpu_write(0x0004, 0x0003);

    for i in 0..5 {
        let data = nes.cpu_read(i);
        println!("Data byte [0x{:04X}] -> [0x{:02X}]", i, data);
    }
}
