use crate::builders::full::*;

/// A vector full of ones
pub trait One {
    /// Return a new vector of given data type and shape,
    /// filled with ones.
    fn ones(&mut self) -> Self;
}

impl One for Vec<u8> {
    fn ones(&mut self) -> Vec<u8> {
        self.full(1)
    }
}

impl One for Vec<Vec<u8>> {
    fn ones(&mut self) -> Vec<Vec<u8>> {
        self.full(1)
    }
}

impl One for Vec<Vec<Vec<u8>>> {
    fn ones(&mut self) -> Vec<Vec<Vec<u8>>> {
        self.full(1)
    }
}

impl One for Vec<Vec<Vec<Vec<u8>>>> {
    fn ones(&mut self) -> Vec<Vec<Vec<Vec<u8>>>> {
        self.full(1)
    }
}

impl One for Vec<u16> {
    fn ones(&mut self) -> Vec<u16> {
        self.full(1)
    }
}

impl One for Vec<Vec<u16>> {
    fn ones(&mut self) -> Vec<Vec<u16>> {
        self.full(1)
    }
}

impl One for Vec<Vec<Vec<u16>>> {
    fn ones(&mut self) -> Vec<Vec<Vec<u16>>> {
        self.full(1)
    }
}

impl One for Vec<Vec<Vec<Vec<u16>>>> {
    fn ones(&mut self) -> Vec<Vec<Vec<Vec<u16>>>> {
        self.full(1)
    }
}

impl One for Vec<u32> {
    fn ones(&mut self) -> Vec<u32> {
        self.full(1)
    }
}

impl One for Vec<Vec<u32>> {
    fn ones(&mut self) -> Vec<Vec<u32>> {
        self.full(1)
    }
}

impl One for Vec<Vec<Vec<u32>>> {
    fn ones(&mut self) -> Vec<Vec<Vec<u32>>> {
        self.full(1)
    }
}

impl One for Vec<Vec<Vec<Vec<u32>>>> {
    fn ones(&mut self) -> Vec<Vec<Vec<Vec<u32>>>> {
        self.full(1)
    }
}

impl One for Vec<u64> {
    fn ones(&mut self) -> Vec<u64> {
        self.full(1)
    }
}

impl One for Vec<Vec<u64>> {
    fn ones(&mut self) -> Vec<Vec<u64>> {
        self.full(1)
    }
}

impl One for Vec<Vec<Vec<u64>>> {
    fn ones(&mut self) -> Vec<Vec<Vec<u64>>> {
        self.full(1)
    }
}

impl One for Vec<Vec<Vec<Vec<u64>>>> {
    fn ones(&mut self) -> Vec<Vec<Vec<Vec<u64>>>> {
        self.full(1)
    }
}

impl One for Vec<u128> {
    fn ones(&mut self) -> Vec<u128> {
        self.full(1)
    }
}

impl One for Vec<Vec<u128>> {
    fn ones(&mut self) -> Vec<Vec<u128>> {
        self.full(1)
    }
}

impl One for Vec<Vec<Vec<u128>>> {
    fn ones(&mut self) -> Vec<Vec<Vec<u128>>> {
        self.full(1)
    }
}

impl One for Vec<Vec<Vec<Vec<u128>>>> {
    fn ones(&mut self) -> Vec<Vec<Vec<Vec<u128>>>> {
        self.full(1)
    }
}

impl One for Vec<i8> {
    fn ones(&mut self) -> Vec<i8> {
        self.full(1)
    }
}

impl One for Vec<Vec<i8>> {
    fn ones(&mut self) -> Vec<Vec<i8>> {
        self.full(1)
    }
}

impl One for Vec<Vec<Vec<i8>>> {
    fn ones(&mut self) -> Vec<Vec<Vec<i8>>> {
        self.full(1)
    }
}

impl One for Vec<Vec<Vec<Vec<i8>>>> {
    fn ones(&mut self) -> Vec<Vec<Vec<Vec<i8>>>> {
        self.full(1)
    }
}

impl One for Vec<i16> {
    fn ones(&mut self) -> Vec<i16> {
        self.full(1)
    }
}

impl One for Vec<Vec<i16>> {
    fn ones(&mut self) -> Vec<Vec<i16>> {
        self.full(1)
    }
}

impl One for Vec<Vec<Vec<i16>>> {
    fn ones(&mut self) -> Vec<Vec<Vec<i16>>> {
        self.full(1)
    }
}

impl One for Vec<Vec<Vec<Vec<i16>>>> {
    fn ones(&mut self) -> Vec<Vec<Vec<Vec<i16>>>> {
        self.full(1)
    }
}

