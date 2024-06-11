#![allow(dead_code, unused_variables, non_snake_case)]

#![cfg(test)]

use serde::Deserialize;
use super::*;

#[derive(Debug, Deserialize)]
pub struct TestState {
    pub pc: u16,
    pub s: u8,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub p: u8,
    pub ram: Vec<(u16, u8)>,
}

#[derive(Debug, Deserialize)]
struct TestCycles {
    address: u16,
    value: u8,
    operation: String,
}

#[derive(Debug, Deserialize)]
struct TestEntry {
    name: String,
    initial: TestState,
    r#final: TestState,
    cycles: Vec<TestCycles>,
}

pub mod cycles_trace {
    use lazy_static::lazy_static;
    use std::sync::Mutex;

    lazy_static! {
        pub static ref CYCLES: Mutex<Vec<(u16, u8, String)>> = Mutex::new(Vec::new());
    }
}

// --------------------------------- [ADC] --------------------------------- //

#[test]
fn ADC_immediate() {
    run_json_test("./tests/69.json");
}

#[test]
fn ADC_zero_page() {
    run_json_test("./tests/65.json");
}

#[test]
fn ADC_zero_page_x() {
    run_json_test("./tests/75.json");
}

#[test]
fn ADC_absolute() {
    run_json_test("./tests/6d.json");
}

#[test]
fn ADC_absolute_x() {
    run_json_test("./tests/7d.json");
}

#[test]
fn ADC_absolute_y() {
    run_json_test("./tests/79.json");
}

#[test]
fn ADC_indirect_x() {
    run_json_test("./tests/61.json");
}

#[test]
fn ADC_indirect_y() {
    run_json_test("./tests/71.json");
}

// --------------------------------- [AND] --------------------------------- //

#[test]
fn AND_immediate() {
    run_json_test("./tests/29.json");
}

#[test]
fn AND_zero_page() {
    run_json_test("./tests/25.json");
}

#[test]
fn AND_zero_page_x() {
    run_json_test("./tests/35.json");
}

#[test]
fn AND_absolute() {
    run_json_test("./tests/2d.json");
}

#[test]
fn AND_absolute_x() {
    run_json_test("./tests/3d.json");
}

#[test]
fn AND_absolute_y() {
    run_json_test("./tests/39.json");
}

#[test]
fn AND_indirect_x() {
    run_json_test("./tests/21.json");
}

#[test]
fn AND_indirect_y() {
    run_json_test("./tests/31.json");
}

// --------------------------------- [ASL] --------------------------------- //

#[test]
fn ASL_accumulator() {
    run_json_test("./tests/0a.json");
}

#[test]
fn ASL_zero_page() {
    run_json_test("./tests/06.json");
}

#[test]
fn ASL_zero_page_x() {
    run_json_test("./tests/16.json");
}

#[test]
fn ASL_absolute() {
    run_json_test("./tests/0e.json");
}

#[test]
fn ASL_absolute_x() {
    run_json_test("./tests/1e.json");
}

// --------------------------------- [BCC] --------------------------------- //

#[test]
fn BCC_relative() {
    run_json_test("./tests/90.json");
}

// --------------------------------- [BEQ] --------------------------------- //

#[test]
fn BEQ_relative() {
    run_json_test("./tests/f0.json");
}

// --------------------------------- [BIT] --------------------------------- //

#[test]
fn BIT_zero_page() {
    run_json_test("./tests/24.json");
}

#[test]
fn BIT_absolute() {
    run_json_test("./tests/2c.json");
}

// --------------------------------- [BMI] --------------------------------- //

#[test]
fn BMI_relative() {
    run_json_test("./tests/30.json");
}

// --------------------------------- [BNE] --------------------------------- //

#[test]
fn BNE_relative() {
    run_json_test("./tests/d0.json");
}

// --------------------------------- [BPL] --------------------------------- //

#[test]
fn BPL_relative() {
    run_json_test("./tests/10.json");
}

// --------------------------------- [BRK] --------------------------------- //

#[test]
fn BRK_implied() {
    run_json_test("./tests/00.json");
}

// --------------------------------- [BVC] --------------------------------- //

#[test]
fn BVC_relative() {
    run_json_test("./tests/50.json");
}

// --------------------------------- [BVS] --------------------------------- //

