use crate::bindings::exports::mrnerdhair::isolated_crypto;

use super::bip32::Seed;
use isolated_crypto::bip39::{Guest, GuestMnemonic};

pub struct Mnemonic {
    mnemonic: bip32::Mnemonic,
}

impl GuestMnemonic for Mnemonic {
    fn new(mnemonic: String) -> Result<isolated_crypto::bip39::Mnemonic, String> {
        Ok(isolated_crypto::bip39::Mnemonic::new(Self {
            mnemonic: bip32::Mnemonic::new(mnemonic, Default::default()).map_err(|x| format!("{x}"))?,
        }))
    }

    fn to_seed(&self, passphrase: String) -> isolated_crypto::bip39::Seed {
        isolated_crypto::bip32::Seed::new(Seed::new(self.mnemonic.to_seed(&passphrase)))
    }
}

impl Guest for crate::Component {
    type Mnemonic = Mnemonic;
}
