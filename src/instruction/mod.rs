pub mod instr;

// constant for instruction parsing
const DR_SHIFT: usize = 9;
const SR1_SHIFT: usize = 6;
const REG_MASK: u16 = 0x7;
const IMM_BIT: usize = 5;

pub fn sign_extend(mut x: u16, bit_count: u8) -> u16 {
    if (x >> (bit_count - 1)) & 1 != 0 {
        x |= 0xFFFF << bit_count;
    }
    x
}

#[cfg(test)]
mod sign_extension_test {

    use super::*;

    #[test]
    fn positive_number() {
        let x = 0b0110;
        let y = sign_extend(x, 4);
        assert_eq!(x, y);
    }

    #[test]
    fn negative_number() {
        let x = 0b1100 as u16;
        let y = sign_extend(x, 4);
        assert_eq!(y, 0xFFFC);
    }
}
