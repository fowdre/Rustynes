#![allow(unused)]

mod bus;
mod devices;

fn main() {
    let cpu = devices::Cpu6502::new();
    dbg!(&cpu);
    let bus = bus::Bus{ cpu: &cpu, ram: [0x0000; 64 * 1024] };
}