#[test]
fn BVS_relative() {
    run_json_test("./tests/70.json");
}

// --------------------------------- [CLC] --------------------------------- //

#[test]
fn CLC_implied() {
    run_json_test("./tests/18.json");
}

// --------------------------------- [CLD] --------------------------------- //

#[test]
fn CLD_implied() {
    run_json_test("./tests/d8.json");
}

// --------------------------------- [CLI] --------------------------------- //

#[test]
fn CLI_implied() {
    run_json_test("./tests/58.json");
}

// --------------------------------- [CLV] --------------------------------- //

#[test]
fn CLV_implied() {
    run_json_test("./tests/b8.json");
}

// --------------------------------- [CMP] --------------------------------- //

#[test]
fn CMP_immediate() {
    run_json_test("./tests/c9.json");
}

#[test]
fn CMP_zero_page() {
    run_json_test("./tests/c5.json");
}

#[test]
fn CMP_zero_page_x() {
    run_json_test("./tests/d5.json");
}

#[test]
fn CMP_absolute() {
    run_json_test("./tests/cd.json");
}

#[test]
fn CMP_absolute_x() {
    run_json_test("./tests/dd.json");
}

#[test]
fn CMP_absolute_y() {
    run_json_test("./tests/d9.json");
}

#[test]
fn CMP_indirect_x() {
    run_json_test("./tests/c1.json");
}

#[test]
fn CMP_indirect_y() {
    run_json_test("./tests/d1.json");
}

// --------------------------------- [CPX] --------------------------------- //

#[test]
fn CPX_immediate() {
    run_json_test("./tests/e0.json");
}

#[test]
fn CPX_zero_page() {
    run_json_test("./tests/e4.json");
}

#[test]
fn CPX_absolute() {
    run_json_test("./tests/ec.json");
}

// --------------------------------- [CPY] --------------------------------- //

#[test]
fn CPY_immediate() {
    run_json_test("./tests/c0.json");
}

#[test]
fn CPY_zero_page() {
    run_json_test("./tests/c4.json");
}

#[test]
fn CPY_absolute() {
    run_json_test("./tests/cc.json");
}

// --------------------------------- [DEC] --------------------------------- //

#[test]
fn DEC_zero_page() {
    run_json_test("./tests/c6.json");
}

#[test]
fn DEC_zero_page_x() {
    run_json_test("./tests/d6.json");
}

#[test]
fn DEC_absolute() {
    run_json_test("./tests/ce.json");
}

#[test]
fn DEC_absolute_x() {
    run_json_test("./tests/de.json");
}

// --------------------------------- [DEX] --------------------------------- //

#[test]
fn DEX_implied() {
    run_json_test("./tests/ca.json");
}

// --------------------------------- [DEY] --------------------------------- //

#[test]
fn DEY_implied() {
    run_json_test("./tests/88.json");
}

// --------------------------------- [EOR] --------------------------------- //

#[test]
fn EOR_immediate() {
    run_json_test("./tests/49.json");
}

#[test]
fn EOR_zero_page() {
    run_json_test("./tests/45.json");
}

#[test]
fn EOR_zero_page_x() {
    run_json_test("./tests/55.json");
}

#[test]
fn EOR_absolute() {
    run_json_test("./tests/4d.json");
}

#[test]
fn EOR_absolute_x() {
    run_json_test("./tests/5d.json");
}

#[test]
fn EOR_absolute_y() {
    run_json_test("./tests/59.json");
}

#[test]
fn EOR_indirect_x() {
    run_json_test("./tests/41.json");
}

#[test]
fn EOR_indirect_y() {
    run_json_test("./tests/51.json");
}

// --------------------------------- [INC] --------------------------------- //

#[test]
fn INC_zero_page() {
    run_json_test("./tests/e6.json");
}

#[test]
fn INC_zero_page_x() {
    run_json_test("./tests/f6.json");
}

#[test]
fn INC_absolute() {
    run_json_test("./tests/ee.json");
}

#[test]
fn INC_absolute_x() {
    run_json_test("./tests/fe.json");
}

// --------------------------------- [INX] --------------------------------- //

#[test]
fn INX_implied() {
    run_json_test("./tests/e8.json");
}

// --------------------------------- [INY] --------------------------------- //

#[test]
fn INY_implied() {
    run_json_test("./tests/c8.json");
}

