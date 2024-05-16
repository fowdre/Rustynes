mod bus;
mod devices;

#[derive(Debug)]
struct NES {
    cpu: devices::Cpu6502,
    bus: bus::Bus,
}

fn main() {
    let mut nes = NES {
        cpu: devices::Cpu6502::new(),
        bus: bus::Bus {
            ram: [0; 64 * 1024],
        },
    };

    println!("{:?}", nes);

    nes.cpu.write(&mut nes.bus, 0x0000, 0x0001);
    nes.cpu.write(&mut nes.bus, 0x0001, 0x0002);
    nes.cpu.write(&mut nes.bus, 0x0004, 0x0003);

    for i in 0..5 {
        let data = nes.cpu.read(&nes.bus, i);
        println!("Data byte [0x{:04X}] -> [0x{:02X}]", i, data);
    }
}
