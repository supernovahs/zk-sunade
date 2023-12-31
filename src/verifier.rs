use crate::constants::{ConstantParams, Constants};
use crate::groth_16::{G1Point, G2Point, Groth16};
use alloy_primitives::U256;
use alloy_sol_types::sol;
use stylus_sdk::prelude::*;
sol_storage! {
    #[entrypoint]
    pub struct Verifier {}
}

sol! {
    struct VerifyingKey {
        G1Point alfa1;
        G2Point beta2;
        G2Point gamma2;
        G2Point delta2;
        G1Point[7] IC;
    }

    struct Proof {
        G1Point A;
        G2Point B;
        G1Point C;
    }
}

#[external]
impl Verifier {
    #[allow(non_snake_case)]
    pub fn verifyProof(words: [U256; 14]) -> Result<bool, Vec<u8>> {
        let proof: [U256; 8] = words[0..8].try_into().unwrap();
        let input: [U256; 6] = words[8..14].try_into().unwrap();

        let mut i = 0;
        while i < 8 {
            if proof[i] >= Constants.PRIME_Q() {
                return Err("first verify".into());
            }
            i += 1;
        }

        let proof = Proof {
            A: G1Point {
                X: proof[0],
                Y: proof[1],
            },
            B: G2Point {
                X: [proof[2], proof[3]],
                Y: [proof[4], proof[5]],
            },
            C: G1Point {
                X: proof[6],
                Y: proof[7],
            },
        };

        let verifying_key = Verifier::verifyingKey()?;

        let vk_x = G1Point {
            X: U256::from(0),
            Y: U256::from(0),
        };
        let mut vk_x = Groth16::plus(&vk_x, &verifying_key.IC[0])?;

        #[allow(clippy::needless_range_loop)]
        for z in 0..6 {
            if input[z] >= Constants.SNARK_SCALAR_FIELD() {
                return Err("sunade".into());
            }
            let scalarmul = Groth16::scalar_mul(&verifying_key.IC[z + 1], input[z])?;
            let val2 = Groth16::plus(&vk_x, &scalarmul)?;
            vk_x = val2;
        }

        Groth16::pairing(
            Groth16::negate(proof.A),
            proof.B,
            verifying_key.alfa1,
            verifying_key.beta2,
            vk_x,
            verifying_key.gamma2,
            proof.C,
            verifying_key.delta2,
        )
    }
}

