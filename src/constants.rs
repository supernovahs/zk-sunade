use alloy_primitives::U256;

pub trait ConstantParams {
    #[allow(non_snake_case)]
    fn PRIME_Q(self) -> U256;
    #[allow(non_snake_case)]
    fn SNARK_SCALAR_FIELD(self) -> U256;
}

pub struct Constants;

impl ConstantParams for Constants {
    #[allow(non_snake_case)]
    fn PRIME_Q(self) -> U256 {
        U256::from_be_bytes([
            48, 100, 78, 114, 225, 49, 160, 41, 184, 80, 69, 182, 129, 129, 88, 93, 151, 129, 106,
            145, 104, 113, 202, 141, 60, 32, 140, 22, 216, 124, 253, 71,
        ])
    }

    #[allow(non_snake_case)]
    fn SNARK_SCALAR_FIELD(self) -> U256 {
        U256::from_be_bytes([
            48, 100, 78, 114, 225, 49, 160, 41, 184, 80, 69, 182, 129, 129, 88, 93, 40, 51, 232,
            72, 121, 185, 112, 145, 67, 225, 245, 147, 240, 0, 0, 1,
        ])
    }
}