// --------------------------------- [JMP] --------------------------------- //

#[test]
fn JMP_absolute() {
    run_json_test("./tests/4c.json");
}

#[test]
fn JMP_indirect() {
    run_json_test("./tests/6c.json");
}

// --------------------------------- [JSR] --------------------------------- //

#[test]
fn JSR_absolute() {
    run_json_test("./tests/20.json");
}

// --------------------------------- [LDA] --------------------------------- //

#[test]
fn LDA_immediate() {
    run_json_test("./tests/a9.json");
}

#[test]
fn LDA_zero_page() {
    run_json_test("./tests/a5.json");
}

#[test]
fn LDA_zero_page_x() {
    run_json_test("./tests/b5.json");
}

#[test]
fn LDA_absolute() {
    run_json_test("./tests/ad.json");
}

#[test]
fn LDA_absolute_x() {
    run_json_test("./tests/bd.json");
}

#[test]
fn LDA_absolute_y() {
    run_json_test("./tests/b9.json");
}

#[test]
fn LDA_indirect_x() {
    run_json_test("./tests/a1.json");
}

#[test]
fn LDA_indirect_y() {
    run_json_test("./tests/b1.json");
}

// --------------------------------- [LDX] --------------------------------- //

#[test]
fn LDX_immediate() {
    run_json_test("./tests/a2.json");
}

#[test]
fn LDX_zero_page() {
    run_json_test("./tests/a6.json");
}

#[test]
fn LDX_zero_page_y() {
    run_json_test("./tests/b6.json");
}

#[test]
fn LDX_absolute() {
    run_json_test("./tests/ae.json");
}

#[test]
fn LDX_absolute_y() {
    run_json_test("./tests/be.json");
}

// --------------------------------- [LDY] --------------------------------- //

#[test]
fn LDY_immediate() {
    run_json_test("./tests/a0.json");
}

#[test]
fn LDY_zero_page() {
    run_json_test("./tests/a4.json");
}

#[test]
fn LDY_zero_page_x() {
    run_json_test("./tests/b4.json");
}

#[test]
fn LDY_absolute() {
    run_json_test("./tests/ac.json");
}

#[test]
fn LDY_absolute_x() {
    run_json_test("./tests/bc.json");
}

// --------------------------------- [LSR] --------------------------------- //

#[test]
fn LSR_accumulator() {
    run_json_test("./tests/4a.json");
}

#[test]
fn LSR_zero_page() {
    run_json_test("./tests/46.json");
}

#[test]
fn LSR_zero_page_x() {
    run_json_test("./tests/56.json");
}

#[test]
fn LSR_absolutetmp() {
    run_json_test("./tests/4e.json");
}

#[test]
fn LSR_absolute_x() {
    run_json_test("./tests/5e.json");
}

// --------------------------------- [NOP] --------------------------------- //

#[test]
fn NOP_implied() {
    run_json_test("./tests/ea.json");
}

// --------------------------------- [ORA] --------------------------------- //

#[test]
fn ORA_immediate() {
    run_json_test("./tests/09.json");
}

#[test]
fn ORA_zero_page() {
    run_json_test("./tests/05.json");
}

#[test]
fn ORA_zero_page_x() {
    run_json_test("./tests/15.json");
}

#[test]
fn ORA_absolute() {
    run_json_test("./tests/0d.json");
}

#[test]
fn ORA_absolute_x() {
    run_json_test("./tests/1d.json");
}

#[test]
fn ORA_absolute_y() {
    run_json_test("./tests/19.json");
}

#[test]
fn ORA_indirect_x() {
    run_json_test("./tests/01.json");
}

#[test]
fn ORA_indirect_y() {
    run_json_test("./tests/11.json");
}

// --------------------------------- [PHA] --------------------------------- //

#[test]
fn PHA_implied() {
    run_json_test("./tests/48.json");
}

// --------------------------------- [PHP] --------------------------------- //

#[test]
fn PHP_implied() {
    run_json_test("./tests/08.json");
}

// --------------------------------- [PLA] --------------------------------- //

#[test]
fn PLA_implied() {
    run_json_test("./tests/68.json");
}

// --------------------------------- [PLP] --------------------------------- //

#[test]
fn PLP_implied() {
    run_json_test("./tests/28.json");
}

// --------------------------------- [ROL] --------------------------------- //

