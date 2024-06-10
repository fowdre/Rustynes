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

}
