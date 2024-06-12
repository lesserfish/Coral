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


pub fn join_bytes(msb : u8, lsb : u8) -> u16 {
    (msb as u16) << 8 + (lsb as u16)
}


fn add_signed_offset(x : u16, y : u16) -> u16 {
    let offset : u16 = if B7(y) {0xFF00 + y} else {y};
    x + (y & 0x7F) + offset
}