#[test]
fn ROL_accumulator() {
    run_json_test("./tests/2a.json");
}

#[test]
fn ROL_zero_page() {
    run_json_test("./tests/26.json");
}

#[test]
fn ROL_zero_page_x() {
    run_json_test("./tests/36.json");
}

#[test]
fn ROL_absolute() {
    run_json_test("./tests/2e.json");
}

// --------------------------------- [ROR] --------------------------------- //

#[test]
fn ROR_accumulator() {
    run_json_test("./tests/6a.json");
}

#[test]
fn ROR_zero_page() {
    run_json_test("./tests/66.json");
}

#[test]
fn ROR_zero_page_x() {
    run_json_test("./tests/76.json");
}

#[test]
fn ROR_absolute() {
    run_json_test("./tests/6e.json");
}

#[test]
fn ROR_absolute_x() {
    run_json_test("./tests/7e.json");
}

// --------------------------------- [RTI] --------------------------------- //

#[test]
fn RTI_implied() {
    run_json_test("./tests/40.json");
}

// --------------------------------- [RTS] --------------------------------- //

#[test]
fn RTS_implied() {
    run_json_test("./tests/60.json");
}

// --------------------------------- [SBC] --------------------------------- //

#[test]
fn SBC_immediate() {
    run_json_test("./tests/e9.json");
}

#[test]
fn SBC_zero_page() {
    run_json_test("./tests/e5.json");
}

#[test]
fn SBC_zero_page_x() {
    run_json_test("./tests/f5.json");
}

#[test]
fn SBC_absolute() {
    run_json_test("./tests/ed.json");
}

#[test]
fn SBC_absolute_x() {
    run_json_test("./tests/fd.json");
}

#[test]
fn SBC_absolute_y() {
    run_json_test("./tests/f9.json");
}

#[test]
fn SBC_indirect_x() {
    run_json_test("./tests/e1.json");
}

#[test]
fn SBC_indirect_y() {
    run_json_test("./tests/f1.json");
}

// --------------------------------- [SEC] --------------------------------- //

#[test]
fn SEC_implied() {
    run_json_test("./tests/38.json");
}

// --------------------------------- [SED] --------------------------------- //

#[test]
fn SED_implied() {
    run_json_test("./tests/f8.json");
}

// --------------------------------- [SEI] --------------------------------- //

#[test]
fn SEI_implied() {
    run_json_test("./tests/78.json");
}

// --------------------------------- [STA] --------------------------------- //

#[test]
fn STA_zero_page() {
    run_json_test("./tests/85.json");
}

#[test]
fn STA_zero_page_x() {
    run_json_test("./tests/95.json");
}

#[test]
fn STA_absolute() {
    run_json_test("./tests/8d.json");
}

#[test]
fn STA_absolute_x() {
    run_json_test("./tests/9d.json");
}

#[test]
fn STA_absolute_y() {
    run_json_test("./tests/99.json");
}

#[test]
fn STA_indirect_x() {
    run_json_test("./tests/81.json");
}

#[test]
fn STA_indirect_y() {
    run_json_test("./tests/91.json");
}

// --------------------------------- [STX] --------------------------------- //

#[test]
fn STX_zero_page() {
    run_json_test("./tests/86.json");
}

#[test]
fn STX_zero_page_y() {
    run_json_test("./tests/96.json");
}

#[test]
fn STX_absolute() {
    run_json_test("./tests/8e.json");
}

// --------------------------------- [STY] --------------------------------- //

#[test]
fn STY_zero_page() {
    run_json_test("./tests/84.json");
}

#[test]
fn STY_zero_page_x() {
    run_json_test("./tests/94.json");
}

#[test]
fn STY_absolute() {
    run_json_test("./tests/8c.json");
}

// --------------------------------- [TAX] --------------------------------- //

#[test]
fn TAX_implied() {
    run_json_test("./tests/aa.json");
}

// --------------------------------- [TAY] --------------------------------- //

#[test]
fn TAY_implied() {
    run_json_test("./tests/a8.json");
}

// --------------------------------- [TSX] --------------------------------- //

#[test]
fn TSX_implied() {
    run_json_test("./tests/ba.json");
}

// --------------------------------- [TXA] --------------------------------- //

#[test]
fn TXA_implied() {
    run_json_test("./tests/8a.json");
}

