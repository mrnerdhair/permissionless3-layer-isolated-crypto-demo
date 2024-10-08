#[allow(dead_code)]
pub mod mrnerdhair {
    #[allow(dead_code)]
    pub mod isolated_crypto {
        #[allow(dead_code, clippy::all)]
        pub mod types {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            /// Big-endian 256-bit integer. Will be enforced by type once WebAssembly/component-model#384 percolates through the tooling.
            /// In the meantime, expect a trap if you pass in a list of the wrong length.
            pub type U256be = _rt::Vec<u8>;
            #[repr(u8)]
            #[derive(Clone, Copy, Eq, PartialEq)]
            pub enum DigestAlgorithm256 {
                Sha256,
                Keccak256,
            }
            impl ::core::fmt::Debug for DigestAlgorithm256 {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        DigestAlgorithm256::Sha256 => {
                            f.debug_tuple("DigestAlgorithm256::Sha256").finish()
                        }
                        DigestAlgorithm256::Keccak256 => {
                            f.debug_tuple("DigestAlgorithm256::Keccak256").finish()
                        }
                    }
                }
            }
            impl DigestAlgorithm256 {
                #[doc(hidden)]
                pub unsafe fn _lift(val: u8) -> DigestAlgorithm256 {
                    if !cfg!(debug_assertions) {
                        return ::core::mem::transmute(val);
                    }
                    match val {
                        0 => DigestAlgorithm256::Sha256,
                        1 => DigestAlgorithm256::Keccak256,
                        _ => panic!("invalid enum discriminant"),
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod secp256k1 {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type U256be = super::super::super::mrnerdhair::isolated_crypto::types::U256be;
            pub type DigestAlgorithm256 = super::super::super::mrnerdhair::isolated_crypto::types::DigestAlgorithm256;
            #[derive(Clone)]
            pub struct CompressedPoint {
                pub x: U256be,
                pub is_y_odd: bool,
            }
            impl ::core::fmt::Debug for CompressedPoint {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("CompressedPoint")
                        .field("x", &self.x)
                        .field("is-y-odd", &self.is_y_odd)
                        .finish()
                }
            }
            wit_bindgen_rt::bitflags::bitflags! {
                #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)] pub
                struct RecoveryId : u8 { const IS_Y_ODD = 1 << 0; const IS_X_REDUCED = 1
                << 1; }
            }
            #[derive(Clone)]
            pub struct Signature {
                pub r: U256be,
                pub s: U256be,
            }
            impl ::core::fmt::Debug for Signature {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("Signature")
                        .field("r", &self.r)
                        .field("s", &self.s)
                        .finish()
                }
            }
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct EcdsaKey {
                handle: _rt::Resource<EcdsaKey>,
            }
            impl EcdsaKey {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for EcdsaKey {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(
                            wasm_import_module = "mrnerdhair:isolated-crypto/secp256k1@0.14.0"
                        )]
                        extern "C" {
                            #[link_name = "[resource-drop]ecdsa-key"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            impl EcdsaKey {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get_public_key(&self) -> CompressedPoint {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 12],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(
                            wasm_import_module = "mrnerdhair:isolated-crypto/secp256k1@0.14.0"
                        )]
                        extern "C" {
                            #[link_name = "[method]ecdsa-key.get-public-key"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = *ptr0.add(0).cast::<*mut u8>();
                        let l2 = *ptr0.add(4).cast::<usize>();
                        let len3 = l2;
                        let l4 = i32::from(*ptr0.add(8).cast::<u8>());
                        CompressedPoint {
                            x: _rt::Vec::from_raw_parts(l1.cast(), len3, len3),
                            is_y_odd: _rt::bool_lift(l4 as u8),
                        }
                    }
                }
            }
            impl EcdsaKey {
                #[allow(unused_unsafe, clippy::all)]
                pub fn sign(
                    &self,
                    digest_algorithm: DigestAlgorithm256,
                    message: &[u8],
                    counter: Option<u32>,
                ) -> (Signature, RecoveryId) {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 20]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 20],
                        );
                        let vec0 = message;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let (result1_0, result1_1) = match counter {
                            Some(e) => (1i32, _rt::as_i32(e)),
                            None => (0i32, 0i32),
                        };
                        let ptr2 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(
                            wasm_import_module = "mrnerdhair:isolated-crypto/secp256k1@0.14.0"
                        )]
                        extern "C" {
                            #[link_name = "[method]ecdsa-key.sign"]
                            fn wit_import(
                                _: i32,
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: i32,
                                _: i32,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: i32,
                            _: i32,
                            _: *mut u8,
                        ) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            digest_algorithm.clone() as i32,
                            ptr0.cast_mut(),
                            len0,
                            result1_0,
                            result1_1,
                            ptr2,
                        );
                        let l3 = *ptr2.add(0).cast::<*mut u8>();
                        let l4 = *ptr2.add(4).cast::<usize>();
                        let len5 = l4;
                        let l6 = *ptr2.add(8).cast::<*mut u8>();
                        let l7 = *ptr2.add(12).cast::<usize>();
                        let len8 = l7;
                        let l9 = i32::from(*ptr2.add(16).cast::<u8>());
                        (
                            Signature {
                                r: _rt::Vec::from_raw_parts(l3.cast(), len5, len5),
                                s: _rt::Vec::from_raw_parts(l6.cast(), len8, len8),
                            },
                            RecoveryId::empty()
                                | RecoveryId::from_bits_retain(((l9 as u8) << 0) as _),
                        )
                    }
                }
            }
            impl EcdsaKey {
                #[allow(unused_unsafe, clippy::all)]
                pub fn sign_raw(
                    &self,
                    digest: &U256be,
                    counter: Option<u32>,
                ) -> (Signature, RecoveryId) {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 20]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 20],
                        );
                        let vec0 = digest;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let (result1_0, result1_1) = match counter {
                            Some(e) => (1i32, _rt::as_i32(e)),
                            None => (0i32, 0i32),
                        };
                        let ptr2 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(
                            wasm_import_module = "mrnerdhair:isolated-crypto/secp256k1@0.14.0"
                        )]
                        extern "C" {
                            #[link_name = "[method]ecdsa-key.sign-raw"]
                            fn wit_import(
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: i32,
                                _: i32,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: i32,
                            _: i32,
                            _: *mut u8,
                        ) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            ptr0.cast_mut(),
                            len0,
                            result1_0,
                            result1_1,
                            ptr2,
                        );
                        let l3 = *ptr2.add(0).cast::<*mut u8>();
                        let l4 = *ptr2.add(4).cast::<usize>();
                        let len5 = l4;
                        let l6 = *ptr2.add(8).cast::<*mut u8>();
                        let l7 = *ptr2.add(12).cast::<usize>();
                        let len8 = l7;
                        let l9 = i32::from(*ptr2.add(16).cast::<u8>());
                        (
                            Signature {
                                r: _rt::Vec::from_raw_parts(l3.cast(), len5, len5),
                                s: _rt::Vec::from_raw_parts(l6.cast(), len8, len8),
                            },
                            RecoveryId::empty()
                                | RecoveryId::from_bits_retain(((l9 as u8) << 0) as _),
                        )
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod bip32 {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type U256be = super::super::super::mrnerdhair::isolated_crypto::types::U256be;
            pub type CompressedPoint = super::super::super::mrnerdhair::isolated_crypto::secp256k1::CompressedPoint;
            pub type EcdsaKey = super::super::super::mrnerdhair::isolated_crypto::secp256k1::EcdsaKey;
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Seed {
                handle: _rt::Resource<Seed>,
            }
            impl Seed {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for Seed {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(
                            wasm_import_module = "mrnerdhair:isolated-crypto/bip32@0.14.0"
                        )]
                        extern "C" {
                            #[link_name = "[resource-drop]seed"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Node {
                handle: _rt::Resource<Node>,
            }
            impl Node {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for Node {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(
                            wasm_import_module = "mrnerdhair:isolated-crypto/bip32@0.14.0"
                        )]
                        extern "C" {
                            #[link_name = "[resource-drop]node"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            impl Seed {
                #[allow(unused_unsafe, clippy::all)]
                pub fn to_master_key(&self, hmac_key: Option<&[u8]>) -> Node {
                    unsafe {
                        let (result1_0, result1_1, result1_2) = match hmac_key {
                            Some(e) => {
                                let vec0 = e;
                                let ptr0 = vec0.as_ptr().cast::<u8>();
                                let len0 = vec0.len();
                                (1i32, ptr0.cast_mut(), len0)
                            }
                            None => (0i32, ::core::ptr::null_mut(), 0usize),
                        };
                        #[cfg(target_arch = "wasm32")]
                        #[link(
                            wasm_import_module = "mrnerdhair:isolated-crypto/bip32@0.14.0"
                        )]
                        extern "C" {
                            #[link_name = "[method]seed.to-master-key"]
                            fn wit_import(_: i32, _: i32, _: *mut u8, _: usize) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: *mut u8, _: usize) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(
                            (self).handle() as i32,
                            result1_0,
                            result1_1,
                            result1_2,
                        );
                        Node::from_handle(ret as u32)
                    }
                }
            }
            impl Node {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get_public_key(&self) -> CompressedPoint {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 12],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(
                            wasm_import_module = "mrnerdhair:isolated-crypto/bip32@0.14.0"
                        )]
                        extern "C" {
                            #[link_name = "[method]node.get-public-key"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = *ptr0.add(0).cast::<*mut u8>();
                        let l2 = *ptr0.add(4).cast::<usize>();
                        let len3 = l2;
                        let l4 = i32::from(*ptr0.add(8).cast::<u8>());
                        super::super::super::mrnerdhair::isolated_crypto::secp256k1::CompressedPoint {
                            x: _rt::Vec::from_raw_parts(l1.cast(), len3, len3),
                            is_y_odd: _rt::bool_lift(l4 as u8),
                        }
                    }
                }
            }
            impl Node {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get_chain_code(&self) -> U256be {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 8],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(
                            wasm_import_module = "mrnerdhair:isolated-crypto/bip32@0.14.0"
                        )]
                        extern "C" {
                            #[link_name = "[method]node.get-chain-code"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = *ptr0.add(0).cast::<*mut u8>();
                        let l2 = *ptr0.add(4).cast::<usize>();
                        let len3 = l2;
                        _rt::Vec::from_raw_parts(l1.cast(), len3, len3)
                    }
                }
            }
            impl Node {
                #[allow(unused_unsafe, clippy::all)]
                pub fn derive(&self, index: u32) -> Node {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(
                            wasm_import_module = "mrnerdhair:isolated-crypto/bip32@0.14.0"
                        )]
                        extern "C" {
                            #[link_name = "[method]node.derive"]
                            fn wit_import(_: i32, _: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(
                            (self).handle() as i32,
                            _rt::as_i32(&index),
                        );
                        Node::from_handle(ret as u32)
                    }
                }
            }
            impl Node {
                #[allow(unused_unsafe, clippy::all)]
                pub fn clone(&self) -> Node {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(
                            wasm_import_module = "mrnerdhair:isolated-crypto/bip32@0.14.0"
                        )]
                        extern "C" {
                            #[link_name = "[method]node.clone"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        Node::from_handle(ret as u32)
                    }
                }
            }
            impl Node {
                #[allow(unused_unsafe, clippy::all)]
                pub fn into_secp256k1_ecdsa_key(&self) -> EcdsaKey {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(
                            wasm_import_module = "mrnerdhair:isolated-crypto/bip32@0.14.0"
                        )]
                        extern "C" {
                            #[link_name = "[method]node.into-secp256k1-ecdsa-key"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        super::super::super::mrnerdhair::isolated_crypto::secp256k1::EcdsaKey::from_handle(
                            ret as u32,
                        )
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod bip39 {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type Seed = super::super::super::mrnerdhair::isolated_crypto::bip32::Seed;
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Mnemonic {
                handle: _rt::Resource<Mnemonic>,
            }
            impl Mnemonic {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for Mnemonic {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(
                            wasm_import_module = "mrnerdhair:isolated-crypto/bip39@0.14.0"
                        )]
                        extern "C" {
                            #[link_name = "[resource-drop]mnemonic"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            impl Mnemonic {
                #[allow(unused_unsafe, clippy::all)]
                pub fn new(mnemonic: &str) -> Result<Mnemonic, _rt::String> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 12],
                        );
                        let vec0 = mnemonic;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(
                            wasm_import_module = "mrnerdhair:isolated-crypto/bip39@0.14.0"
                        )]
                        extern "C" {
                            #[link_name = "[static]mnemonic.new"]
                            fn wit_import(_: *mut u8, _: usize, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import(ptr0.cast_mut(), len0, ptr1);
                        let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                        match l2 {
                            0 => {
                                let e = {
                                    let l3 = *ptr1.add(4).cast::<i32>();
                                    Mnemonic::from_handle(l3 as u32)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l4 = *ptr1.add(4).cast::<*mut u8>();
                                    let l5 = *ptr1.add(8).cast::<usize>();
                                    let len6 = l5;
                                    let bytes6 = _rt::Vec::from_raw_parts(
                                        l4.cast(),
                                        len6,
                                        len6,
                                    );
                                    _rt::string_lift(bytes6)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Mnemonic {
                #[allow(unused_unsafe, clippy::all)]
                pub fn to_seed(&self, passphrase: &str) -> Seed {
                    unsafe {
                        let vec0 = passphrase;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        #[cfg(target_arch = "wasm32")]
                        #[link(
                            wasm_import_module = "mrnerdhair:isolated-crypto/bip39@0.14.0"
                        )]
                        extern "C" {
                            #[link_name = "[method]mnemonic.to-seed"]
                            fn wit_import(_: i32, _: *mut u8, _: usize) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(
                            (self).handle() as i32,
                            ptr0.cast_mut(),
                            len0,
                        );
                        super::super::super::mrnerdhair::isolated_crypto::bip32::Seed::from_handle(
                            ret as u32,
                        )
                    }
                }
            }
        }
    }
}
#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod mrnerdhair {
        #[allow(dead_code)]
        pub mod isolated_crypto {
            #[allow(dead_code, clippy::all)]
            pub mod mnemonic_provider {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type Mnemonic = super::super::super::super::mrnerdhair::isolated_crypto::bip39::Mnemonic;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_get_mnemonic_cabi<T: Guest>() -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_mnemonic();
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result0 {
                        Ok(e) => {
                            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
                            *ptr1.add(4).cast::<i32>() = (e).take_handle() as i32;
                        }
                        Err(e) => {
                            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
                            let vec2 = (e.into_bytes()).into_boxed_slice();
                            let ptr2 = vec2.as_ptr().cast::<u8>();
                            let len2 = vec2.len();
                            ::core::mem::forget(vec2);
                            *ptr1.add(8).cast::<usize>() = len2;
                            *ptr1.add(4).cast::<*mut u8>() = ptr2.cast_mut();
                        }
                    };
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_get_mnemonic<T: Guest>(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                    }
                }
                pub trait Guest {
                    fn get_mnemonic() -> Result<Mnemonic, _rt::String>;
                }
                #[doc(hidden)]
                macro_rules! __export_mrnerdhair_isolated_crypto_mnemonic_provider_0_14_0_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "mrnerdhair:isolated-crypto/mnemonic-provider@0.14.0#get-mnemonic"]
                        unsafe extern "C" fn export_get_mnemonic() -> * mut u8 {
                        $($path_to_types)*:: _export_get_mnemonic_cabi::<$ty > () }
                        #[export_name =
                        "cabi_post_mrnerdhair:isolated-crypto/mnemonic-provider@0.14.0#get-mnemonic"]
                        unsafe extern "C" fn _post_return_get_mnemonic(arg0 : * mut u8,)
                        { $($path_to_types)*:: __post_return_get_mnemonic::<$ty > (arg0)
                        } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_mrnerdhair_isolated_crypto_mnemonic_provider_0_14_0_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 12]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 12],
                );
            }
        }
    }
}
mod _rt {
    pub use alloc_crate::vec::Vec;
    use core::fmt;
    use core::marker;
    use core::sync::atomic::{AtomicU32, Ordering::Relaxed};
    /// A type which represents a component model resource, either imported or
    /// exported into this component.
    ///
    /// This is a low-level wrapper which handles the lifetime of the resource
    /// (namely this has a destructor). The `T` provided defines the component model
    /// intrinsics that this wrapper uses.
    ///
    /// One of the chief purposes of this type is to provide `Deref` implementations
    /// to access the underlying data when it is owned.
    ///
    /// This type is primarily used in generated code for exported and imported
    /// resources.
    #[repr(transparent)]
    pub struct Resource<T: WasmResource> {
        handle: AtomicU32,
        _marker: marker::PhantomData<T>,
    }
    /// A trait which all wasm resources implement, namely providing the ability to
    /// drop a resource.
    ///
    /// This generally is implemented by generated code, not user-facing code.
    #[allow(clippy::missing_safety_doc)]
    pub unsafe trait WasmResource {
        /// Invokes the `[resource-drop]...` intrinsic.
        unsafe fn drop(handle: u32);
    }
    impl<T: WasmResource> Resource<T> {
        #[doc(hidden)]
        pub unsafe fn from_handle(handle: u32) -> Self {
            debug_assert!(handle != u32::MAX);
            Self {
                handle: AtomicU32::new(handle),
                _marker: marker::PhantomData,
            }
        }
        /// Takes ownership of the handle owned by `resource`.
        ///
        /// Note that this ideally would be `into_handle` taking `Resource<T>` by
        /// ownership. The code generator does not enable that in all situations,
        /// unfortunately, so this is provided instead.
        ///
        /// Also note that `take_handle` is in theory only ever called on values
        /// owned by a generated function. For example a generated function might
        /// take `Resource<T>` as an argument but then call `take_handle` on a
        /// reference to that argument. In that sense the dynamic nature of
        /// `take_handle` should only be exposed internally to generated code, not
        /// to user code.
        #[doc(hidden)]
        pub fn take_handle(resource: &Resource<T>) -> u32 {
            resource.handle.swap(u32::MAX, Relaxed)
        }
        #[doc(hidden)]
        pub fn handle(resource: &Resource<T>) -> u32 {
            resource.handle.load(Relaxed)
        }
    }
    impl<T: WasmResource> fmt::Debug for Resource<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Resource").field("handle", &self.handle).finish()
        }
    }
    impl<T: WasmResource> Drop for Resource<T> {
        fn drop(&mut self) {
            unsafe {
                match self.handle.load(Relaxed) {
                    u32::MAX => {}
                    other => T::drop(other),
                }
            }
        }
    }
    pub unsafe fn bool_lift(val: u8) -> bool {
        if cfg!(debug_assertions) {
            match val {
                0 => false,
                1 => true,
                _ => panic!("invalid bool discriminant"),
            }
        } else {
            val != 0
        }
    }
    pub fn as_i32<T: AsI32>(t: T) -> i32 {
        t.as_i32()
    }
    pub trait AsI32 {
        fn as_i32(self) -> i32;
    }
    impl<'a, T: Copy + AsI32> AsI32 for &'a T {
        fn as_i32(self) -> i32 {
            (*self).as_i32()
        }
    }
    impl AsI32 for i32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for char {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for usize {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    pub use alloc_crate::string::String;
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub unsafe fn invalid_enum_discriminant<T>() -> T {
        if cfg!(debug_assertions) {
            panic!("invalid enum discriminant")
        } else {
            core::hint::unreachable_unchecked()
        }
    }
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    extern crate alloc as alloc_crate;
    pub use alloc_crate::alloc;
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_mnemonic_provider_world_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::mrnerdhair::isolated_crypto::mnemonic_provider::__export_mrnerdhair_isolated_crypto_mnemonic_provider_0_14_0_cabi!($ty
        with_types_in $($path_to_types_root)*::
        exports::mrnerdhair::isolated_crypto::mnemonic_provider);
    };
}
#[doc(inline)]
pub(crate) use __export_mnemonic_provider_world_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.30.0:mnemonic-provider-world:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 1525] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xe7\x0a\x01A\x02\x01\
A\x10\x01B\x04\x01p}\x04\0\x06u256be\x03\0\0\x01m\x02\x06sha256\x09keccak256\x04\
\0\x13digest-algorithm256\x03\0\x02\x03\x01'mrnerdhair:isolated-crypto/types@0.1\
4.0\x05\0\x02\x03\0\0\x06u256be\x02\x03\0\0\x13digest-algorithm256\x01B\x15\x02\x03\
\x02\x01\x01\x04\0\x06u256be\x03\0\0\x02\x03\x02\x01\x02\x04\0\x13digest-algorit\
hm256\x03\0\x02\x01r\x02\x01x\x01\x08is-y-odd\x7f\x04\0\x10compressed-point\x03\0\
\x04\x01n\x02\x08is-y-odd\x0cis-x-reduced\x04\0\x0brecovery-id\x03\0\x06\x01r\x02\
\x01r\x01\x01s\x01\x04\0\x09signature\x03\0\x08\x04\0\x09ecdsa-key\x03\x01\x01h\x0a\
\x01@\x01\x04self\x0b\0\x05\x04\0\x20[method]ecdsa-key.get-public-key\x01\x0c\x01\
p}\x01ky\x01o\x02\x09\x07\x01@\x04\x04self\x0b\x10digest-algorithm\x03\x07messag\
e\x0d\x07counter\x0e\0\x0f\x04\0\x16[method]ecdsa-key.sign\x01\x10\x01@\x03\x04s\
elf\x0b\x06digest\x01\x07counter\x0e\0\x0f\x04\0\x1a[method]ecdsa-key.sign-raw\x01\
\x11\x03\x01+mrnerdhair:isolated-crypto/secp256k1@0.14.0\x05\x03\x02\x03\0\x01\x10\
compressed-point\x02\x03\0\x01\x09ecdsa-key\x01B\x1a\x02\x03\x02\x01\x01\x04\0\x06\
u256be\x03\0\0\x02\x03\x02\x01\x04\x04\0\x10compressed-point\x03\0\x02\x02\x03\x02\
\x01\x05\x04\0\x09ecdsa-key\x03\0\x04\x04\0\x04seed\x03\x01\x04\0\x04node\x03\x01\
\x01h\x06\x01p}\x01k\x09\x01i\x07\x01@\x02\x04self\x08\x08hmac-key\x0a\0\x0b\x04\
\0\x1a[method]seed.to-master-key\x01\x0c\x01h\x07\x01@\x01\x04self\x0d\0\x03\x04\
\0\x1b[method]node.get-public-key\x01\x0e\x01@\x01\x04self\x0d\0\x01\x04\0\x1b[m\
ethod]node.get-chain-code\x01\x0f\x01@\x02\x04self\x0d\x05indexy\0\x0b\x04\0\x13\
[method]node.derive\x01\x10\x01@\x01\x04self\x0d\0\x0b\x04\0\x12[method]node.clo\
ne\x01\x11\x01i\x05\x01@\x01\x04self\x0d\0\x12\x04\0%[method]node.into-secp256k1\
-ecdsa-key\x01\x13\x03\x01'mrnerdhair:isolated-crypto/bip32@0.14.0\x05\x06\x02\x03\
\0\x02\x04seed\x01B\x0b\x02\x03\x02\x01\x07\x04\0\x04seed\x03\0\0\x04\0\x08mnemo\
nic\x03\x01\x01i\x02\x01j\x01\x03\x01s\x01@\x01\x08mnemonics\0\x04\x04\0\x14[sta\
tic]mnemonic.new\x01\x05\x01h\x02\x01i\x01\x01@\x02\x04self\x06\x0apassphrases\0\
\x07\x04\0\x18[method]mnemonic.to-seed\x01\x08\x03\x01'mrnerdhair:isolated-crypt\
o/bip39@0.14.0\x05\x08\x02\x03\0\x03\x08mnemonic\x01B\x06\x02\x03\x02\x01\x09\x04\
\0\x08mnemonic\x03\0\0\x01i\x01\x01j\x01\x02\x01s\x01@\0\0\x03\x04\0\x0cget-mnem\
onic\x01\x04\x04\x013mrnerdhair:isolated-crypto/mnemonic-provider@0.14.0\x05\x0a\
\x04\x019mrnerdhair:isolated-crypto/mnemonic-provider-world@0.14.0\x04\0\x0b\x1d\
\x01\0\x17mnemonic-provider-world\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\
\x0dwit-component\x070.215.0\x10wit-bindgen-rust\x060.30.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
