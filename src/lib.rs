use std::fmt;

#[derive(Clone, Copy)]
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

    pub fn from_u8(value: u8) -> Self {
        let mut bytes = [0_u8; 32];
        bytes[0] = value;
        Uint256 { bytes }
    }

    pub fn from_u32(value: u32) -> Self {
        Self::from_hexa_str(format!("{:08x}", value).as_str())
    }

    pub fn from_slice(slice: &[u8]) -> Self {
        if slice.len() > Self::NUM_BYTES {
            panic!("invalid construction")
        }
        let mut bytes = [0u8; 32];
        let mut i = 0;
        for byte in slice.into_iter() {
            bytes[i] = *byte;
            i += 1;
        }
        Uint256 { bytes }
    }

    pub fn from_hexa_str(hexa_str: &str) -> Self {
        let mut bytes = [0_u8; 32];

        let mut _hexa_str = hexa_str.to_owned();
        if _hexa_str.starts_with("0x") || _hexa_str.starts_with("0X") {
            _hexa_str = _hexa_str[2..].to_owned();
        }

        if _hexa_str.len() == 0 {
            panic!("invalid hexa_str {}", _hexa_str);
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
                'a' => 0xA_u8,
                'b' => 0xB_u8,
                'c' => 0xC_u8,
                'd' => 0xD_u8,
                'e' => 0xE_u8,
                'f' => 0xF_u8,
                _ => panic!("invalid char {}", character),
            }
        }

        let mut index = 0;
        _hexa_str
            .chars()
            .collect::<Vec<char>>()
            .chunks(2)
            .rev()
            .for_each(|digits| {
                let value = (get_value(digits[0]) << 4) | get_value(digits[1]);
                bytes[index] = value;
                index += 1;
            });

        Uint256 { bytes }
    }

    pub fn get_byte(&self, index: usize) -> u8 {
        self.bytes[index]
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..]
    }

    pub fn to_u8(&self) -> u8 {
        // FIX: manage this as overflow
        if Self::NUM_BYTES < 1 {
            panic!("invalid conversion to u8")
        }
        for index in 1..Self::NUM_BYTES {
            if self.bytes[index] != 0u8 {
                panic!("invalid conversion to u8 (overflow)")
            }
        }
        self.bytes[0]
    }

    pub fn to_u32(&self) -> u32 {
        // FIX: manage this as overflow
        if Self::NUM_BYTES < 4 {
            panic!("invalid conversion to u32")
        }
        for index in 4..Self::NUM_BYTES {
            if self.bytes[index] != 0u8 {
                panic!("invalid conversion to u32 (overflow)")
            }
        }
        // FIX: subject to representation being hexadecimal
        let mut _hexa_str = self.to_string();
        if _hexa_str.starts_with("0x") || _hexa_str.starts_with("0X") {
            _hexa_str = _hexa_str[2..].to_owned();
        }
        u32::from_str_radix(_hexa_str.as_str(), 16).unwrap()
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

            while i < Self::NUM_BYTES - byte_shift {
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

    pub fn add(&self, other: Self) -> Self {
        let mut bytes = [0_u8; 32];
        let mut carry = 0_u8;
        for i in 0..Self::NUM_BYTES {
            let result: u16 = carry as u16 + self.bytes[i] as u16 + other.bytes[i] as u16;
            let addition_bytes = result.to_ne_bytes();
            bytes[i] = addition_bytes[0];
            carry = addition_bytes[1];
        }
        Uint256 { bytes }
    }

    pub fn mul(&self, _other: Self) -> Self {
        Uint256::zero()
    }

    pub fn to_binary_string(&self) -> String {
        let mut str = "".to_owned();

        let mut bytes = self.bytes.clone();
        bytes.reverse();

        let mut index = Self::NUM_BYTES;
        bytes.chunks(4).for_each(|chunk| {
            str += &format!(
                "[{0:02}..{1:02}] {2:08b} ({2:02x}) | {3:08b} ({3:02x}) | {4:08b} ({4:02x}) | {5:08b} ({5:02x})\n",
                index - 1,
                index - 4,
                chunk[0],
                chunk[1],
                chunk[2],
                chunk[3]
            );
            index -= 4;
        });

        str
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
            Uint256::from_hexa_str(format!("{:016x}", 0x001122334455667788_u64).as_str())
                .to_string()
                .to_ascii_lowercase(),
            "0x0000000000000000000000000000000000000000000000001122334455667788"
                .to_ascii_lowercase()
        );
        assert_eq!(
            Uint256::from_hexa_str(format!("{:016x}", u64::MAX).as_str())
                .to_string()
                .to_ascii_lowercase(),
            "0x000000000000000000000000000000000000000000000000ffffffffffffffff"
                .to_ascii_lowercase()
        );
        assert_eq!(
            Uint256::from_hexa_str(
                "0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"
            )
            .to_string()
            .to_ascii_lowercase(),
            "0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"
                .to_ascii_lowercase()
        );
    }

    #[test]
    fn shift_left() {
        // shifting one

        let mut one = Uint256::one();
        assert_eq!(
            one.to_string().to_ascii_lowercase(),
            "0x0000000000000000000000000000000000000000000000000000000000000001"
                .to_ascii_lowercase()
        );

        one.shift_left(1);
        assert_eq!(
            one.to_string().to_ascii_lowercase(),
            "0x0000000000000000000000000000000000000000000000000000000000000002"
                .to_ascii_lowercase()
        );

        one.shift_left(7);
        assert_eq!(
            one.to_string().to_ascii_lowercase(),
            "0x0000000000000000000000000000000000000000000000000000000000000100"
                .to_ascii_lowercase()
        );

        one.shift_left(16);
        assert_eq!(
            one.to_string().to_ascii_lowercase(),
            "0x0000000000000000000000000000000000000000000000000000000001000000"
                .to_ascii_lowercase()
        );

        // shifting zero

        let mut zero = Uint256::zero();
        assert_eq!(
            zero.to_string().to_ascii_lowercase(),
            "0x0000000000000000000000000000000000000000000000000000000000000000"
                .to_ascii_lowercase()
        );

        zero.shift_left(128);
        assert_eq!(
            zero.to_string().to_ascii_lowercase(),
            "0x0000000000000000000000000000000000000000000000000000000000000000"
                .to_ascii_lowercase()
        );
    }

    #[test]
    fn shift_right() {
        // shifting some number

        let mut number = Uint256::from_hexa_str("0x40000000");
        assert_eq!(
            number.to_string().to_ascii_lowercase(),
            "0x0000000000000000000000000000000000000000000000000000000040000000"
                .to_ascii_lowercase()
        );

        number.shift_right(1);
        assert_eq!(
            number.to_string(),
            "0x0000000000000000000000000000000000000000000000000000000020000000"
        );

        number.shift_right(7);
        assert_eq!(
            number.to_string(),
            "0x0000000000000000000000000000000000000000000000000000000000400000"
        );

        number.shift_right(16);
        assert_eq!(
            number.to_string(),
            "0x0000000000000000000000000000000000000000000000000000000000000040"
        );

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

    #[test]
    fn to_binary_string() {
        let number = Uint256::from_hexa_str(
            "0x1F1E1D1C1B1A191817161514131211100F0E0D0C0B0A09080706050403020100",
        );
        println!("{}", number.to_binary_string());
    }

    #[test]
    fn shift_bytes_two_way() {
        let mut number = Uint256::from_hexa_str("0x01");
        println!("1:\n{}", number.to_binary_string());
        assert_eq!(
            number.to_string().to_ascii_lowercase(),
            "0x0000000000000000000000000000000000000000000000000000000000000001"
                .to_ascii_lowercase()
        );

        let bytes_to_shift = 31;
        let bits_to_shift = 8 * bytes_to_shift;

        number.shift_left(bits_to_shift);
        println!(
            "after << {} bits ({} bytes):\n{}",
            bits_to_shift,
            bytes_to_shift,
            number.to_binary_string()
        );

        assert_eq!(
            number.to_string().to_ascii_lowercase(),
            "0x0100000000000000000000000000000000000000000000000000000000000000"
                .to_ascii_lowercase()
        );

        number.shift_right(bits_to_shift);
        println!(
            "after >> {} bits ({} bytes):\n{}",
            bits_to_shift,
            bytes_to_shift,
            number.to_binary_string()
        );

        assert_eq!(
            number.to_string().to_ascii_lowercase(),
            "0x0000000000000000000000000000000000000000000000000000000000000001"
                .to_ascii_lowercase()
        );
    }

    #[test]
    fn shift_bytes_and_bits_two_way() {
        let mut number = Uint256::from_hexa_str("0x01");
        println!("1:\n{}", number.to_binary_string());
        assert_eq!(
            number.to_string().to_ascii_lowercase(),
            "0x0000000000000000000000000000000000000000000000000000000000000001"
                .to_ascii_lowercase()
        );

        let bytes_to_shift = 31;
        let extra_bits = 4;
        let bits_to_shift = 8 * bytes_to_shift + extra_bits;

        number.shift_left(bits_to_shift);
        println!(
            "after << {} bits ({} bytes + {} extra bits):\n{}",
            bits_to_shift,
            bytes_to_shift,
            extra_bits,
            number.to_binary_string()
        );

        assert_eq!(
            number.to_string().to_ascii_lowercase(),
            "0x1000000000000000000000000000000000000000000000000000000000000000"
                .to_ascii_lowercase()
        );

        number.shift_right(bits_to_shift);
        println!(
            "after >> {} bits ({} bytes + {} extra bits):\n{}",
            bits_to_shift,
            bytes_to_shift,
            extra_bits,
            number.to_binary_string()
        );

        assert_eq!(
            number.to_string().to_ascii_lowercase(),
            "0x0000000000000000000000000000000000000000000000000000000000000001"
                .to_ascii_lowercase()
        );
    }

    #[test]
    fn from_u8() {
        assert_eq!(
            Uint256::from_u8(0x01_u8).to_string().to_ascii_lowercase(),
            "0x0000000000000000000000000000000000000000000000000000000000000001"
                .to_ascii_lowercase()
        );

        assert_eq!(
            Uint256::from_u8(0xFF_u8).to_string().to_ascii_lowercase(),
            "0x00000000000000000000000000000000000000000000000000000000000000FF"
                .to_ascii_lowercase()
        );

        assert_eq!(
            Uint256::from_u8(0x00_u8).to_string().to_ascii_lowercase(),
            "0x0000000000000000000000000000000000000000000000000000000000000000"
                .to_ascii_lowercase()
        );
    }

    #[test]
    fn from_u32() {
        assert_eq!(
            Uint256::from_u32(0x01_u32).to_string().to_ascii_lowercase(),
            "0x0000000000000000000000000000000000000000000000000000000000000001"
                .to_ascii_lowercase()
        );

        assert_eq!(
            Uint256::from_u32(0xFFFFFFFF_u32)
                .to_string()
                .to_ascii_lowercase(),
            "0x00000000000000000000000000000000000000000000000000000000FFFFFFFF"
                .to_ascii_lowercase()
        );

        assert_eq!(
            Uint256::from_u32(0x00_u32).to_string().to_ascii_lowercase(),
            "0x0000000000000000000000000000000000000000000000000000000000000000"
                .to_ascii_lowercase()
        );
    }

    #[test]
    fn to_u8() {
        assert_eq!(Uint256::from_u8(0x01_u8).to_u8(), 0x01_u8);
    }

    #[test]
    fn to_u32() {
        let value = Uint256::from_u32(0x01_u32).to_u32();
        assert_eq!(value, 0x01_u32);
    }

    #[test]
    fn from_slice() {
        let bytes = [
            0x00_u8, 0x01_u8, 0x02_u8, 0x03_u8, 0x04_u8, 0x05_u8, 0x06_u8, 0x07_u8, 0x08_u8,
            0x09_u8, 0x0A_u8, 0x0B_u8, 0x0C_u8, 0x0D_u8, 0x0E_u8, 0x0F_u8, 0x10_u8, 0x11_u8,
            0x12_u8, 0x13_u8, 0x14_u8, 0x15_u8, 0x16_u8, 0x17_u8, 0x18_u8, 0x19_u8, 0x1A_u8,
            0x1B_u8, 0x1C_u8, 0x1D_u8, 0x1E_u8, 0x1F_u8,
        ];
        let number = Uint256::from_slice(&bytes[..]);
        assert_eq!(
            number.to_string().to_ascii_lowercase(),
            "0x1F1E1D1C1B1A191817161514131211100F0E0D0C0B0A09080706050403020100"
                .to_ascii_lowercase(),
        );
    }

    #[test]
    fn add() {
        assert_eq!(
            Uint256::from_u32(10_u32)
                .add(Uint256::from_u32(10_u32))
                .to_string()
                .to_ascii_lowercase(),
            "0x0000000000000000000000000000000000000000000000000000000000000014"
                .to_ascii_lowercase(),
        );

        assert_eq!(
            Uint256::from_hexa_str(
                "0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"
            )
            .add(Uint256::from_u32(1_u32))
            .to_string()
            .to_ascii_lowercase(),
            "0x0000000000000000000000000000000000000000000000000000000000000000"
                .to_ascii_lowercase(),
        );
    }

    // #[test]
    // fn mul() {
    //     assert_eq!(
    //         Uint256::from_u32(10_u32)
    //             .add(Uint256::from_u32(10_u32))
    //             .to_string()
    //             .to_ascii_lowercase(),
    //         "0x0000000000000000000000000000000000000000000000000000000000000064"
    //             .to_ascii_lowercase(),
    //     );

    //     assert_eq!(
    //         Uint256::from_hexa_str(
    //             "0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"
    //         )
    //         .mul(Uint256::from_u32(2_u32))
    //         .to_string()
    //         .to_ascii_lowercase(),
    //         "0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFE"
    //             .to_ascii_lowercase(),
    //     );
    // }
}
