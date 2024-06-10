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

fn run_json_test(path: &str) {
    const CYCLE_LIMIT : usize = 10000;
    // const CYCLE_LIMIT : usize = 1;
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
