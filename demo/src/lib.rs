#[allow(warnings)]
mod bindings;

use layer_wasi::{block_on, Reactor, Request, WasiPollable};

use crate::bindings::component::isolated_crypto::{self, bip39::Mnemonic, secp256k1::EcdsaKey};
use crate::bindings::{Guest, Output, TaskQueueInput};

use isolated_crypto::bip32::CompressedPoint;
use isolated_crypto::secp256k1::RecoveryId;
use sha3::{Digest, Keccak256};

struct Component;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct EtherscanVerifyMessageSignatureResponseInner {
    #[serde(rename = "verifiedMessageLocation")]
    pub verified_message_location: String,
}

#[derive(Deserialize, Debug)]
pub struct EtherscanVerifyMessageSignatureResponse {
    pub d: EtherscanVerifyMessageSignatureResponseInner,
}

fn compressed_point_to_sec1(value: CompressedPoint) -> [u8; 33] {
    [
        [if !value.is_y_odd { 0x02u8 } else { 0x03u8 }].as_slice(),
        &value.x,
    ]
    .concat()
    .try_into()
    .unwrap()
}

async fn verify_on_etherscan(
    reactor: &Reactor,
    address: [u8; 20],
    sig: [u8; 65],
    message: String,
) -> Output {
    let req_body = format!(
        r#"{{"address":"0x{}","messageSignature":"0x{}","messageRaw":"{}","saveOption":"1"}}"#,
        hex::encode(address),
        hex::encode(sig),
        &message
    );

    let mut req =
        Request::post("https://etherscan.io/verifiedSignatures.aspx/VerifyMessageSignature")?;
    req.headers = vec![("Content-Type".to_string(), "application/json".to_string())];
    req.body = req_body.into_bytes();
    let res = reactor.send(req).await?;

    match res.status {
        200 => res
            .json::<EtherscanVerifyMessageSignatureResponse>()
            .map(|info| {
                format!(
                    "https://etherscan.io/{}",
                    &info.d.verified_message_location[1..]
                )
                .into_bytes()
            }),
        status => Err(format!("unexpected status code: {status}")),
    }
}

fn get_ethereum_key(mnemonic: &Mnemonic) -> EcdsaKey {
    let seed = mnemonic.to_seed("");
    let node = seed.to_master_key(None);
    let node = node.derive(0x80000000 + 44);
    let node = node.derive(0x80000000 + 60);
    let node = node.derive(0x80000000 + 0);
    let node = node.derive(0);
    let node = node.derive(0);

    node.into_secp256k1_ecdsa_key()
}

fn get_address(mnemonic: &Mnemonic) -> [u8; 20] {
    let ecdsa_key = get_ethereum_key(mnemonic);
    let pk = compressed_point_to_sec1(ecdsa_key.get_public_key());
    let address = k256::ecdsa::VerifyingKey::from_sec1_bytes(&pk)
        .unwrap()
        .to_encoded_point(false);
    let address = [address.x().unwrap().clone(), address.y().unwrap().clone()].concat();
    let address = Keccak256::digest(address.clone());
    let address: [u8; 20] = address[12..32].try_into().unwrap();

    address
}

fn personal_sign(mnemonic: &Mnemonic, message: &[u8]) -> [u8; 65] {
    let ecdsa_key = get_ethereum_key(mnemonic);
    let sign_bytes = [
        "\x19Ethereum Signed Message:\n".as_bytes(),
        format!("{}", message.len()).as_bytes(),
        &message,
    ]
    .concat();
    let (sig, rec_id) = ecdsa_key.sign(
        isolated_crypto::types::DigestAlgorithm256::Keccak256,
        &sign_bytes,
        None,
    );
    let v: u8 =
        27 + if !rec_id.contains(RecoveryId::IS_Y_ODD) {
            0b00
        } else {
            0b01
        } + if rec_id.contains(RecoveryId::IS_X_REDUCED) {
            0b10
        } else {
            0b00
        };
    let sig: [u8; 65] = [sig.r, sig.s, [v].to_vec()].concat().try_into().unwrap();

    sig
}

impl Guest for Component {
    fn run_task(input: TaskQueueInput) -> Output {
        block_on(|reactor: Reactor| async { Self::run(reactor, input).await })
    }
}

impl Component {
    pub async fn run(reactor: Reactor, input: TaskQueueInput) -> Output {
        let message: String =
            String::from_utf8(input.request).or(Err("input must be valid UTF-8"))?;
        let mnemonic = isolated_crypto::mnemonic_provider::get_mnemonic()?;

        let address = get_address(&mnemonic);
        let sig = personal_sign(&mnemonic, message.as_bytes());
        verify_on_etherscan(&reactor, address, sig, message).await
    }
}

bindings::export!(Component with_types_in bindings);
