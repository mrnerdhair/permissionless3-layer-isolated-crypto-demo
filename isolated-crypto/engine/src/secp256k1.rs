use crate::bindings::exports::component::isolated_crypto;

use isolated_crypto::{
    secp256k1::{Guest, CompressedPoint, GuestEcdsaKey, RecoveryId, Signature},
    types::{DigestAlgorithm256, U256be},
};

use k256::{
    ecdsa::hazmat::SignPrimitive, elliptic_curve::point::AffineCoordinates, Scalar, SecretKey,
};
use sha2::{Digest, Sha256};
use sha3::Keccak256;

#[allow(unused)]
fn from_binding_recovery_id(value: RecoveryId) -> k256::ecdsa::RecoveryId {
    k256::ecdsa::RecoveryId::new(
        (value & RecoveryId::IS_Y_ODD) == RecoveryId::IS_Y_ODD,
        (value & RecoveryId::IS_X_REDUCED) == RecoveryId::IS_X_REDUCED,
    )
}

fn into_binding_recovery_id(value: k256::ecdsa::RecoveryId) -> RecoveryId {
    let mut out = RecoveryId::empty();
    out.set(RecoveryId::IS_Y_ODD, value.is_y_odd());
    out.set(RecoveryId::IS_X_REDUCED, value.is_x_reduced());
    out
}

pub struct EcdsaKey {
    secret_key: SecretKey,
}

impl EcdsaKey {
    pub fn new(secret_key: SecretKey) -> Self {
        Self { secret_key }
    }
}

impl GuestEcdsaKey for EcdsaKey {
    fn get_public_key(&self) -> CompressedPoint {
        let pk_point = self.secret_key.public_key();
        let pk_point = pk_point.as_affine();
        CompressedPoint {
            x: pk_point.x().to_vec(),
            is_y_odd: pk_point.y_is_odd().into(),
        }
    }

    fn sign(
        &self,
        digest_algorithm: DigestAlgorithm256,
        message: Vec<u8>,
        counter: Option<u32>,
    ) -> (Signature, RecoveryId) {
        let digest = match digest_algorithm {
            DigestAlgorithm256::Sha256 => Sha256::digest(&message),
            DigestAlgorithm256::Keccak256 => Keccak256::digest(&message),
        };
        self.sign_raw(digest.to_vec().try_into().unwrap(), counter)
    }

    fn sign_raw(
        &self,
        digest: U256be,
        counter: Option<u32>,
    ) -> (Signature, RecoveryId) {
        let key: Scalar = *self.secret_key.to_nonzero_scalar();
        let entropy = match counter {
            None => vec![],
            Some(counter) => counter.to_be_bytes().to_vec(),
        };

        let digest: [u8; 32] = digest.try_into().unwrap();

        let (sig, recovery_id) = key
            .try_sign_prehashed_rfc6979::<Sha256>(&digest.into(), &entropy)
            .unwrap();

        (
            Signature {
                r: sig.r().to_bytes().to_vec(),
                s: sig.s().to_bytes().to_vec(),
            },
            into_binding_recovery_id(recovery_id.unwrap()),
        )
    }
}

impl Guest for crate::Component {
    type EcdsaKey = EcdsaKey;
}
