use serde::{Deserialize, Serialize};

use crate::{ErrorKind, Result};

const INVALID_PINS: [[u8; 8]; 12] = [
    [1, 2, 3, 4, 5, 6, 7, 8],
    [8, 7, 6, 5, 4, 3, 2, 1],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 1, 1, 1],
    [2, 2, 2, 2, 2, 2, 2, 2],
    [3, 3, 3, 3, 3, 3, 3, 3],
    [4, 4, 4, 4, 4, 4, 4, 4],
    [5, 5, 5, 5, 5, 5, 5, 5],
    [6, 6, 6, 6, 6, 6, 6, 6],
    [7, 7, 7, 7, 7, 7, 7, 7],
    [8, 8, 8, 8, 8, 8, 8, 8],
    [9, 9, 9, 9, 9, 9, 9, 9],
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pin {
    pin: [u8; 8],
}

impl Pin {
    pub fn new(pin: [u8; 8]) -> Result<Self> {
        if INVALID_PINS.contains(&pin) {
            return Err(ErrorKind::PinTooEasy.into());
        }
        for digit in &pin {
            if digit > &9 {
                return Err(ErrorKind::InvalidPin.into());
            }
        }

        Ok(Pin { pin })
    }

    pub fn to_string(&self) -> String {
        format!(
            "{}{}{}-{}{}-{}{}{}",
            &self.pin[0],
            &self.pin[1],
            &self.pin[2],
            &self.pin[3],
            &self.pin[4],
            &self.pin[5],
            &self.pin[6],
            &self.pin[7],
        )
    }

    // TODO: fix UTF-8 encoding here
    // pub fn as_bytes(&self) -> [u8; 10] {
    //     [
    //         self.pin[0],
    //         self.pin[1],
    //         self.pin[2],
    //         45, // '-'
    //         self.pin[3],
    //         self.pin[4],
    //         45, // '-'
    //         self.pin[5],
    //         self.pin[6],
    //         self.pin[7],
    //     ]
    // }
}

mod tests {
    use super::*;

    #[test]
    fn test_invalid_pin() {
        let too_easy_pin = Pin::new([1, 2, 3, 4, 5, 6, 7, 8]);
        let pin_with_invalid_number = Pin::new([0, 0, 0, 0, 0, 0, 0, 123]);
    }

    #[test]
    fn test_to_string() {
        let pin = Pin::new([1, 1, 1, 2, 2, 3, 3, 3]).unwrap();
        assert_eq!(pin.to_string(), "111-22-333".to_string());
    }

    // #[test]
    // fn test_as_bytes() {
    //     let pin = Pin::new([1, 1, 1, 2, 2, 3, 3, 3]).unwrap();
    //     let bytes = pin.as_bytes();
    //     assert_eq!(bytes, "111-22-333".to_string().as_bytes());
    // }
}
