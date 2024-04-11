// struct _256 {
//     bytes: [u8; 32]
// }

// struct __256<'a> {
//     bytes: &'a [u8]
// }

pub struct Uint256 {
    bytes: Box<[u8]>,
}

impl Uint256 {
    pub fn zero() -> Self {
        Uint256 {
            bytes: Box::new([0_u8; 32]),
        }
    }

    pub fn one() -> Self {
        let mut bytes = [0_u8; 32];
        bytes[0] = 1_u8;
        Uint256 {
            bytes: Box::new(bytes),
        }
    }

    pub fn get_byte(&self, index: usize) -> u8 {
        self.bytes[index]
    }

    pub fn shift_left(&mut self, places: usize) {
        self.bytes[0] <<= places;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero() {
        let zero = Uint256::zero();
        for index in 0_usize..32_usize {
            assert_eq!(zero.get_byte(index), 0_u8);
        }
    }

    #[test]
    fn one() {
        let one = Uint256::one();
        assert_eq!(one.get_byte(0), 1_u8);
        for index in 1_usize..32_usize {
            assert_eq!(one.get_byte(index), 0_u8);
        }
    }

    #[test]
    fn shift_left_by_one() {
        let mut one = Uint256::one();
        one.shift_left(1);
        assert_eq!(one.get_byte(0), 0b00000010_u8);
        for index in 1_usize..32_usize {
            assert_eq!(one.get_byte(index), 0_u8);
        }
    }

    #[test]
    fn shift_left_by_two() {
        let mut one = Uint256::one();
        one.shift_left(2);
        assert_eq!(one.get_byte(0), 0b00000100_u8);
        for index in 1_usize..32_usize {
            assert_eq!(one.get_byte(index), 0_u8);
        }
    }
}
