struct _256 {
    bytes: [u8; 32]
}

struct __256<'a> {
    bytes: &'a [u8]
}

pub struct Uint256 {
    bytes: Box<[u8]>,
}

impl Uint256 {
    pub fn zero() -> Self {
        Uint256 {
            bytes: Box::new([0_u8; 32]),
        }
    }

    pub fn get_byte(self, index: usize) -> u8 {
        self.bytes[index]
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