impl One for Vec<i32> {
    fn ones(&mut self) -> Vec<i32> {
        self.full(1)
    }
}

impl One for Vec<Vec<i32>> {
    fn ones(&mut self) -> Vec<Vec<i32>> {
        self.full(1)
    }
}

impl One for Vec<Vec<Vec<i32>>> {
    fn ones(&mut self) -> Vec<Vec<Vec<i32>>> {
        self.full(1)
    }
}

impl One for Vec<Vec<Vec<Vec<i32>>>> {
    fn ones(&mut self) -> Vec<Vec<Vec<Vec<i32>>>> {
        self.full(1)
    }
}

impl One for Vec<i64> {
    fn ones(&mut self) -> Vec<i64> {
        self.full(1)
    }
}

impl One for Vec<Vec<i64>> {
    fn ones(&mut self) -> Vec<Vec<i64>> {
        self.full(1)
    }
}

impl One for Vec<Vec<Vec<i64>>> {
    fn ones(&mut self) -> Vec<Vec<Vec<i64>>> {
        self.full(1)
    }
}

impl One for Vec<Vec<Vec<Vec<i64>>>> {
    fn ones(&mut self) -> Vec<Vec<Vec<Vec<i64>>>> {
        self.full(1)
    }
}

impl One for Vec<i128> {
    fn ones(&mut self) -> Vec<i128> {
        self.full(1)
    }
}

impl One for Vec<Vec<i128>> {
    fn ones(&mut self) -> Vec<Vec<i128>> {
        self.full(1)
    }
}

impl One for Vec<Vec<Vec<i128>>> {
    fn ones(&mut self) -> Vec<Vec<Vec<i128>>> {
        self.full(1)
    }
}

impl One for Vec<Vec<Vec<Vec<i128>>>> {
    fn ones(&mut self) -> Vec<Vec<Vec<Vec<i128>>>> {
        self.full(1)
    }
}

impl One for Vec<f32> {
    fn ones(&mut self) -> Vec<f32> {
        self.full(1.0)
    }
}

impl One for Vec<Vec<f32>> {
    fn ones(&mut self) -> Vec<Vec<f32>> {
        self.full(1.0)
    }
}

impl One for Vec<Vec<Vec<f32>>> {
    fn ones(&mut self) -> Vec<Vec<Vec<f32>>> {
        self.full(1.0)
    }
}

impl One for Vec<Vec<Vec<Vec<f32>>>> {
    fn ones(&mut self) -> Vec<Vec<Vec<Vec<f32>>>> {
        self.full(1.0)
    }
}

impl One for Vec<f64> {
    fn ones(&mut self) -> Vec<f64> {
        self.full(1.0)
    }
}

impl One for Vec<Vec<f64>> {
    fn ones(&mut self) -> Vec<Vec<f64>> {
        self.full(1.0)
    }
}

impl One for Vec<Vec<Vec<f64>>> {
    fn ones(&mut self) -> Vec<Vec<Vec<f64>>> {
        self.full(1.0)
    }
}

