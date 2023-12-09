use alloy_primitives::U256;

pub trait ConstantParams {
    fn PRIME_Q(self) -> U256;
    fn SNARK_SCALAR_FIELD(self) -> U256;
}

pub struct Constants;

impl ConstantParams for Constants {
    fn PRIME_Q(self) -> U256 {
        "21888242871839275222246405745257275088696311157297823662689037894645226208583"
            .parse()
            .unwrap()
    }

    fn SNARK_SCALAR_FIELD(self) -> U256 {
        "21888242871839275222246405745257275088548364400416034343698204186575808495617"
            .parse()
            .unwrap()
    }
}
