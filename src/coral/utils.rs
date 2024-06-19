#![allow(unused)]
#![allow(non_snake_case)]

pub fn b0(i : u8) -> bool { i & 0x01 > 0 }
pub fn b1(i : u8) -> bool { i & 0x02 > 0 }
pub fn b2(i : u8) -> bool { i & 0x04 > 0 }
pub fn b3(i : u8) -> bool { i & 0x08 > 0 }
pub fn b4(i : u8) -> bool { i & 0x10 > 0 }
pub fn b5(i : u8) -> bool { i & 0x20 > 0 }
pub fn b6(i : u8) -> bool { i & 0x40 > 0 }
pub fn b7(i : u8) -> bool { i & 0x80 > 0 }

pub fn s0(i : &mut u8, v : bool) { if v { *i |= 0x01 } else { *i &= !0x01 } }
pub fn s1(i : &mut u8, v : bool) { if v { *i |= 0x02 } else { *i &= !0x02 } }
pub fn s2(i : &mut u8, v : bool) { if v { *i |= 0x04 } else { *i &= !0x04 } }
pub fn s3(i : &mut u8, v : bool) { if v { *i |= 0x08 } else { *i &= !0x08 } }
pub fn s4(i : &mut u8, v : bool) { if v { *i |= 0x10 } else { *i &= !0x10 } }
pub fn s5(i : &mut u8, v : bool) { if v { *i |= 0x20 } else { *i &= !0x20 } }
pub fn s6(i : &mut u8, v : bool) { if v { *i |= 0x40 } else { *i &= !0x40 } }
pub fn s7(i : &mut u8, v : bool) { if v { *i |= 0x80 } else { *i &= !0x80 } }

pub fn p0(i : u8, v : bool) -> u8 { if v { i | 0x01 } else { i & !0x01 } }
pub fn p1(i : u8, v : bool) -> u8 { if v { i | 0x02 } else { i & !0x02 } }
pub fn p2(i : u8, v : bool) -> u8 { if v { i | 0x04 } else { i & !0x04 } }
pub fn p3(i : u8, v : bool) -> u8 { if v { i | 0x08 } else { i & !0x08 } }
pub fn p4(i : u8, v : bool) -> u8 { if v { i | 0x10 } else { i & !0x10 } }
pub fn p5(i : u8, v : bool) -> u8 { if v { i | 0x20 } else { i & !0x20 } }
pub fn p6(i : u8, v : bool) -> u8 { if v { i | 0x40 } else { i & !0x40 } }
pub fn p7(i : u8, v : bool) -> u8 { if v { i | 0x80 } else { i & !0x80 } }

pub fn t1(i : u8) -> u8 { i & 0x01 }
pub fn t2(i : u8) -> u8 { i & 0x03 }
pub fn t3(i : u8) -> u8 { i & 0x07 }
pub fn t4(i : u8) -> u8 { i & 0x0F }
pub fn t5(i : u8) -> u8 { i & 0x1F }
pub fn t6(i : u8) -> u8 { i & 0x3F }
pub fn t7(i : u8) -> u8 { i & 0x7F }
pub fn t8(i : u8) -> u8 { i & 0xFF }

pub fn B0 (i : u16) -> bool { i & 0x0001 > 0 }
pub fn B1 (i : u16) -> bool { i & 0x0002 > 0 }
pub fn B2 (i : u16) -> bool { i & 0x0004 > 0 }
pub fn B3 (i : u16) -> bool { i & 0x0008 > 0 }
pub fn B4 (i : u16) -> bool { i & 0x0010 > 0 }
pub fn B5 (i : u16) -> bool { i & 0x0020 > 0 }
pub fn B6 (i : u16) -> bool { i & 0x0040 > 0 }
pub fn B7 (i : u16) -> bool { i & 0x0080 > 0 }
pub fn B8 (i : u16) -> bool { i & 0x0100 > 0 }
pub fn B9 (i : u16) -> bool { i & 0x0200 > 0 }
pub fn B10(i : u16) -> bool { i & 0x0400 > 0 }
pub fn B11(i : u16) -> bool { i & 0x0800 > 0 }
pub fn B12(i : u16) -> bool { i & 0x1000 > 0 }
pub fn B13(i : u16) -> bool { i & 0x2000 > 0 }
pub fn B14(i : u16) -> bool { i & 0x4000 > 0 }
pub fn B15(i : u16) -> bool { i & 0x8000 > 0 }