impl One for Vec<Vec<Vec<Vec<f64>>>> {
    fn ones(&mut self) -> Vec<Vec<Vec<Vec<f64>>>> {
        self.full(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_ones_u8_one_dim() {
        let arr: Vec<u8> = Vec::one_dim(2).ones();
        assert_eq!(arr, [1, 1]);
    }

    #[test]
    fn test_ones_u8_two_dim() {
        let arr: Vec<Vec<u8>> = Vec::two_dim(1, 2).ones();
        assert_eq!(arr, [[1, 1]]);
    }

    #[test]
    fn test_ones_u8_three_dim() {
        let arr: Vec<Vec<Vec<u8>>> = Vec::three_dim(1, 1, 2).ones();
        assert_eq!(arr, [[[1, 1]]]);
    }

    #[test]
    fn test_ones_u8_four_dim() {
        let arr: Vec<Vec<Vec<Vec<u8>>>> = Vec::four_dim(1, 1, 1, 2).ones();
        assert_eq!(arr, [[[[1, 1]]]]);
    }

    #[test]
    fn test_ones_u16_one_dim() {
        let arr: Vec<u16> = Vec::one_dim(2).ones();
        assert_eq!(arr, [1, 1]);
    }

    #[test]
    fn test_ones_u16_two_dim() {
        let arr: Vec<Vec<u16>> = Vec::two_dim(1, 2).ones();
        assert_eq!(arr, [[1, 1]]);
    }

    #[test]
    fn test_ones_u16_three_dim() {
        let arr: Vec<Vec<Vec<u16>>> = Vec::three_dim(1, 1, 2).ones();
        assert_eq!(arr, [[[1, 1]]]);
    }

    #[test]
    fn test_ones_u16_four_dim() {
        let arr: Vec<Vec<Vec<Vec<u16>>>> = Vec::four_dim(1, 1, 1, 2).ones();
        assert_eq!(arr, [[[[1, 1]]]]);
    }

    #[test]
    fn test_ones_u32_one_dim() {
        let arr: Vec<u32> = Vec::one_dim(2).ones();
        assert_eq!(arr, [1, 1]);
    }

    #[test]
    fn test_ones_u32_two_dim() {
        let arr: Vec<Vec<u32>> = Vec::two_dim(1, 2).ones();
        assert_eq!(arr, [[1, 1]]);
    }

    #[test]
    fn test_ones_u32_three_dim() {
        let arr: Vec<Vec<Vec<u32>>> = Vec::three_dim(1, 1, 2).ones();
        assert_eq!(arr, [[[1, 1]]]);
    }

    #[test]
    fn test_ones_u32_four_dim() {
        let arr: Vec<Vec<Vec<Vec<u32>>>> = Vec::four_dim(1, 1, 1, 2).ones();
        assert_eq!(arr, [[[[1, 1]]]]);
    }

    #[test]
    fn test_ones_u64_one_dim() {
        let arr: Vec<u64> = Vec::one_dim(2).ones();
        assert_eq!(arr, [1, 1]);
    }

    #[test]
    fn test_ones_u64_two_dim() {
        let arr: Vec<Vec<u64>> = Vec::two_dim(1, 2).ones();
        assert_eq!(arr, [[1, 1]]);
    }

    #[test]
    fn test_ones_u64_three_dim() {
        let arr: Vec<Vec<Vec<u64>>> = Vec::three_dim(1, 1, 2).ones();
        assert_eq!(arr, [[[1, 1]]]);
    }

    #[test]
    fn test_ones_u64_four_dim() {
        let arr: Vec<Vec<Vec<Vec<u64>>>> = Vec::four_dim(1, 1, 1, 2).ones();
        assert_eq!(arr, [[[[1, 1]]]]);
    }

    #[test]
    fn test_ones_u128_one_dim() {
        let arr: Vec<u128> = Vec::one_dim(2).ones();
        assert_eq!(arr, [1, 1]);
    }

    #[test]
    fn test_ones_u128_two_dim() {
        let arr: Vec<Vec<u128>> = Vec::two_dim(1, 2).ones();
        assert_eq!(arr, [[1, 1]]);
    }

    #[test]
    fn test_ones_u128_three_dim() {
        let arr: Vec<Vec<Vec<u128>>> = Vec::three_dim(1, 1, 2).ones();
        assert_eq!(arr, [[[1, 1]]]);
    }

    #[test]
    fn test_ones_u128_four_dim() {
        let arr: Vec<Vec<Vec<Vec<u128>>>> = Vec::four_dim(1, 1, 1, 2).ones();
        assert_eq!(arr, [[[[1, 1]]]]);
    }

    #[test]
    fn test_ones_i8_one_dim() {
        let arr: Vec<i8> = Vec::one_dim(2).ones();
        assert_eq!(arr, [1, 1]);
    }

    #[test]
    fn test_ones_i8_two_dim() {
        let arr: Vec<Vec<i8>> = Vec::two_dim(1, 2).ones();
        assert_eq!(arr, [[1, 1]]);
    }

    #[test]
    fn test_ones_i8_three_dim() {
        let arr: Vec<Vec<Vec<i8>>> = Vec::three_dim(1, 1, 2).ones();
        assert_eq!(arr, [[[1, 1]]]);
    }

    #[test]
    fn test_ones_i8_four_dim() {
        let arr: Vec<Vec<Vec<Vec<i8>>>> = Vec::four_dim(1, 1, 1, 2).ones();
        assert_eq!(arr, [[[[1, 1]]]]);
    }

    #[test]
    fn test_ones_i16_one_dim() {
        let arr: Vec<i16> = Vec::one_dim(2).ones();
        assert_eq!(arr, [1, 1]);
    }

    #[test]
    fn test_ones_i16_two_dim() {
        let arr: Vec<Vec<i16>> = Vec::two_dim(1, 2).ones();
        assert_eq!(arr, [[1, 1]]);
    }

    #[test]
    fn test_ones_i16_three_dim() {
        let arr: Vec<Vec<Vec<i16>>> = Vec::three_dim(1, 1, 2).ones();
        assert_eq!(arr, [[[1, 1]]]);
    }

    #[test]
    fn test_ones_i16_four_dim() {
        let arr: Vec<Vec<Vec<Vec<i16>>>> = Vec::four_dim(1, 1, 1, 2).ones();
        assert_eq!(arr, [[[[1, 1]]]]);
    }

    #[test]
    fn test_ones_i32_one_dim() {
        let arr: Vec<i32> = Vec::one_dim(2).ones();
        assert_eq!(arr, [1, 1]);
    }

    #[test]
    fn test_ones_i32_two_dim() {
        let arr: Vec<Vec<i32>> = Vec::two_dim(1, 2).ones();
        assert_eq!(arr, [[1, 1]]);
    }

    #[test]
    fn test_ones_i32_three_dim() {
        let arr: Vec<Vec<Vec<i32>>> = Vec::three_dim(1, 1, 2).ones();
        assert_eq!(arr, [[[1, 1]]]);
    }

    #[test]
    fn test_ones_i32_four_dim() {
        let arr: Vec<Vec<Vec<Vec<i32>>>> = Vec::four_dim(1, 1, 1, 2).ones();
        assert_eq!(arr, [[[[1, 1]]]]);
    }

    #[test]
    fn test_ones_i64_one_dim() {
        let arr: Vec<i64> = Vec::one_dim(2).ones();
        assert_eq!(arr, [1, 1]);
    }

    #[test]
    fn test_ones_i64_two_dim() {
        let arr: Vec<Vec<i64>> = Vec::two_dim(1, 2).ones();
        assert_eq!(arr, [[1, 1]]);
    }

    #[test]
    fn test_ones_i64_three_dim() {
        let arr: Vec<Vec<Vec<i64>>> = Vec::three_dim(1, 1, 2).ones();
        assert_eq!(arr, [[[1, 1]]]);
    }

    #[test]
    fn test_ones_i64_four_dim() {
        let arr: Vec<Vec<Vec<Vec<i64>>>> = Vec::four_dim(1, 1, 1, 2).ones();
        assert_eq!(arr, [[[[1, 1]]]]);
    }

    #[test]
    fn test_ones_i128_one_dim() {
        let arr: Vec<i128> = Vec::one_dim(2).ones();
        assert_eq!(arr, [1, 1]);
    }

    #[test]
    fn test_ones_i128_two_dim() {
        let arr: Vec<Vec<i128>> = Vec::two_dim(1, 2).ones();
        assert_eq!(arr, [[1, 1]]);
    }

    #[test]
    fn test_ones_i128_three_dim() {
        let arr: Vec<Vec<Vec<i128>>> = Vec::three_dim(1, 1, 2).ones();
        assert_eq!(arr, [[[1, 1]]]);
    }

    #[test]
    fn test_ones_i128_four_dim() {
        let arr: Vec<Vec<Vec<Vec<i128>>>> = Vec::four_dim(1, 1, 1, 2).ones();
        assert_eq!(arr, [[[[1, 1]]]]);
    }

    #[test]
    fn test_ones_f32_one_dim() {
        let arr: Vec<f32> = Vec::one_dim(2).ones();
        assert_eq!(arr, [1.0, 1.0]);
    }

    #[test]
    fn test_ones_f32_two_dim() {
        let arr: Vec<Vec<f32>> = Vec::two_dim(1, 2).ones();
        assert_eq!(arr, [[1.0, 1.0]]);
    }

    #[test]
    fn test_ones_f32_three_dim() {
        let arr: Vec<Vec<Vec<f32>>> = Vec::three_dim(1, 1, 2).ones();
        assert_eq!(arr, [[[1.0, 1.0]]]);
    }

    #[test]
    fn test_ones_f32_four_dim() {
        let arr: Vec<Vec<Vec<Vec<f32>>>> = Vec::four_dim(1, 1, 1, 2).ones();
        assert_eq!(arr, [[[[1.0, 1.0]]]]);
    }

    #[test]
    fn test_ones_f64_one_dim() {
        let arr: Vec<f64> = Vec::one_dim(2).ones();
        assert_eq!(arr, [1.0, 1.0]);
    }

    #[test]
    fn test_ones_f64_two_dim() {
        let arr: Vec<Vec<f64>> = Vec::two_dim(1, 2).ones();
        assert_eq!(arr, [[1.0, 1.0]]);
    }

    #[test]
    fn test_ones_f64_three_dim() {
        let arr: Vec<Vec<Vec<f64>>> = Vec::three_dim(1, 1, 2).ones();
        assert_eq!(arr, [[[1.0, 1.0]]]);
    }

    #[test]
    fn test_ones_f64_four_dim() {
        let arr: Vec<Vec<Vec<Vec<f64>>>> = Vec::four_dim(1, 1, 1, 2).ones();
        assert_eq!(arr, [[[[1.0, 1.0]]]]);
    }
}
