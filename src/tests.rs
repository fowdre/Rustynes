#[cfg(test)]

use super::*;

#[test]
fn test_cpu_read_and_write() {
    let mut nes = Nes::new();

    assert_eq!(nes.cpu_read(0x0000), 0x00);
    assert_eq!(nes.cpu_read(0x0001), 0x00);
    assert_eq!(nes.cpu_read(0x0002), 0x00);
    assert_eq!(nes.cpu_read(0x0003), 0x00);
    assert_eq!(nes.cpu_read(0x0004), 0x00);

    nes.cpu_write(0x0000, 0x0001);
    nes.cpu_write(0x0001, 0x0002);
    nes.cpu_write(0x0004, 0x0003);

    assert_eq!(nes.cpu_read(0x0000), 0x01);
    assert_eq!(nes.cpu_read(0x0001), 0x02);
    assert_eq!(nes.cpu_read(0x0002), 0x00);
    assert_eq!(nes.cpu_read(0x0003), 0x00);
    assert_eq!(nes.cpu_read(0x0004), 0x03);
}