pub fn S0 (i : &mut u16, v : bool) { if v { *i |= 0x0001 } else { *i &= !0x0001 } }
pub fn S1 (i : &mut u16, v : bool) { if v { *i |= 0x0002 } else { *i &= !0x0002 } }
pub fn S2 (i : &mut u16, v : bool) { if v { *i |= 0x0004 } else { *i &= !0x0004 } }
pub fn S3 (i : &mut u16, v : bool) { if v { *i |= 0x0008 } else { *i &= !0x0008 } }
pub fn S4 (i : &mut u16, v : bool) { if v { *i |= 0x0010 } else { *i &= !0x0010 } }
pub fn S5 (i : &mut u16, v : bool) { if v { *i |= 0x0020 } else { *i &= !0x0020 } }
pub fn S6 (i : &mut u16, v : bool) { if v { *i |= 0x0040 } else { *i &= !0x0040 } }
pub fn S7 (i : &mut u16, v : bool) { if v { *i |= 0x0080 } else { *i &= !0x0080 } }
pub fn S8 (i : &mut u16, v : bool) { if v { *i |= 0x0100 } else { *i &= !0x0100 } }
pub fn S9 (i : &mut u16, v : bool) { if v { *i |= 0x0200 } else { *i &= !0x0200 } }
pub fn S10(i : &mut u16, v : bool) { if v { *i |= 0x0400 } else { *i &= !0x0400 } }
pub fn S11(i : &mut u16, v : bool) { if v { *i |= 0x0800 } else { *i &= !0x0800 } }
pub fn S12(i : &mut u16, v : bool) { if v { *i |= 0x1000 } else { *i &= !0x1000 } }
pub fn S13(i : &mut u16, v : bool) { if v { *i |= 0x2000 } else { *i &= !0x2000 } }
pub fn S14(i : &mut u16, v : bool) { if v { *i |= 0x4000 } else { *i &= !0x4000 } }
pub fn S15(i : &mut u16, v : bool) { if v { *i |= 0x8000 } else { *i &= !0x8000 } }

pub fn P0 (i : u16, v : bool) -> u16 { if v { i | 0x0001 } else { i & !0x0001 } }
pub fn P1 (i : u16, v : bool) -> u16 { if v { i | 0x0002 } else { i & !0x0002 } }
pub fn P2 (i : u16, v : bool) -> u16 { if v { i | 0x0004 } else { i & !0x0004 } }
pub fn P3 (i : u16, v : bool) -> u16 { if v { i | 0x0008 } else { i & !0x0008 } }
pub fn P4 (i : u16, v : bool) -> u16 { if v { i | 0x0010 } else { i & !0x0010 } }
pub fn P5 (i : u16, v : bool) -> u16 { if v { i | 0x0020 } else { i & !0x0020 } }
pub fn P6 (i : u16, v : bool) -> u16 { if v { i | 0x0040 } else { i & !0x0040 } }
pub fn P7 (i : u16, v : bool) -> u16 { if v { i | 0x0080 } else { i & !0x0080 } }
pub fn P8 (i : u16, v : bool) -> u16 { if v { i | 0x0100 } else { i & !0x0100 } }
pub fn P9 (i : u16, v : bool) -> u16 { if v { i | 0x0200 } else { i & !0x0200 } }
pub fn P10(i : u16, v : bool) -> u16 { if v { i | 0x0400 } else { i & !0x0400 } }
pub fn P11(i : u16, v : bool) -> u16 { if v { i | 0x0800 } else { i & !0x0800 } }
pub fn P12(i : u16, v : bool) -> u16 { if v { i | 0x1000 } else { i & !0x1000 } }
pub fn P13(i : u16, v : bool) -> u16 { if v { i | 0x2000 } else { i & !0x2000 } }
pub fn P14(i : u16, v : bool) -> u16 { if v { i | 0x4000 } else { i & !0x4000 } }
pub fn P15(i : u16, v : bool) -> u16 { if v { i | 0x8000 } else { i & !0x8000 } }

pub fn T1 (i : u16) -> u16 { i & 0x0001 }
pub fn T2 (i : u16) -> u16 { i & 0x0003 }
pub fn T3 (i : u16) -> u16 { i & 0x0007 }
pub fn T4 (i : u16) -> u16 { i & 0x000F }
pub fn T5 (i : u16) -> u16 { i & 0x001F }
pub fn T6 (i : u16) -> u16 { i & 0x003F }
pub fn T7 (i : u16) -> u16 { i & 0x007F }
pub fn T8 (i : u16) -> u16 { i & 0x00FF }
pub fn T9 (i : u16) -> u16 { i & 0x01FF }
pub fn T10(i : u16) -> u16 { i & 0x03FF }
pub fn T11(i : u16) -> u16 { i & 0x07FF }
pub fn T12(i : u16) -> u16 { i & 0x0FFF }
pub fn T13(i : u16) -> u16 { i & 0x1FFF }
pub fn T14(i : u16) -> u16 { i & 0x3FFF }
pub fn T15(i : u16) -> u16 { i & 0x7FFF }
pub fn T16(i : u16) -> u16 { i & 0xFFFF }

pub fn join_bytes(msb : u8, lsb : u8) -> u16 {
    (msb as u16) << 8 | (lsb as u16)
}
pub fn split_bytes(x : u16) -> (u8, u8) {
    let lsb = x & 0x00FF;
    let msb = (x & 0xFF00) >> 8;
    (msb as u8, lsb as u8)
}

pub fn page_cross_sum(a : u16, b : u16) -> (u16, bool){
    let output = a + b;
    let page_cross = !(a & 0xFF00 == output & 0xFF00);
    (output, page_cross)
}


pub fn flatten_u8(data : [u8; 8]) -> u8 {
    let mut output : u8 = 0;
    for i in 0..8 {
        if data[i] > 0 {
            s0(&mut output, true);
        }
        output = output << 1;
    }
    output
}
