#[derive(Debug, Clone)]
pub struct BitField {
    bits: Vec<u8>,
    length: usize,
}

impl BitField {
    pub fn new(length: usize) -> Self {
        let mut bits = vec![0; length / 8];
        if length % 8 != 0 {
            bits.push(0);
        }
        Self { bits, length }
    }

    pub fn set(&mut self, index: usize) {
        let byte = index / 8;
        let bit = index % 8;
        self.bits[byte] |= 1 << (7 - bit);
    }

    pub fn clear(&mut self, index: usize) {
        let byte = index / 8;
        let bit = index % 8;
        self.bits[byte] &= !(1 << (7 - bit));
    }

    pub fn get(&self, index: usize) -> bool {
        let byte = index / 8;
        let bit = index % 8;
        self.bits[byte] & (1 << (7 - bit)) != 0
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn iter(&self) -> BitFieldIter {
        BitFieldIter {
            bitfield: self,
            index: 0,
        }
    }
}

pub struct BitFieldIter<'a> {
    bitfield: &'a BitField,
    index: usize,
}

impl<'a> Iterator for BitFieldIter<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.bitfield.len() {
            let result = self.bitfield.get(self.index);
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitfield() {
        let mut bitfield = BitField::new(10);
        assert_eq!(bitfield.len(), 10);
        assert_eq!(bitfield.iter().count(), 10);
        assert_eq!(bitfield.iter().filter(|&b| b).count(), 0);

        bitfield.set(0);
        assert_eq!(bitfield.iter().filter(|&b| b).count(), 1);

        bitfield.set(9);
        assert_eq!(bitfield.iter().filter(|&b| b).count(), 2);

        bitfield.set(5);
        assert_eq!(bitfield.iter().filter(|&b| b).count(), 3);

        bitfield.set(5);
        assert_eq!(bitfield.iter().filter(|&b| b).count(), 3);

        bitfield.clear(5);
        assert_eq!(bitfield.iter().filter(|&b| b).count(), 2);

        bitfield.clear(9);
        assert_eq!(bitfield.iter().filter(|&b| b).count(), 1);
    }
}
