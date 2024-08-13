#![allow(non_snake_case)]
use coral::mos::primitive::set_decimal_enabled;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::File;
use std::io::{self, Read};

#[derive(Serialize, Deserialize)]
struct TomHarte {
    pc: u16,
    s: u8,
    a: u8,
    x: u8,
    y: u8,
    p: u8,
    ram: Vec<(u16, u8)>,
}

impl fmt::Debug for TomHarte {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TomHarte:")?;
        write!(f, "\n\tPC: {:04X}", self.pc)?;
        write!(f, "\n\tS: {:04X}", self.s)?;
        write!(f, "\n\tA: {:04X}", self.a)?;
        write!(f, "\n\tX: {:04X}", self.x)?;
        write!(f, "\n\tY: {:04X}", self.y)?;
        write!(f, "\n\tPS: {:08b}", self.p)?;
        write!(f, "\n\t")?;
        for (a, b) in &self.ram {
            write!(f, "({:04X}, {:02X})", a, b)?;
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Cycle {
    address: u16,
    byte: u8,
    action: String,
}

#[derive(Serialize, Deserialize)]
struct Cycles(Vec<Cycle>);

impl fmt::Debug for Cycles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for cycle in &self.0 {
            write!(
                f,
                "\n{} address {:04X} ({}) and got {:02X} ({})",
                cycle.action, cycle.address, cycle.address, cycle.byte, cycle.byte
            )?;
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Test {
    name: String,
    #[serde(rename(deserialize = "initial"))]
    initial_state: TomHarte,
    #[serde(rename(deserialize = "final"))]
    final_state: TomHarte,
    cycles: Cycles,
}

#[derive(Serialize, Deserialize, Debug)]
struct Tests(Vec<Test>);

struct SimpleBus {
    cpu: coral::mos::Mos,
    ram: [u8; 0x10000],
    cycles: Cycles,
}

impl coral::mos::Bus for SimpleBus {
    fn read_byte(&mut self, address: u16) -> u8 {
        let byte = self.ram[address as usize];
        let action = "read".to_string();
        self.cycles.0.push(Cycle {
            address,
            byte,
            action,
        });
        return byte;
    }
    fn write_byte(&mut self, address: u16, byte: u8) {
        let action = "write".to_string();
        self.cycles.0.push(Cycle {
            address,
            byte,
            action,
        });
        self.ram[address as usize] = byte;
    }
    fn fetch_mos(&mut self) -> &mut coral::mos::Mos {
        return &mut self.cpu;
    }
}
fn ram_from_tom(tom: &TomHarte) -> [u8; 0x10000] {
    let thram = &tom.ram;
    let mut ram = [0; 0x10000];
    for (addr, byte) in thram {
        ram[*addr as usize] = *byte;
    }
    return ram;
}

fn ram_to_tom(tom: &mut TomHarte, ram: [u8; 0x10000]) {
    for addr in 0..0x10000 {
        let byte = ram[addr];
        if byte != 0 {
            tom.ram.push((addr as u16, byte));
        }
    }
}

fn mos_from_tom(tom: &TomHarte) -> coral::mos::Mos {
    let mut cpu = coral::mos::new();
    cpu.registers.pc = tom.pc;
    cpu.registers.sp = tom.s;
    cpu.registers.acc = tom.a;
    cpu.registers.idx = tom.x;
    cpu.registers.idy = tom.y;
    cpu.registers.ps = tom.p;
    return cpu;
}

fn to_tom(simple: &SimpleBus) -> TomHarte {
    let r: Vec<(u16, u8)> = vec![];
    let cpu = simple.cpu;
    let mut tom = TomHarte {
        pc: cpu.registers.pc,
        s: cpu.registers.sp,
        a: cpu.registers.acc,
        x: cpu.registers.idx,
        y: cpu.registers.idy,
        p: cpu.registers.ps,
        ram: r,
    };
    ram_to_tom(&mut tom, simple.ram);
    return tom;
}

fn compare_memory(tom: &TomHarte, simple: &SimpleBus) -> bool {
    let sram = simple.ram;
    let tram = ram_from_tom(tom);
    for addr in 0..0x10000 {
        if sram[addr] != tram[addr] {
            return false;
        }
    }
    return true;
}
fn compare_status(tom: &TomHarte, simple: &SimpleBus) -> bool {
    let c_pc = simple.cpu.registers.pc == tom.pc;
    let c_sp = simple.cpu.registers.sp == tom.s;
    let c_acc = simple.cpu.registers.acc == tom.a;
    let c_idx = simple.cpu.registers.idx == tom.x;
    let c_idy = simple.cpu.registers.idy == tom.y;
    let c_ps = simple.cpu.registers.ps == tom.p;

    return c_pc && c_sp && c_acc && c_idx && c_idy && c_ps;
}

fn compare(tom: &TomHarte, simple: &SimpleBus) -> bool {
    let c_memory = compare_memory(tom, simple);
    let c_status = compare_status(tom, simple);
    return c_memory && c_status;
}

fn from_tom(tom: &TomHarte) -> SimpleBus {
    let c = mos_from_tom(tom);
    let r = ram_from_tom(tom);
    let cyc: Cycles = Cycles(vec![]);
    let mut simple = SimpleBus {
        cpu: c,
        ram: r,
        cycles: cyc,
    };
    set_decimal_enabled(&mut simple, true);
    return simple;
}

fn test(t: Test) {
    let initial_state = t.initial_state;
    let mut simple = from_tom(&initial_state);
    coral::mos::tick(&mut simple);
    let result = compare(&t.final_state, &simple);
    let prediction = to_tom(&simple);

    let mut assertion_message = String::new();

    assertion_message.push_str(&format!("\nName: {}\n", t.name));
    assertion_message.push_str(&format!("\nInitial State:\n{:?}\n", initial_state));
    assertion_message.push_str(&format!("\nFinal State:\n{:?}\n", t.final_state));
    assertion_message.push_str(&format!("\nMOS:\n{:?}\n", prediction));
    assertion_message.push_str(&format!("\nCycles: {:?}\n", t.cycles));
    assertion_message.push_str(&format!("\nMOS Cycles: {:?}\n", simple.cycles));

    assert!(result, "{}", assertion_message);
}

fn test_opcode(opcode: u8) {
    let tests = load_tests(opcode).unwrap();
    for t in tests.0 {
        test(t);
    }
}

fn load_tests(opcode: u8) -> io::Result<Tests> {
    let filepath = format!("tests/TomHarte/6502/v1/{:02x}.json", opcode);
    println!("{}", filepath);
    let mut file = File::open(filepath)?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let contents =
        String::from_utf8(buffer).map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
    let tests: Tests = serde_json::from_str(&contents)
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
    return Ok(tests);
}

#[test]
fn test_0x69() {
    test_opcode(0x69);
}
#[test]
fn test_0x65() {
    test_opcode(0x65);
}
#[test]
fn test_0x75() {
    test_opcode(0x75);
}
#[test]
fn test_0x6D() {
    test_opcode(0x6D);
}
#[test]
fn test_0x7D() {
    test_opcode(0x7D);
}
#[test]
fn test_0x79() {
    test_opcode(0x79);
}
#[test]
fn test_0x61() {
    test_opcode(0x61);
}
#[test]
fn test_0x71() {
    test_opcode(0x71);
}
#[test]
fn test_0x29() {
    test_opcode(0x29);
}
#[test]
fn test_0x25() {
    test_opcode(0x25);
}
#[test]
fn test_0x35() {
    test_opcode(0x35);
}
#[test]
fn test_0x2D() {
    test_opcode(0x2D);
}
#[test]
fn test_0x3D() {
    test_opcode(0x3D);
}
#[test]
fn test_0x39() {
    test_opcode(0x39);
}
#[test]
fn test_0x21() {
    test_opcode(0x21);
}
#[test]
fn test_0x31() {
    test_opcode(0x31);
}
#[test]
fn test_0x0A() {
    test_opcode(0x0A);
}
#[test]
fn test_0x06() {
    test_opcode(0x06);
}
#[test]
fn test_0x16() {
    test_opcode(0x16);
}
#[test]
fn test_0x0E() {
    test_opcode(0x0E);
}
#[test]
fn test_0x1E() {
    test_opcode(0x1E);
}
#[test]
fn test_0x90() {
    test_opcode(0x90);
}
#[test]
fn test_0xB0() {
    test_opcode(0xB0);
}
#[test]
fn test_0xF0() {
    test_opcode(0xF0);
}
#[test]
fn test_0x24() {
    test_opcode(0x24);
}
#[test]
fn test_0x2C() {
    test_opcode(0x2C);
}
#[test]
fn test_0x30() {
    test_opcode(0x30);
}
#[test]
fn test_0xD0() {
    test_opcode(0xD0);
}
#[test]
fn test_0x10() {
    test_opcode(0x10);
}
#[test]
fn test_0x00() {
    test_opcode(0x00);
}
#[test]
fn test_0x50() {
    test_opcode(0x50);
}
#[test]
fn test_0x70() {
    test_opcode(0x70);
}
#[test]
fn test_0x18() {
    test_opcode(0x18);
}
#[test]
fn test_0xD8() {
    test_opcode(0xD8);
}
#[test]
fn test_0x58() {
    test_opcode(0x58);
}
#[test]
fn test_0xB8() {
    test_opcode(0xB8);
}
#[test]
fn test_0xC9() {
    test_opcode(0xC9);
}
#[test]
fn test_0xC5() {
    test_opcode(0xC5);
}
#[test]
fn test_0xD5() {
    test_opcode(0xD5);
}
#[test]
fn test_0xCD() {
    test_opcode(0xCD);
}
#[test]
fn test_0xDD() {
    test_opcode(0xDD);
}
#[test]
fn test_0xD9() {
    test_opcode(0xD9);
}
#[test]
fn test_0xC1() {
    test_opcode(0xC1);
}
#[test]
fn test_0xD1() {
    test_opcode(0xD1);
}
#[test]
fn test_0xE0() {
    test_opcode(0xE0);
}
#[test]
fn test_0xE4() {
    test_opcode(0xE4);
}
#[test]
fn test_0xEC() {
    test_opcode(0xEC);
}
#[test]
fn test_0xC0() {
    test_opcode(0xC0);
}
#[test]
fn test_0xC4() {
    test_opcode(0xC4);
}
#[test]
fn test_0xCC() {
    test_opcode(0xCC);
}
#[test]
fn test_0xC6() {
    test_opcode(0xC6);
}
#[test]
fn test_0xD6() {
    test_opcode(0xD6);
}
#[test]
fn test_0xCE() {
    test_opcode(0xCE);
}
#[test]
fn test_0xDE() {
    test_opcode(0xDE);
}
#[test]
fn test_0xCA() {
    test_opcode(0xCA);
}
#[test]
fn test_0x88() {
    test_opcode(0x88);
}
#[test]
fn test_0x49() {
    test_opcode(0x49);
}
#[test]
fn test_0x45() {
    test_opcode(0x45);
}
#[test]
fn test_0x55() {
    test_opcode(0x55);
}
#[test]
fn test_0x4D() {
    test_opcode(0x4D);
}
#[test]
fn test_0x5D() {
    test_opcode(0x5D);
}
#[test]
fn test_0x59() {
    test_opcode(0x59);
}
#[test]
fn test_0x41() {
    test_opcode(0x41);
}
#[test]
fn test_0x51() {
    test_opcode(0x51);
}
#[test]
fn test_0xE6() {
    test_opcode(0xE6);
}
#[test]
fn test_0xF6() {
    test_opcode(0xF6);
}
#[test]
fn test_0xEE() {
    test_opcode(0xEE);
}
#[test]
fn test_0xFE() {
    test_opcode(0xFE);
}
#[test]
fn test_0xE8() {
    test_opcode(0xE8);
}
#[test]
fn test_0xC8() {
    test_opcode(0xC8);
}
#[test]
fn test_0x4C() {
    test_opcode(0x4C);
}
#[test]
fn test_0x6C() {
    test_opcode(0x6C);
}
#[test]
fn test_0x20() {
    test_opcode(0x20);
}
#[test]
fn test_0xA9() {
    test_opcode(0xA9);
}
#[test]
fn test_0xA5() {
    test_opcode(0xA5);
}
#[test]
fn test_0xB5() {
    test_opcode(0xB5);
}
#[test]
fn test_0xAD() {
    test_opcode(0xAD);
}
#[test]
fn test_0xBD() {
    test_opcode(0xBD);
}
#[test]
fn test_0xB9() {
    test_opcode(0xB9);
}
#[test]
fn test_0xA1() {
    test_opcode(0xA1);
}
#[test]
fn test_0xB1() {
    test_opcode(0xB1);
}
#[test]
fn test_0xA2() {
    test_opcode(0xA2);
}
#[test]
fn test_0xA6() {
    test_opcode(0xA6);
}
#[test]
fn test_0xB6() {
    test_opcode(0xB6);
}
#[test]
fn test_0xAE() {
    test_opcode(0xAE);
}
#[test]
fn test_0xBE() {
    test_opcode(0xBE);
}
#[test]
fn test_0xA0() {
    test_opcode(0xA0);
}
#[test]
fn test_0xA4() {
    test_opcode(0xA4);
}
#[test]
fn test_0xB4() {
    test_opcode(0xB4);
}
#[test]
fn test_0xAC() {
    test_opcode(0xAC);
}
#[test]
fn test_0xBC() {
    test_opcode(0xBC);
}
#[test]
fn test_0x4A() {
    test_opcode(0x4A);
}
#[test]
fn test_0x46() {
    test_opcode(0x46);
}
#[test]
fn test_0x56() {
    test_opcode(0x56);
}
#[test]
fn test_0x4E() {
    test_opcode(0x4E);
}
#[test]
fn test_0x5E() {
    test_opcode(0x5E);
}
#[test]
fn test_0xEA() {
    test_opcode(0xEA);
}
#[test]
fn test_0x09() {
    test_opcode(0x09);
}
#[test]
fn test_0x05() {
    test_opcode(0x05);
}
#[test]
fn test_0x15() {
    test_opcode(0x15);
}
#[test]
fn test_0x0D() {
    test_opcode(0x0D);
}
#[test]
fn test_0x1D() {
    test_opcode(0x1D);
}
#[test]
fn test_0x19() {
    test_opcode(0x19);
}
#[test]
fn test_0x01() {
    test_opcode(0x01);
}
#[test]
fn test_0x11() {
    test_opcode(0x11);
}
#[test]
fn test_0x48() {
    test_opcode(0x48);
}
#[test]
fn test_0x08() {
    test_opcode(0x08);
}
#[test]
fn test_0x68() {
    test_opcode(0x68);
}
#[test]
fn test_0x28() {
    test_opcode(0x28);
}
#[test]
fn test_0x2A() {
    test_opcode(0x2A);
}
#[test]
fn test_0x26() {
    test_opcode(0x26);
}
#[test]
fn test_0x36() {
    test_opcode(0x36);
}
#[test]
fn test_0x2E() {
    test_opcode(0x2E);
}
#[test]
fn test_0x3E() {
    test_opcode(0x3E);
}
#[test]
fn test_0x6A() {
    test_opcode(0x6A);
}
#[test]
fn test_0x66() {
    test_opcode(0x66);
}
#[test]
fn test_0x76() {
    test_opcode(0x76);
}
#[test]
fn test_0x6E() {
    test_opcode(0x6E);
}
#[test]
fn test_0x7E() {
    test_opcode(0x7E);
}
#[test]
fn test_0x40() {
    test_opcode(0x40);
}
#[test]
fn test_0x60() {
    test_opcode(0x60);
}
#[test]
fn test_0xE9() {
    test_opcode(0xE9);
}
#[test]
fn test_0xE5() {
    test_opcode(0xE5);
}
#[test]
fn test_0xF5() {
    test_opcode(0xF5);
}
#[test]
fn test_0xED() {
    test_opcode(0xED);
}
#[test]
fn test_0xFD() {
    test_opcode(0xFD);
}
#[test]
fn test_0xF9() {
    test_opcode(0xF9);
}
#[test]
fn test_0xE1() {
    test_opcode(0xE1);
}
#[test]
fn test_0xF1() {
    test_opcode(0xF1);
}
#[test]
fn test_0x38() {
    test_opcode(0x38);
}
#[test]
fn test_0xF8() {
    test_opcode(0xF8);
}
#[test]
fn test_0x78() {
    test_opcode(0x78);
}
#[test]
fn test_0x85() {
    test_opcode(0x85);
}
#[test]
fn test_0x95() {
    test_opcode(0x95);
}
#[test]
fn test_0x8D() {
    test_opcode(0x8D);
}
#[test]
fn test_0x9D() {
    test_opcode(0x9D);
}
#[test]
fn test_0x99() {
    test_opcode(0x99);
}
#[test]
fn test_0x81() {
    test_opcode(0x81);
}
#[test]
fn test_0x91() {
    test_opcode(0x91);
}
#[test]
fn test_0x86() {
    test_opcode(0x86);
}
#[test]
fn test_0x96() {
    test_opcode(0x96);
}
#[test]
fn test_0x8E() {
    test_opcode(0x8E);
}
#[test]
fn test_0x84() {
    test_opcode(0x84);
}
#[test]
fn test_0x94() {
    test_opcode(0x94);
}
#[test]
fn test_0x8C() {
    test_opcode(0x8C);
}
#[test]
fn test_0xAA() {
    test_opcode(0xAA);
}
#[test]
fn test_0xA8() {
    test_opcode(0xA8);
}
#[test]
fn test_0xBA() {
    test_opcode(0xBA);
}
#[test]
fn test_0x8A() {
    test_opcode(0x8A);
}
#[test]
fn test_0x9A() {
    test_opcode(0x9A);
}
#[test]
fn test_0x98() {
    test_opcode(0x98);
}