// --------------------------------- [TXS] --------------------------------- //

#[test]
fn TXS_implied() {
    run_json_test("./tests/9a.json");
}

// --------------------------------- [TYA] --------------------------------- //

#[test]
fn TYA_implied() {
    run_json_test("./tests/98.json");
}

fn run_json_test(path: &str) {
    const CYCLE_LIMIT : usize = 10000;
    // const CYCLE_LIMIT : usize = 5;
    let file_contents = std::fs::read_to_string(path).expect("Failed to read file");
    let deserialized: Vec<TestEntry> = serde_json::from_str(&file_contents).unwrap();

    let mut test_passed = Vec::new();
    let mut nes = Nes::new();
    
    for entry in deserialized.iter() {
        let mut local_cycle = 0_u64;
        nes.test_reset();
        cycles_trace::CYCLES.lock().unwrap().clear();
        
        println!("\n[Executing test: {}]", entry.name);
        
        nes.test_set_initial_state(&entry.initial);

        while cycles_trace::CYCLES.lock().unwrap().len() != entry.cycles.len() {
            nes.test_tick();
            local_cycle += 1;
            if local_cycle > CYCLE_LIMIT as u64 {
                println!("\n[ERROR] End state should have been:\n");
                println!("pc [{:4X}] | sp [{:2X}] | a [{:2X}] | x [{:2X}] | y [{:2X}] | status [{:2X}]", entry.r#final.pc, entry.r#final.s, entry.r#final.a, entry.r#final.x, entry.r#final.y, entry.r#final.p);
                for cycle in entry.cycles.iter() {
                    println!("[{}, {}, \"{}\"]", cycle.address, cycle.value, cycle.operation);
                }
                println!("\n[ERROR] But it was:\n");
                println!("pc [{:4X}] | sp [{:2X}] | a [{:2X}] | x [{:2X}] | y [{:2X}] | status [{:2X}]", nes.get_cpu_info().program_counter, nes.get_cpu_info().stack_pointer, nes.get_cpu_info().reg_a, nes.get_cpu_info().reg_x, nes.get_cpu_info().reg_y, nes.get_cpu_flags());
                for cycle in cycles_trace::CYCLES.lock().unwrap().iter() {
                    println!("[{}, {}, \"{}\"]", cycle.0, cycle.1, cycle.2);
                }
                panic!("Failed to execute test: {} - cycle limit reached\n", entry.name);
            }
        }

        println!("\nExpected cycles");
        for cycle in entry.cycles.iter() {
            println!("[{}, {}, \"{}\"]", cycle.address, cycle.value, cycle.operation);
        }
        println!("\nMy cycles");
        for cycle in cycles_trace::CYCLES.lock().unwrap().iter() {
            println!("[{}, {}, \"{}\"]", cycle.0, cycle.1, cycle.2);
        }

        if !nes.test_end_state(&entry.r#final) {
            println!("\n[ERROR] End state should have been:\n");
            println!("pc [{:4X}] | sp [{:2X}] | a [{:2X}] | x [{:2X}] | y [{:2X}] | status [{:2X}]", entry.r#final.pc, entry.r#final.s, entry.r#final.a, entry.r#final.x, entry.r#final.y, entry.r#final.p);
            for cycle in entry.cycles.iter() {
                println!("[{}, {}, \"{}\"]", cycle.address, cycle.value, cycle.operation);
            }
            println!("\n[ERROR] But it was:\n");
            println!("pc [{:4X}] | sp [{:2X}] | a [{:2X}] | x [{:2X}] | y [{:2X}] | status [{:2X}]", nes.get_cpu_info().program_counter, nes.get_cpu_info().stack_pointer, nes.get_cpu_info().reg_a, nes.get_cpu_info().reg_x, nes.get_cpu_info().reg_y, nes.get_cpu_flags());
            for cycle in cycles_trace::CYCLES.lock().unwrap().iter() {
                println!("[{}, {}, \"{}\"]", cycle.0, cycle.1, cycle.2);
            }
            panic!("Failed to execute test: {}\n", entry.name);
        }

        test_passed.push(&entry.name);
    }

    println!("\n\nTests passed:");
    for test in test_passed.iter() {
        println!("{}", test);
    }

    assert_eq!(test_passed.len(), deserialized.len());
}
