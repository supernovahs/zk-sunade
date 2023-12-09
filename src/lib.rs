//! Implements a hello-world example for Arbitrum Stylus, providing a Solidity ABI-equivalent
//! Rust implementation of the Counter contract example provided by Foundry.
//! Warning: this code is a template only and has not been audited.
//! ```
//! contract Counter {
//!     uint256 public number;
//!     function setNumber(uint256 newNumber) public {
//!         number = newNumber;
//!     }
//!     function increment() public {
//!         number++;
//!     }
//! }
//! ```

// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Initializes a custom, global allocator for Rust programs compiled to WASM.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
use alloy_sol_types::sol;
use stylus_sdk::abi::AbiType;
use stylus_sdk::call::RawCall;
/// Import the Stylus SDK along with alloy primitive types for use in our program.
use stylus_sdk::{
    alloy_primitives::{address, Bytes, FixedBytes, U256},
    prelude::*,
};

// Define the entrypoint as a Solidity storage object, in this case a struct
// called `Counter` with a single uint256 value called `number`. The sol_storage! macro
// will generate Rust-equivalent structs with all fields mapped to Solidity-equivalent
// storage slots and types.
sol_storage! {
    #[entrypoint]
    pub struct Verifier {}

    pub struct Groth16 {}
}

pub trait ConstantParams {
    fn PRIME_Q(self) -> U256;
    fn SNARK_SCALAR_FIELD(self) -> U256;
}

sol! {
    struct G1Point {
        uint256 X;
        uint256 Y;
    }

    // Encoding of field elements is: X[0] * z + X[1]
    struct G2Point {
        uint256[2] X;
        uint256[2] Y;
    }
}

// impl  AbiType for G1Point{

// }

struct Constants;

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

#[external]
impl Verifier {
    fn verifyProof() -> Result<(), Vec<u8>> {
        Ok(todo!())
    }
}

// let primt_q:U256 ="21888242871839275222246405745257275088696311157297823662689037894645226208583".parse().unwrap();
/// Define an implementation of the generated Counter struct, defining a set_number
/// and increment method using the features of the Stylus SDK.
impl Groth16 {
    pub fn negate(p: G1Point) -> G1Point {
        if p.X == U256::ZERO && p.Y == U256::ZERO {
            G1Point {
                X: U256::ZERO,
                Y: U256::ZERO,
            }
        } else {
            G1Point {
                X: p.X,
                Y: Constants.PRIME_Q() - (p.Y % Constants.PRIME_Q()),
            }
        }
    }

    fn plus(p1: G1Point, p2: G1Point) -> Result<G1Point, Vec<u8>> {
        let calldata = [p1.X, p1.Y, p2.X, p2.Y]
            .map(|i| i.to_be_bytes::<32>())
            .concat();
        let call_result = RawCall::new_static().gas(u64::MAX).call(
            address!("0000000000000000000000000000000000000006"),
            &calldata,
        );
        if call_result.is_err() {
            return Err(call_result.err().unwrap());
        }
        let returndata = call_result.unwrap();
        Ok(G1Point {
            X: U256::from_be_bytes::<32>(returndata[0..32].try_into().unwrap()),
            Y: U256::from_be_bytes::<32>(returndata[32..64].try_into().unwrap()),
        })
    }

    fn scalar_mul(p1: G1Point, s: U256) -> Result<G1Point, Vec<u8>> {
        let calldata = [p1.X, p1.Y, s].map(|i| i.to_be_bytes::<32>()).concat();
        // let calldata = ;
        let call_result = RawCall::new_static().gas(u64::MAX).call(
            address!("0000000000000000000000000000000000000007"),
            &calldata,
        );

        if call_result.is_err() {
            return Err(call_result.err().unwrap());
        }

        let returndata = call_result.unwrap();
        Ok(G1Point {
            X: U256::from_be_bytes::<32>(returndata[0..32].try_into().unwrap()),
            Y: U256::from_be_bytes::<32>(returndata[32..64].try_into().unwrap()),
        })
    }

    // function scalar_mul(G1Point memory p, uint256 s) internal view returns (G1Point memory r) {
    //     uint256[3] memory input;
    //     input[0] = p.X;
    //     input[1] = p.Y;
    //     input[2] = s;
    //     bool success;
    //     // solium-disable-next-line security/no-inline-assembly
    //     assembly {
    //         success := staticcall(sub(gas(), 2000), 7, input, 0x80, r, 0x60)
    //         // Use "invalid" to make gas estimation work
    //         switch success case 0 { invalid() }
    //     }
    //     require(success, "pairing-mul-failed");
    // }
    // function negate(G1Point memory p) internal pure returns (G1Point memory) {
    //     // The prime q in the base field F_q for G1
    //     if (p.X == 0 && p.Y == 0) {
    //         return G1Point(0, 0);
    //     } else {
    //         return G1Point(p.X, PRIME_Q - (p.Y % PRIME_Q));
    //     }
    // }

    // /// Gets the number from storage.
    // pub fn number(&self) -> Result<U256, Vec<u8>> {
    //     Ok(self.number.get())
    // }

    // /// Sets a number in storage to a user-specified value.
    // pub fn set_number(&mut self, new_number: U256) -> Result<(), Vec<u8>> {
    //     self.number.set(new_number);
    //     Ok(())
    // }

    // /// Increments number and updates it values in storage.
    // pub fn increment(&mut self) -> Result<(), Vec<u8>> {
    //     let number = self.number.get();
    //     self.set_number(number + U256::from(1))
    // }

    #[allow(clippy::too_many_arguments)]
    fn pairing(
        a1: G1Point,
        a2: G2Point,
        b1: G1Point,
        b2: G2Point,
        c1: G1Point,
        c2: G2Point,
        d1: G1Point,
        d2: G2Point,
    ) -> Result<bool, Vec<u8>> {
        let p1 = [a1, b1, c1, d1];
        let p2 = [a2, b2, c2, d2];

        let mut input = [U256::ZERO; 24];

        for i in 0..4 {
            let j = i * 6;
            input[j] = p1[i].X;
            input[j + 1] = p1[i].Y;
            input[j + 2] = p2[i].X[0];
            input[j + 3] = p2[i].X[1];
            input[j + 4] = p2[i].Y[0];
            input[j + 5] = p2[i].Y[1];
        }

        let calldata = input.map(|i| i.to_be_bytes::<32>()).concat();
        let call_result = RawCall::new_static().gas(u64::MAX).call(
            address!("0000000000000000000000000000000000000006"),
            &calldata,
        );
        if call_result.is_err() {
            return Err("pairing-opcode-failed".as_bytes().to_vec());
        }
        let returndata = call_result.unwrap();
        let len = U256::from_be_bytes::<32>(returndata[0..32].try_into().unwrap());
        Ok(len != U256::from(0))
    }
}
