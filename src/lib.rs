use std::fmt;

pub struct Uint256 {
    bytes: [u8; 32],
}

impl fmt::Display for Uint256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        for byte in self.bytes.iter() {
            let v = format!("{:02x}", byte);
            let a = v.as_str();
            str = a.to_owned() + &str;
        }
        write!(f, "0x{}", str)
    }
}

impl Uint256 {
    const NUM_BYTES: usize = 32;

    pub fn zero() -> Self {
        Uint256 { bytes: [0_u8; 32] }
    }

    pub fn one() -> Self {
        let mut bytes = [0_u8; 32];
        bytes[0] = 1_u8;
        Uint256 { bytes }
    }

    pub fn from_u64(value: u64) -> Self {
        let mut bytes = [0_u8; 32];
        let value_bytes = value.to_be_bytes();
        for i in 0..8 {
            bytes[i] = value_bytes[i];
        }
        Uint256 { bytes }
    }

    pub fn get_byte(&self, index: usize) -> u8 {
        self.bytes[index]
    }

    pub fn shift_left(&mut self, places: usize) {
        let byte_shift = places / 8;
        let bit_shift = places % 8;

        if byte_shift > 0 {
            let mut i = Self::NUM_BYTES - 1;
            while i >= byte_shift {
                self.bytes[i] = self.bytes[i - byte_shift];
                i -= 1;
            }

            for i in 0..byte_shift {
                self.bytes[i] = 0;
            }
        }

        if bit_shift > 0 {
            let mut i = Self::NUM_BYTES - 1;
            while i > 0 {
                self.bytes[i] =
                    (self.bytes[i] << bit_shift) | (self.bytes[i - 1] >> (8 - bit_shift));
                i -= 1;
            }
            self.bytes[0] <<= bit_shift;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero() {
        let zero = Uint256::zero();
        assert_eq!(
            zero.to_string(),
            "0x0000000000000000000000000000000000000000000000000000000000000000"
        );
    }

    #[test]
    fn one() {
        let one = Uint256::one();
        assert_eq!(
            one.to_string(),
            "0x0000000000000000000000000000000000000000000000000000000000000001"
        );
    }

    #[test]
    fn from_u64() {
        assert_eq!(
            Uint256::from_u64(u64::MAX).to_string(),
            "0x000000000000000000000000000000000000000000000000ffffffffffffffff"
        );
    }

    #[test]
    fn shift_left() {
        // shifting one

        let mut one = Uint256::one();
        assert_eq!(
            one.to_string(),
            "0x0000000000000000000000000000000000000000000000000000000000000001"
        );

        one.shift_left(1);
        assert_eq!(
            one.to_string(),
            "0x0000000000000000000000000000000000000000000000000000000000000002"
        );

        one.shift_left(7);
        assert_eq!(
            one.to_string(),
            "0x0000000000000000000000000000000000000000000000000000000000000100"
        );

        one.shift_left(16);
        assert_eq!(
            one.to_string(),
            "0x0000000000000000000000000000000000000000000000000000000001000000"
        );

        // shifting zero

        let mut zero = Uint256::zero();
        assert_eq!(
            zero.to_string(),
            "0x0000000000000000000000000000000000000000000000000000000000000000"
        );

        zero.shift_left(128);
        assert_eq!(
            zero.to_string(),
            "0x0000000000000000000000000000000000000000000000000000000000000000"
        );
    }
}
