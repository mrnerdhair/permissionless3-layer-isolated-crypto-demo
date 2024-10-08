#[allow(warnings)]
mod bindings;

use bindings::exports::mrnerdhair::isolated_crypto;

use isolated_crypto::mnemonic_provider::{Guest, Mnemonic};

struct Component;

impl Guest for Component {
    fn get_mnemonic() -> Result<Mnemonic, String> {
        Mnemonic::new(&std::env::var("MNEMONIC").or(Err("error fetching mnemonic"))?)
    }
}

bindings::export!(Component with_types_in bindings);
