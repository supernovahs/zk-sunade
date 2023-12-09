use crate::constants::ConstantParams;
use crate::groth_16::{self, G1Point, G2Point, Groth16};
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
    pub fn verifyProof(proof: Vec<u8>, input: [U256; 6]) -> Result<bool, Vec<u8>> {
        let mut i = 0;
        let mut next = 0x00;
        while i < 8 {
            if U256::from_be_bytes::<32>(proof[next..(next + 0x20)].try_into().unwrap())
                >= groth_16::Constants.PRIME_Q()
            {
                return Err("v".into());
            }
            next = next + 0x20;
            i += 1;
        }

        let proof = Proof {
            A: G1Point {
                X: U256::from_be_bytes::<32>(proof[0..0x20].try_into().unwrap()),
                Y: U256::from_be_bytes::<32>(proof[0x20..0x40].try_into().unwrap()),
            },
            B: G2Point {
                X: [
                    U256::from_be_bytes::<32>(proof[0x40..0x60].try_into().unwrap()),
                    U256::from_be_bytes::<32>(proof[0x60..0x80].try_into().unwrap()),
                ],
                Y: [
                    U256::from_be_bytes::<32>(proof[0x80..0xA0].try_into().unwrap()),
                    U256::from_be_bytes::<32>(proof[0xA0..0xC0].try_into().unwrap()),
                ],
            },
            C: G1Point {
                X: U256::from_be_bytes::<32>(proof[0xC0..0xE0].try_into().unwrap()),
                Y: U256::from_be_bytes::<32>(proof[0xE0..0x100].try_into().unwrap()),
            },
        };

        let verifying_key = Verifier::verifyingKey();
        let mut vk_x = G1Point {
            X: U256::from(0),
            Y: U256::from(0),
        };
        if let Ok(key) = verifying_key {
            let res = Groth16::plus(&vk_x, &key.IC[0]);
            if let Ok(val) = res {
                vk_x = val
            }

            for z in 0..6 {
                if input[z] < groth_16::Constants.SNARK_SCALAR_FIELD() {
                    return Err("v".into());
                }
                if let Ok(scalarmul) = Groth16::scalar_mul(&key.IC[z + 1], input[z]) {
                    if let Ok(val2) = Groth16::plus(&vk_x, &scalarmul) {
                        vk_x = val2;
                        return Groth16::pairing(
                            Groth16::negate(proof.A),
                            proof.B,
                            key.alfa1,
                            key.beta2,
                            vk_x,
                            key.gamma2,
                            proof.C,
                            key.delta2,
                        );
                    }
                }
            }
            Ok(false)
        } else {
            return Err("C".into());
        }

        // return
        // Pairing.pairing(
        //     Pairing.negate(_proof.A),
        //     _proof.B,
        //     vk.alfa1,
        //     vk.beta2,
        //     vk_x,
        //     vk.gamma2,
        //     _proof.C,
        //     vk.delta2
        // );
        // vk_x = Pairing.plus(
        //     vk_x,
        //     Pairing.scalar_mul(vk.IC[i + 1], input[i])
        // );
        // for (uint256 i = 0; i < input.length; i++) {
        //     require(
        //         input[i] < SNARK_SCALAR_FIELD,
        //         "verifier-gte-snark-scalar-field"
        //     );
        //     vk_x = Pairing.plus(
        //         vk_x,
        //         Pairing.scalar_mul(vk.IC[i + 1], input[i])
        //     );
        // }
        // let p = [
        //     U256::from_be_bytes::<32>(proof[0..0x20].try_into().unwrap()),
        //     U256::from_be_bytes::<32>(proof[0x20..0x40].try_into().unwrap()),
        //     U256::from_be_bytes::<32>(proof[0x40..0x60].try_into().unwrap()),
        //     U256::from_be_bytes::<32>(proof[0x60..0x80].try_into().unwrap()),
        //     U256::from_be_bytes::<32>(proof[0x80..0xA0].try_into().unwrap()),
        //     U256::from_be_bytes::<32>(proof[0xA0..0xC0].try_into().unwrap()),
        //     U256::from_be_bytes::<32>(proof[0xC0..0xE0].try_into().unwrap()),
        //     U256::from_be_bytes::<32>(proof[0xE0..0x100].try_into().unwrap()),
        // ];
    }
}

