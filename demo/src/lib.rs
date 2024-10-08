#[allow(warnings)]
mod bindings;

use crate::bindings::mrnerdhair::isolated_crypto;
use crate::bindings::{Guest, Output, TaskQueueInput};

use isolated_crypto::bip32::CompressedPoint;
use isolated_crypto::secp256k1::RecoveryId;
use sha3::{Digest, Keccak256};

struct Component;

fn compressed_point_to_sec1(value: CompressedPoint) -> [u8; 33] {
    [[if !value.is_y_odd { 0x02u8 } else { 0x03u8 }].as_slice(), &value.x].concat().try_into().unwrap()
}

impl Guest for Component {
    fn run_task(request: TaskQueueInput) -> Output {
        let message: String = String::from_utf8(request.request).or(Err("input must be valid UTF-8"))?;
        let mnemonic = isolated_crypto::mnemonic_provider::get_mnemonic()?;
        let seed = mnemonic.to_seed("");
        let node = seed.to_master_key(None);
        let node = node.derive(0x80000000 + 44);
        let node = node.derive(0x80000000 + 60);
        let node = node.derive(0x80000000 + 0);
        let node = node.derive(0);
        let node = node.derive(0);
        let ecdsa_key = node.into_secp256k1_ecdsa_key();
        let sign_bytes = ["\x19Ethereum Signed Message:\n".as_bytes(), format!("{}", message.len()).as_bytes(),  &message.as_bytes()].concat();
        let (sig, rec_id) = ecdsa_key.sign(isolated_crypto::types::DigestAlgorithm256::Keccak256, &sign_bytes, None);
        let v = 27 + if !rec_id.contains(RecoveryId::IS_Y_ODD) { 0b00 } else { 0b01 } + if rec_id.contains(RecoveryId::IS_X_REDUCED) { 0b10 } else { 0b00 };
        let pk =  compressed_point_to_sec1(ecdsa_key.get_public_key());
        let address = k256::ecdsa::VerifyingKey::from_sec1_bytes(&pk).unwrap().to_encoded_point(false);
        let address = [address.x().unwrap().clone(), address.y().unwrap().clone()].concat();
        let address = Keccak256::digest(address.clone());
        let address = &address[12..32];
        Ok(format!(r#"{{"address": "0x{}", "msg": "{}", "sig": "0x{}{}{:02x}"}}"#, hex::encode(address), &message, hex::encode(sig.r), hex::encode(sig.s), v).into())
    }
}

bindings::export!(Component with_types_in bindings);
