# WASM Isolated Crypto Component

Layer is a system which enables developers to build their own lightweight AVS. Developers build WASM components using their language of choice and can deploy them to run on-chain, where they are run by a set of operators. WebAssembly components are exciting because they can be composed with strong isolation guarantees between components, which enables a practical capability-based security model.

This is a prototype of an interface specification for allowing AVS components running on Layer to use cryptographic keys controlled by the network's operator nodes. The strong isolation provided by the WASM component architecture, combined with the careful choice of API surface area, ensures that "tenant" components can use these keys to perform cryptographic operations without being able to leak or exfiltrate the key material itself.

An implementation of an software-backed isolated crypto engine is also provided, but use of the common `isolated-crypto` interface means that any engine implementation can be plugged into client components. Future engine implementations could, for example, perform cryptographic tasks in a hardware enclave or via an MPC key-sharing arrangement, providing enhanced security guarantees.

## Highlights

- The [`mrnerdhair:isolated-crypto/engine`](https://wa.dev/mrnerdhair:isolated-crypto) WIT package defines the interface exposed to components using cryptographic material. Currently this supports ECDSA over the secp256k1 curve (with support for creating Ethereum-style recoverable signatures), the BIP32 keypair derivation scheme, and the BIP39 mechanism for providing entropy via a mnemonic phrase. The API surface has been carefully chosen to ensure that the cryptographic primitives exposed support use across many common blockchains.
- The [`mrnerdhair:isolated-crypto/mnemonic-provider`](https://wa.dev/mrnerdhair:isolated-crypto#mnemonic-provider) interface provides the host-specific machinery needed to inject secret material. Currently this loads a 24-word mnemonic string from an environment variable.
- The [`mrnerdhair:isolated-crypto-demo/demo`](https://wa.dev/mrnerdhair:isolated-crypto-demo) component uses these interfaces to create an Ethereum-style `personal_sign` signature over a provided message and submit it to Etherscan's registry of publically verifiable signatures, returning a URL which simultaneously proves correct use of the provided private key and public disclosure of the signature and message.

## Dependencies

- cargo-component
- wac
- wkg
- avs-toolkit-cli

## Notes on linking with `wkg`

You must bump the version strings on all `.wit` files and in the corresponding `package.metadata.component.target` sections of each `Cargo.toml` file whenever changes are made to the WIT interfaces.

Due to `wkg`'s currently limited support for local paths, you will need to have a default registry set for `wkg` and associated credentials which allow publishing to the `mrnerdhair:isolated-crypto` and `mrnerdhair:isolated-crypto-demo` namespaces. You will probably not have these credentials. Unfortunately, this means you'll need to replace the `mrnerdhair` namespace with your own in quite a few files to perform a successful build. 

To mitigate this, this repo currently includes committed intermediate build products.

## Running the Demo

```sh
./build.sh  # Skip this step if you don't happen to have my personal keys! The committed intermediate build products will suffice for the demo.
./demo.sh   # Signs with the default mnemonic and message
MNEMONIC="..." ./demo.sh "Your Message Here"    # Signs with other values; you must use a 24-word, English BIP39 mnemonic.
```
