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
