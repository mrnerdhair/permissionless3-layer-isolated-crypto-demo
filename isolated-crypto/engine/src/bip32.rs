use crate::bindings::exports::mrnerdhair::isolated_crypto;

use bip32::XPrv;
use isolated_crypto::bip32::{Guest, GuestNode, GuestSeed};
use k256::elliptic_curve::point::AffineCoordinates;

pub struct Seed {
    seed: bip32::Seed,
}

impl Seed {
    pub fn new(seed: bip32::Seed) -> Self {
        Self { seed }
    }
}

impl GuestSeed for Seed {
    fn to_master_key(&self, hmac_key: Option<Vec<u8>>) -> isolated_crypto::bip32::Node {
        if !hmac_key.is_none() {
            unimplemented!()
        }
        isolated_crypto::bip32::Node::new(Node::new(
            bip32::XPrv::new(self.seed.as_bytes()).unwrap(),
        ))
    }
}

#[derive(Clone)]
pub struct Node {
    xprv: bip32::XPrv,
}

impl Node {
    pub fn new(xprv: XPrv) -> Self {
        Self { xprv }
    }
}

impl GuestNode for Node {
    fn get_public_key(&self) -> isolated_crypto::bip32::CompressedPoint {
        let pk_point = self.xprv.public_key();
        let pk_point = pk_point.public_key().as_affine();
        isolated_crypto::bip32::CompressedPoint {
            x: pk_point.x().to_vec(),
            is_y_odd: pk_point.y_is_odd().into(),
        }
    }

    fn get_chain_code(&self) -> isolated_crypto::bip32::U256be {
        self.xprv.public_key().attrs().chain_code.into()
    }

    fn derive(&self, index: u32) -> isolated_crypto::bip32::Node {
        isolated_crypto::bip32::Node::new(Self::new(
            self.xprv
                .derive_child(
                    bip32::ChildNumber::new(
                        index & !bip32::ChildNumber::HARDENED_FLAG,
                        index & bip32::ChildNumber::HARDENED_FLAG != 0,
                    )
                    .unwrap(),
                )
                .unwrap(),
        ))
    }

    fn clone(&self) -> isolated_crypto::bip32::Node {
        isolated_crypto::bip32::Node::new(<Self as Clone>::clone(self))
    }

    fn into_secp256k1_ecdsa_key(&self) -> isolated_crypto::bip32::EcdsaKey {
        isolated_crypto::bip32::EcdsaKey::new(super::secp256k1::EcdsaKey::new(
            self.xprv.private_key().clone().into(),
        ))
    }
}

impl Guest for crate::Component {
    type Seed = Seed;
    type Node = Node;
}
