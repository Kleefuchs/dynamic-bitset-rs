
#[derive(Default, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DynamicBitset<const SIZE: usize> {
    data: std::vec::Vec<u32>
}

impl<const SIZE: usize> DynamicBitset<SIZE> {
    pub fn reset_array(&mut self, min_array_index: usize, max_array_index: usize) {
        for i in min_array_index..max_array_index {
            self.data[i] = 0;
        }
    }

    #[must_use]
    pub fn new() -> Self {
        let mut result = Self {
            data: vec![0; SIZE]
        };
        result.reset_array(0, SIZE - 1);
        result
    }

    #[must_use]
    fn is_valid_inner_index(&self, inner_index: usize) -> bool {
        inner_index > 31
    }

    #[must_use]
    fn is_valid_array_index(&self, array_index: usize) -> bool {
        array_index >= self.data.len()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len() * 32
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[must_use]
    pub fn data_len(&self) -> usize {
        self.data.len()
    }

    fn get_from_array_and_inner_index(&self, array_index: usize, inner_index: usize) -> Result<bool, String> {
        if self.is_valid_array_index(array_index) && self.is_valid_inner_index(inner_index) {
            return Err("Index is out of bounds".to_string());
        }
        Ok((self.data[array_index] << inner_index).is_power_of_two())

    }

    pub fn get(&self, index: usize) -> Result<bool, String> {
        let quotient = index / 32;
        let remainder = index % 32;

        self.get_from_array_and_inner_index(quotient, remainder)
    }

    fn set_from_array_and_inner_index(&mut self, array_index: usize, inner_index: usize, state: bool) -> Result<(), String> {
        if self.is_valid_array_index(array_index) && self.is_valid_inner_index(inner_index) {
            return Err("Index is out of bounds".to_string());
        }

        self.data[array_index] = (self.data[array_index] << inner_index) | (state as u32);

        Ok(())
    }

    pub fn set(&mut self, index: usize, state: bool) -> Result<(), String> {
        let quotient = index / 32;
        let remainder = index % 32;

        self.set_from_array_and_inner_index(quotient, remainder, state)
    }

    pub fn resize(&mut self, new_len: usize) -> Result<(), String> {
        match new_len < self.data.len() {
            true => {
                Err("Provided length is shorter than current length".to_string())
            },
            false => {
                self.data.resize(new_len, 0);
                Ok(())
            },
        }
    }

    pub fn push(&mut self, value: u32) {
        self.data.push(value);
    }

    /*
     * Be careful with this function as it will pop your last 32 bits.
     * */
    pub fn pop(&mut self) {
        self.data.pop();
    }
}

#[cfg(test)]
mod tests {
    use crate::dynamic_bitset;

    #[test]
    fn test_set_and_get() {
        let mut dynamic_bitset: dynamic_bitset::DynamicBitset<2> = dynamic_bitset::DynamicBitset::new();
        dynamic_bitset.set(5, true).expect("Out of bounds in testing set");
        assert_eq!(dynamic_bitset.get(5), Ok(true));
        dynamic_bitset.set(37, true).expect("Out of bounds in testing set");
        assert_eq!(dynamic_bitset.get(37), Ok(true));
    }

    #[test]
    fn test_resize() {
        let mut dynamic_bitset: dynamic_bitset::DynamicBitset<1> = dynamic_bitset::DynamicBitset::new();
        dynamic_bitset.set(5, true).expect("Out of bounds in testing set");

        assert_eq!(dynamic_bitset.resize(3), Ok(()));
        assert_eq!(dynamic_bitset.data_len(), 3);
        assert_eq!(dynamic_bitset.get(5), Ok(true));
        assert_eq!(dynamic_bitset.get(85), Ok(false));
        dynamic_bitset.set(85, true).expect("Out of bounds in testing resize with setting");
        assert_eq!(dynamic_bitset.get(85), Ok(true));
    }
}
