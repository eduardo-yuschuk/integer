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

    pub fn from_hexa_str(hexa_str: &str) -> Self {
        let mut bytes = [0_u8; 32];

        let mut _hexa_str = hexa_str.to_owned();
        if _hexa_str.starts_with("0x") || _hexa_str.starts_with("0X") {
            _hexa_str = _hexa_str[2..].to_owned();
        }

        if _hexa_str.len() == 0 {
            panic!("invalid hexa_str {}", hexa_str);
        }

        if _hexa_str.len() % 2 == 1 {
            _hexa_str = "0".to_owned() + _hexa_str.as_str();
        }

        fn get_value(character: char) -> u8 {
            match character.to_ascii_lowercase() {
                '0' => 0x0_u8,
                '1' => 0x1_u8,
                '2' => 0x2_u8,
                '3' => 0x3_u8,
                '4' => 0x4_u8,
                '5' => 0x5_u8,
                '6' => 0x6_u8,
                '7' => 0x7_u8,
                '8' => 0x8_u8,
                '9' => 0x9_u8,
                'A' => 0xA_u8,
                'B' => 0xB_u8,
                'C' => 0xC_u8,
                'D' => 0xD_u8,
                'E' => 0xE_u8,
                'F' => 0xF_u8,
                _ => panic!("invalid char {}", character),
            }
        }

        let mut index = 0;
        hexa_str
            .chars()
            .collect::<Vec<char>>()
            .chunks(2)
            .rev()
            .for_each(|digits| {
                let value = (get_value(digits[1]) << 4) | get_value(digits[0]);
                bytes[index] = value;
                index += 1;
            });

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

    pub fn shift_right(&mut self, places: usize) {
        let byte_shift = places / 8;
        let bit_shift = places % 8;

        if byte_shift > 0 {
            let mut i = 0;

            while i < Self::NUM_BYTES - 1 {
                self.bytes[i] = self.bytes[i + byte_shift];
                i += 1;
            }

            for i in (Self::NUM_BYTES - byte_shift)..Self::NUM_BYTES {
                self.bytes[i] = 0;
            }
        }

        if bit_shift > 0 {
            let mut i = 0;
            while i < (Self::NUM_BYTES - 1) {
                self.bytes[i] =
                    (self.bytes[i] >> bit_shift) | (self.bytes[i + 1] << (8 - bit_shift));
                i += 1;
            }
            self.bytes[Self::NUM_BYTES - 1] >>= bit_shift;
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
    fn from_hexa_str() {
        assert_eq!(
            Uint256::from_hexa_str(format!("{:016x}", u64::MAX).as_str()).to_string(),
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

    #[test]
    fn shift_right() {
        // shifting one

        let gibibyte = Uint256::from_hexa_str("0x40000000");
        assert_eq!(
            gibibyte.to_string(),
            "0x0000000000000000000000000000000000000000000000000000000040000000"
        );

        // one.shift_right(1);
        // assert_eq!(
        //     one.to_string(),
        //     "0x0000000000000000000000000000000000000000000000000000000000000002"
        // );

        // one.shift_right(7);
        // assert_eq!(
        //     one.to_string(),
        //     "0x0000000000000000000000000000000000000000000000000000000000000100"
        // );

        // one.shift_right(16);
        // assert_eq!(
        //     one.to_string(),
        //     "0x0000000000000000000000000000000000000000000000000000000001000000"
        // );

        // shifting zero

        let mut zero = Uint256::zero();
        assert_eq!(
            zero.to_string(),
            "0x0000000000000000000000000000000000000000000000000000000000000000"
        );

        zero.shift_right(128);
        assert_eq!(
            zero.to_string(),
            "0x0000000000000000000000000000000000000000000000000000000000000000"
        );
    }
}