impl Verifier {
    #[allow(non_snake_case)]
    pub fn verifyingKey() -> Result<VerifyingKey, Vec<u8>> {
        let alfa1 = G1Point {
            X: "20692898189092739278193869274495556617788530808486270118371701516666252877969"
                .parse()
                .unwrap(),
            Y: "11713062878292653967971378194351968039596396853904572879488166084231740557279"
                .parse()
                .unwrap(),
        };
        let beta2 = G2Point {
            X: [
                "12168528810181263706895252315640534818222943348193302139358377162645029937006"
                    .parse()
                    .unwrap(),
                "281120578337195720357474965979947690431622127986816839208576358024608803542"
                    .parse()
                    .unwrap(),
            ],
            Y: [
                "16129176515713072042442734839012966563817890688785805090011011570989315559913"
                    .parse()
                    .unwrap(),
                "9011703453772030375124466642203641636825223906145908770308724549646909480510"
                    .parse()
                    .unwrap(),
            ],
        };
        let gamma2 = G2Point {
            X: [
                "11559732032986387107991004021392285783925812861821192530917403151452391805634"
                    .parse()
                    .unwrap(),
                "10857046999023057135944570762232829481370756359578518086990519993285655852781"
                    .parse()
                    .unwrap(),
            ],
            Y: [
                "4082367875863433681332203403145435568316851327593401208105741076214120093531"
                    .parse()
                    .unwrap(),
                "8495653923123431417604973247489272438418190587263600148770280649306958101930"
                    .parse()
                    .unwrap(),
            ],
        };

        let delta2 = G2Point {
            X: [
                "21280594949518992153305586783242820682644996932183186320680800072133486887432"
                    .parse()
                    .unwrap(),
                "150879136433974552800030963899771162647715069685890547489132178314736470662"
                    .parse()
                    .unwrap(),
            ],
            Y: [
                "1081836006956609894549771334721413187913047383331561601606260283167615953295"
                    .parse()
                    .unwrap(),
                "11434086686358152335540554643130007307617078324975981257823476472104616196090"
                    .parse()
                    .unwrap(),
            ],
        };

        let ic = [
            G1Point {
                X: "16225148364316337376768119297456868908427925829817748684139175309620217098814"
                    .parse()
                    .unwrap(),
                Y: "5167268689450204162046084442581051565997733233062478317813755636162413164690"
                    .parse()
                    .unwrap(),
            },
            G1Point {
                X: "12882377842072682264979317445365303375159828272423495088911985689463022094260"
                    .parse()
                    .unwrap(),
                Y: "19488215856665173565526758360510125932214252767275816329232454875804474844786"
                    .parse()
                    .unwrap(),
            },
            G1Point {
                X: "13083492661683431044045992285476184182144099829507350352128615182516530014777"
                    .parse()
                    .unwrap(),
                Y: "602051281796153692392523702676782023472744522032670801091617246498551238913"
                    .parse()
                    .unwrap(),
            },
            G1Point {
                X: "9732465972180335629969421513785602934706096902316483580882842789662669212890"
                    .parse()
                    .unwrap(),
                Y: "2776526698606888434074200384264824461688198384989521091253289776235602495678"
                    .parse()
                    .unwrap(),
            },
            G1Point {
                X: "8586364274534577154894611080234048648883781955345622578531233113180532234842"
                    .parse()
                    .unwrap(),
                Y: "21276134929883121123323359450658320820075698490666870487450985603988214349407"
                    .parse()
                    .unwrap(),
            },
            G1Point {
                X: "4910628533171597675018724709631788948355422829499855033965018665300386637884"
                    .parse()
                    .unwrap(),
                Y: "20532468890024084510431799098097081600480376127870299142189696620752500664302"
                    .parse()
                    .unwrap(),
            },
            G1Point {
                X: "15335858102289947642505450692012116222827233918185150176888641903531542034017"
                    .parse()
                    .unwrap(),
                Y: "5311597067667671581646709998171703828965875677637292315055030353779531404812"
                    .parse()
                    .unwrap(),
            }
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