impl Verifier {
    #[allow(non_snake_case)]
    pub fn verifyingKey() -> Result<VerifyingKey, Vec<u8>> {
        let alfa1 = G1Point {
            X: U256::from_be_bytes([
                45, 191, 195, 236, 98, 163, 238, 229, 163, 180, 180, 100, 188, 241, 248, 82, 123,
                188, 161, 42, 222, 160, 241, 241, 32, 51, 205, 79, 97, 176, 224, 145,
            ]),
            Y: U256::from_be_bytes([
                25, 229, 91, 208, 183, 44, 18, 109, 161, 134, 101, 3, 149, 86, 119, 102, 66, 255,
                130, 226, 243, 71, 242, 79, 206, 162, 71, 95, 77, 176, 135, 223,
            ]),
        };
        let beta2 = G2Point {
            X: [
                U256::from_be_bytes([
                    26, 231, 36, 171, 19, 78, 90, 124, 107, 216, 161, 22, 250, 85, 5, 178, 89, 82,
                    44, 15, 22, 74, 94, 129, 38, 227, 236, 125, 52, 70, 95, 110,
                ]),
                U256::from_be_bytes([
                    0, 159, 27, 205, 200, 83, 248, 227, 83, 23, 86, 187, 98, 91, 13, 29, 192, 20,
                    244, 171, 87, 195, 247, 159, 79, 78, 46, 126, 247, 224, 234, 214,
                ]),
            ],
            Y: [
                U256::from_be_bytes([
                    35, 168, 202, 87, 96, 69, 126, 114, 99, 101, 185, 47, 208, 206, 180, 134, 102,
                    87, 151, 205, 104, 195, 93, 207, 253, 142, 74, 232, 6, 102, 145, 233,
                ]),
                U256::from_be_bytes([
                    19, 236, 113, 130, 201, 253, 104, 51, 26, 16, 248, 190, 15, 232, 133, 215, 48,
                    222, 92, 127, 137, 170, 125, 11, 123, 175, 170, 0, 155, 188, 158, 62,
                ]),
            ],
        };
        let gamma2 = G2Point {
            X: [
                U256::from_be_bytes([
                    25, 142, 147, 147, 146, 13, 72, 58, 114, 96, 191, 183, 49, 251, 93, 37, 241,
                    170, 73, 51, 53, 169, 231, 18, 151, 228, 133, 183, 174, 243, 18, 194,
                ]),
                U256::from_be_bytes([
                    24, 0, 222, 239, 18, 31, 30, 118, 66, 106, 0, 102, 94, 92, 68, 121, 103, 67,
                    34, 212, 247, 94, 218, 221, 70, 222, 189, 92, 217, 146, 246, 237,
                ]),
            ],
            Y: [
                U256::from_be_bytes([
                    9, 6, 137, 208, 88, 95, 240, 117, 236, 158, 153, 173, 105, 12, 51, 149, 188,
                    75, 49, 51, 112, 179, 142, 243, 85, 172, 218, 220, 209, 34, 151, 91,
                ]),
                U256::from_be_bytes([
                    18, 200, 94, 165, 219, 140, 109, 235, 74, 171, 113, 128, 141, 203, 64, 143,
                    227, 209, 231, 105, 12, 67, 211, 123, 76, 230, 204, 1, 102, 250, 125, 170,
                ]),
            ],
        };

        let delta2 = G2Point {
            X: [
                U256::from_be_bytes([
                    47, 12, 99, 208, 197, 59, 61, 251, 202, 39, 182, 180, 58, 231, 251, 245, 90,
                    56, 215, 138, 33, 71, 9, 150, 72, 91, 3, 18, 138, 204, 194, 8,
                ]),
                U256::from_be_bytes([
                    0, 85, 101, 2, 53, 110, 55, 237, 21, 13, 178, 227, 101, 49, 176, 242, 117, 253,
                    104, 53, 192, 252, 25, 69, 146, 46, 39, 11, 72, 196, 138, 134,
                ]),
            ],
            Y: [
                U256::from_be_bytes([
                    2, 100, 76, 39, 181, 219, 215, 147, 89, 42, 112, 183, 53, 226, 44, 121, 138,
                    94, 48, 159, 161, 122, 153, 42, 125, 194, 160, 80, 224, 27, 41, 143,
                ]),
                U256::from_be_bytes([
                    25, 71, 118, 182, 165, 52, 57, 215, 51, 111, 56, 157, 42, 143, 102, 81, 228, 8,
                    133, 245, 202, 37, 56, 176, 220, 156, 181, 52, 251, 35, 247, 250,
                ]),
            ],
        };

        let ic = [
            G1Point {
                X: U256::from_be_bytes([
                    35, 223, 27, 201, 22, 94, 156, 28, 155, 43, 192, 54, 216, 235, 221, 16, 231,
                    174, 174, 126, 94, 128, 25, 253, 230, 138, 236, 124, 129, 139, 178, 62,
                ]),
                Y: U256::from_be_bytes([
                    11, 108, 146, 8, 13, 55, 197, 251, 45, 223, 48, 137, 42, 51, 102, 94, 84, 120,
                    67, 46, 243, 247, 26, 200, 118, 142, 203, 190, 98, 199, 120, 146,
                ]),
            },
            G1Point {
                X: U256::from_be_bytes([
                    28, 123, 42, 223, 69, 224, 70, 202, 234, 0, 9, 86, 178, 236, 178, 216, 97, 126,
                    113, 13, 42, 123, 178, 1, 169, 94, 162, 118, 249, 35, 7, 180,
                ]),
                Y: U256::from_be_bytes([
                    43, 21, 240, 117, 54, 244, 89, 72, 207, 74, 190, 101, 150, 99, 125, 144, 47,
                    250, 187, 24, 200, 194, 245, 193, 81, 84, 76, 41, 76, 228, 166, 114,
                ]),
            },
            G1Point {
                X: U256::from_be_bytes([
                    28, 236, 254, 146, 136, 42, 140, 131, 90, 71, 191, 1, 191, 166, 85, 207, 98,
                    140, 187, 167, 248, 28, 244, 4, 33, 121, 253, 19, 237, 205, 106, 57,
                ]),
                Y: U256::from_be_bytes([
                    1, 84, 191, 187, 44, 183, 134, 202, 36, 125, 75, 105, 24, 61, 23, 81, 242, 103,
                    187, 199, 101, 107, 232, 208, 240, 231, 165, 164, 126, 44, 17, 1,
                ]),
            },
            G1Point {
                X: U256::from_be_bytes([
                    21, 132, 97, 106, 116, 35, 239, 204, 114, 246, 158, 168, 79, 160, 178, 188, 1,
                    67, 54, 119, 41, 127, 78, 131, 81, 190, 191, 193, 91, 205, 12, 218,
                ]),
                Y: U256::from_be_bytes([
                    6, 35, 117, 91, 20, 136, 82, 109, 170, 159, 236, 240, 225, 27, 17, 13, 214,
                    223, 18, 196, 97, 87, 157, 121, 46, 29, 182, 90, 245, 35, 200, 190,
                ]),
            },
            G1Point {
                X: U256::from_be_bytes([
                    18, 251, 181, 191, 202, 157, 97, 53, 123, 162, 214, 65, 96, 76, 244, 133, 46,
                    33, 239, 84, 250, 161, 128, 254, 83, 156, 24, 153, 77, 193, 218, 90,
                ]),
                Y: U256::from_be_bytes([
                    47, 9, 221, 153, 114, 161, 175, 95, 123, 207, 204, 243, 215, 171, 96, 12, 157,
                    137, 142, 166, 214, 147, 49, 80, 186, 10, 226, 40, 236, 225, 126, 95,
                ]),
            },
            G1Point {
                X: U256::from_be_bytes([
                    10, 219, 81, 55, 150, 253, 242, 16, 48, 34, 198, 65, 81, 206, 5, 247, 199, 166,
                    217, 32, 14, 141, 129, 159, 165, 158, 101, 79, 196, 191, 232, 60,
                ]),
                Y: U256::from_be_bytes([
                    45, 100, 247, 46, 244, 237, 223, 156, 160, 50, 5, 142, 210, 191, 105, 23, 88,
                    56, 126, 145, 58, 119, 207, 153, 214, 163, 207, 179, 124, 139, 167, 238,
                ]),
            },
            G1Point {
                X: U256::from_be_bytes([
                    33, 231, 201, 191, 253, 167, 75, 253, 44, 67, 147, 182, 128, 61, 119, 85, 69,
                    222, 111, 168, 145, 69, 244, 162, 52, 118, 36, 29, 152, 129, 182, 97,
                ]),
                Y: U256::from_be_bytes([
                    11, 190, 65, 229, 34, 55, 172, 19, 235, 123, 1, 243, 203, 153, 155, 115, 148,
                    208, 135, 52, 231, 27, 28, 58, 218, 98, 113, 62, 23, 235, 86, 12,
                ]),
            },
        ];
        Ok(VerifyingKey {
            alfa1,
            beta2,
            gamma2,
            delta2,
            IC: ic,
        })
    }
}
