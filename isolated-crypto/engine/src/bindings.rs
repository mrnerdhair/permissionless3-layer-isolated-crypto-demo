#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod mrnerdhair {
        #[allow(dead_code)]
        pub mod isolated_crypto {
            #[allow(dead_code, clippy::all)]
            pub mod types {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
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
                #[doc(hidden)]
                macro_rules! __export_mrnerdhair_isolated_crypto_types_0_14_0_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = {};
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_mrnerdhair_isolated_crypto_types_0_14_0_cabi;
            }
            #[allow(dead_code, clippy::all)]
            pub mod secp256k1 {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type U256be = super::super::super::super::exports::mrnerdhair::isolated_crypto::types::U256be;
                pub type DigestAlgorithm256 = super::super::super::super::exports::mrnerdhair::isolated_crypto::types::DigestAlgorithm256;
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
                    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
                    pub struct RecoveryId : u8 { const IS_Y_ODD = 1 << 0; const
                    IS_X_REDUCED = 1 << 1; }
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
                type _EcdsaKeyRep<T> = Option<T>;
                impl EcdsaKey {
                    /// Creates a new resource from the specified representation.
                    ///
                    /// This function will create a new resource handle by moving `val` onto
                    /// the heap and then passing that heap pointer to the component model to
                    /// create a handle. The owned handle is then returned as `EcdsaKey`.
                    pub fn new<T: GuestEcdsaKey>(val: T) -> Self {
                        Self::type_guard::<T>();
                        let val: _EcdsaKeyRep<T> = Some(val);
                        let ptr: *mut _EcdsaKeyRep<T> = _rt::Box::into_raw(
                            _rt::Box::new(val),
                        );
                        unsafe { Self::from_handle(T::_resource_new(ptr.cast())) }
                    }
                    /// Gets access to the underlying `T` which represents this resource.
                    pub fn get<T: GuestEcdsaKey>(&self) -> &T {
                        let ptr = unsafe { &*self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    /// Gets mutable access to the underlying `T` which represents this
                    /// resource.
                    pub fn get_mut<T: GuestEcdsaKey>(&mut self) -> &mut T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_mut().unwrap()
                    }
                    /// Consumes this resource and returns the underlying `T`.
                    pub fn into_inner<T: GuestEcdsaKey>(self) -> T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.take().unwrap()
                    }
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
                    #[doc(hidden)]
                    fn type_guard<T: 'static>() {
                        use core::any::TypeId;
                        static mut LAST_TYPE: Option<TypeId> = None;
                        unsafe {
                            assert!(! cfg!(target_feature = "atomics"));
                            let id = TypeId::of::<T>();
                            match LAST_TYPE {
                                Some(ty) => {
                                    assert!(
                                        ty == id, "cannot use two types with this resource type"
                                    )
                                }
                                None => LAST_TYPE = Some(id),
                            }
                        }
                    }
                    #[doc(hidden)]
                    pub unsafe fn dtor<T: 'static>(handle: *mut u8) {
                        Self::type_guard::<T>();
                        let _ = _rt::Box::from_raw(handle as *mut _EcdsaKeyRep<T>);
                    }
                    fn as_ptr<T: GuestEcdsaKey>(&self) -> *mut _EcdsaKeyRep<T> {
                        EcdsaKey::type_guard::<T>();
                        T::_resource_rep(self.handle()).cast()
                    }
                }
                /// A borrowed version of [`EcdsaKey`] which represents a borrowed value
                /// with the lifetime `'a`.
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct EcdsaKeyBorrow<'a> {
                    rep: *mut u8,
                    _marker: core::marker::PhantomData<&'a EcdsaKey>,
                }
                impl<'a> EcdsaKeyBorrow<'a> {
                    #[doc(hidden)]
                    pub unsafe fn lift(rep: usize) -> Self {
                        Self {
                            rep: rep as *mut u8,
                            _marker: core::marker::PhantomData,
                        }
                    }
                    /// Gets access to the underlying `T` in this resource.
                    pub fn get<T: GuestEcdsaKey>(&self) -> &T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    fn as_ptr<T: 'static>(&self) -> *mut _EcdsaKeyRep<T> {
                        EcdsaKey::type_guard::<T>();
                        self.rep.cast()
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
                                wasm_import_module = "[export]mrnerdhair:isolated-crypto/secp256k1@0.14.0"
                            )]
                            extern "C" {
                                #[link_name = "[resource-drop]ecdsa-key"]
                                fn drop(_: u32);
                            }
                            drop(_handle);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_ecdsa_key_get_public_key_cabi<
                    T: GuestEcdsaKey,
                >(arg0: *mut u8) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_public_key(
                        EcdsaKeyBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let CompressedPoint { x: x2, is_y_odd: is_y_odd2 } = result0;
                    let vec3 = (x2).into_boxed_slice();
                    let ptr3 = vec3.as_ptr().cast::<u8>();
                    let len3 = vec3.len();
                    ::core::mem::forget(vec3);
                    *ptr1.add(4).cast::<usize>() = len3;
                    *ptr1.add(0).cast::<*mut u8>() = ptr3.cast_mut();
                    *ptr1.add(8).cast::<u8>() = (match is_y_odd2 {
                        true => 1,
                        false => 0,
                    }) as u8;
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_ecdsa_key_get_public_key<
                    T: GuestEcdsaKey,
                >(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let base2 = l0;
                    let len2 = l1;
                    _rt::cabi_dealloc(base2, len2 * 1, 1);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_ecdsa_key_sign_cabi<T: GuestEcdsaKey>(
                    arg0: *mut u8,
                    arg1: i32,
                    arg2: *mut u8,
                    arg3: usize,
                    arg4: i32,
                    arg5: i32,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg3;
                    let result1 = T::sign(
                        EcdsaKeyBorrow::lift(arg0 as u32 as usize).get(),
                        super::super::super::super::exports::mrnerdhair::isolated_crypto::types::DigestAlgorithm256::_lift(
                            arg1 as u8,
                        ),
                        _rt::Vec::from_raw_parts(arg2.cast(), len0, len0),
                        match arg4 {
                            0 => None,
                            1 => {
                                let e = arg5 as u32;
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        },
                    );
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let (t3_0, t3_1) = result1;
                    let Signature { r: r4, s: s4 } = t3_0;
                    let vec5 = (r4).into_boxed_slice();
                    let ptr5 = vec5.as_ptr().cast::<u8>();
                    let len5 = vec5.len();
                    ::core::mem::forget(vec5);
                    *ptr2.add(4).cast::<usize>() = len5;
                    *ptr2.add(0).cast::<*mut u8>() = ptr5.cast_mut();
                    let vec6 = (s4).into_boxed_slice();
                    let ptr6 = vec6.as_ptr().cast::<u8>();
                    let len6 = vec6.len();
                    ::core::mem::forget(vec6);
                    *ptr2.add(12).cast::<usize>() = len6;
                    *ptr2.add(8).cast::<*mut u8>() = ptr6.cast_mut();
                    let flags7 = t3_1;
                    *ptr2.add(16).cast::<u8>() = ((flags7.bits() >> 0) as i32) as u8;
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_ecdsa_key_sign<T: GuestEcdsaKey>(
                    arg0: *mut u8,
                ) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let base2 = l0;
                    let len2 = l1;
                    _rt::cabi_dealloc(base2, len2 * 1, 1);
                    let l3 = *arg0.add(8).cast::<*mut u8>();
                    let l4 = *arg0.add(12).cast::<usize>();
                    let base5 = l3;
                    let len5 = l4;
                    _rt::cabi_dealloc(base5, len5 * 1, 1);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_ecdsa_key_sign_raw_cabi<T: GuestEcdsaKey>(
                    arg0: *mut u8,
                    arg1: *mut u8,
                    arg2: usize,
                    arg3: i32,
                    arg4: i32,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let result1 = T::sign_raw(
                        EcdsaKeyBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::Vec::from_raw_parts(arg1.cast(), len0, len0),
                        match arg3 {
                            0 => None,
                            1 => {
                                let e = arg4 as u32;
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        },
                    );
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let (t3_0, t3_1) = result1;
                    let Signature { r: r4, s: s4 } = t3_0;
                    let vec5 = (r4).into_boxed_slice();
                    let ptr5 = vec5.as_ptr().cast::<u8>();
                    let len5 = vec5.len();
                    ::core::mem::forget(vec5);
                    *ptr2.add(4).cast::<usize>() = len5;
                    *ptr2.add(0).cast::<*mut u8>() = ptr5.cast_mut();
                    let vec6 = (s4).into_boxed_slice();
                    let ptr6 = vec6.as_ptr().cast::<u8>();
                    let len6 = vec6.len();
                    ::core::mem::forget(vec6);
                    *ptr2.add(12).cast::<usize>() = len6;
                    *ptr2.add(8).cast::<*mut u8>() = ptr6.cast_mut();
                    let flags7 = t3_1;
                    *ptr2.add(16).cast::<u8>() = ((flags7.bits() >> 0) as i32) as u8;
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_ecdsa_key_sign_raw<T: GuestEcdsaKey>(
                    arg0: *mut u8,
                ) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let base2 = l0;
                    let len2 = l1;
                    _rt::cabi_dealloc(base2, len2 * 1, 1);
                    let l3 = *arg0.add(8).cast::<*mut u8>();
                    let l4 = *arg0.add(12).cast::<usize>();
                    let base5 = l3;
                    let len5 = l4;
                    _rt::cabi_dealloc(base5, len5 * 1, 1);
                }
                pub trait Guest {
                    type EcdsaKey: GuestEcdsaKey;
                }
                pub trait GuestEcdsaKey: 'static {
                    #[doc(hidden)]
                    unsafe fn _resource_new(val: *mut u8) -> u32
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = val;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]mrnerdhair:isolated-crypto/secp256k1@0.14.0"
                            )]
                            extern "C" {
                                #[link_name = "[resource-new]ecdsa-key"]
                                fn new(_: *mut u8) -> u32;
                            }
                            new(val)
                        }
                    }
                    #[doc(hidden)]
                    fn _resource_rep(handle: u32) -> *mut u8
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = handle;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]mrnerdhair:isolated-crypto/secp256k1@0.14.0"
                            )]
                            extern "C" {
                                #[link_name = "[resource-rep]ecdsa-key"]
                                fn rep(_: u32) -> *mut u8;
                            }
                            unsafe { rep(handle) }
                        }
                    }
                    fn get_public_key(&self) -> CompressedPoint;
                    fn sign(
                        &self,
                        digest_algorithm: DigestAlgorithm256,
                        message: _rt::Vec<u8>,
                        counter: Option<u32>,
                    ) -> (Signature, RecoveryId);
                    fn sign_raw(
                        &self,
                        digest: U256be,
                        counter: Option<u32>,
                    ) -> (Signature, RecoveryId);
                }
                #[doc(hidden)]
                macro_rules! __export_mrnerdhair_isolated_crypto_secp256k1_0_14_0_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "mrnerdhair:isolated-crypto/secp256k1@0.14.0#[method]ecdsa-key.get-public-key"]
                        unsafe extern "C" fn export_method_ecdsa_key_get_public_key(arg0
                        : * mut u8,) -> * mut u8 { $($path_to_types)*::
                        _export_method_ecdsa_key_get_public_key_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::EcdsaKey > (arg0) } #[export_name =
                        "cabi_post_mrnerdhair:isolated-crypto/secp256k1@0.14.0#[method]ecdsa-key.get-public-key"]
                        unsafe extern "C" fn
                        _post_return_method_ecdsa_key_get_public_key(arg0 : * mut u8,) {
                        $($path_to_types)*::
                        __post_return_method_ecdsa_key_get_public_key::<<$ty as
                        $($path_to_types)*:: Guest >::EcdsaKey > (arg0) } #[export_name =
                        "mrnerdhair:isolated-crypto/secp256k1@0.14.0#[method]ecdsa-key.sign"]
                        unsafe extern "C" fn export_method_ecdsa_key_sign(arg0 : * mut
                        u8, arg1 : i32, arg2 : * mut u8, arg3 : usize, arg4 : i32, arg5 :
                        i32,) -> * mut u8 { $($path_to_types)*::
                        _export_method_ecdsa_key_sign_cabi::<<$ty as $($path_to_types)*::
                        Guest >::EcdsaKey > (arg0, arg1, arg2, arg3, arg4, arg5) }
                        #[export_name =
                        "cabi_post_mrnerdhair:isolated-crypto/secp256k1@0.14.0#[method]ecdsa-key.sign"]
                        unsafe extern "C" fn _post_return_method_ecdsa_key_sign(arg0 : *
                        mut u8,) { $($path_to_types)*::
                        __post_return_method_ecdsa_key_sign::<<$ty as
                        $($path_to_types)*:: Guest >::EcdsaKey > (arg0) } #[export_name =
                        "mrnerdhair:isolated-crypto/secp256k1@0.14.0#[method]ecdsa-key.sign-raw"]
                        unsafe extern "C" fn export_method_ecdsa_key_sign_raw(arg0 : *
                        mut u8, arg1 : * mut u8, arg2 : usize, arg3 : i32, arg4 : i32,)
                        -> * mut u8 { $($path_to_types)*::
                        _export_method_ecdsa_key_sign_raw_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::EcdsaKey > (arg0, arg1, arg2, arg3,
                        arg4) } #[export_name =
                        "cabi_post_mrnerdhair:isolated-crypto/secp256k1@0.14.0#[method]ecdsa-key.sign-raw"]
                        unsafe extern "C" fn _post_return_method_ecdsa_key_sign_raw(arg0
                        : * mut u8,) { $($path_to_types)*::
                        __post_return_method_ecdsa_key_sign_raw::<<$ty as
                        $($path_to_types)*:: Guest >::EcdsaKey > (arg0) } const _ : () =
                        { #[doc(hidden)] #[export_name =
                        "mrnerdhair:isolated-crypto/secp256k1@0.14.0#[dtor]ecdsa-key"]
                        #[allow(non_snake_case)] unsafe extern "C" fn dtor(rep : * mut
                        u8) { $($path_to_types)*:: EcdsaKey::dtor::< <$ty as
                        $($path_to_types)*:: Guest >::EcdsaKey > (rep) } }; };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_mrnerdhair_isolated_crypto_secp256k1_0_14_0_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 20]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 20],
                );
            }
            #[allow(dead_code, clippy::all)]
            pub mod bip32 {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type U256be = super::super::super::super::exports::mrnerdhair::isolated_crypto::types::U256be;
                pub type CompressedPoint = super::super::super::super::exports::mrnerdhair::isolated_crypto::secp256k1::CompressedPoint;
                pub type EcdsaKey = super::super::super::super::exports::mrnerdhair::isolated_crypto::secp256k1::EcdsaKey;
                pub type EcdsaKeyBorrow<'a> = super::super::super::super::exports::mrnerdhair::isolated_crypto::secp256k1::EcdsaKeyBorrow<
                    'a,
                >;
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct Seed {
                    handle: _rt::Resource<Seed>,
                }
                type _SeedRep<T> = Option<T>;
                impl Seed {
                    /// Creates a new resource from the specified representation.
                    ///
                    /// This function will create a new resource handle by moving `val` onto
                    /// the heap and then passing that heap pointer to the component model to
                    /// create a handle. The owned handle is then returned as `Seed`.
                    pub fn new<T: GuestSeed>(val: T) -> Self {
                        Self::type_guard::<T>();
                        let val: _SeedRep<T> = Some(val);
                        let ptr: *mut _SeedRep<T> = _rt::Box::into_raw(
                            _rt::Box::new(val),
                        );
                        unsafe { Self::from_handle(T::_resource_new(ptr.cast())) }
                    }
                    /// Gets access to the underlying `T` which represents this resource.
                    pub fn get<T: GuestSeed>(&self) -> &T {
                        let ptr = unsafe { &*self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    /// Gets mutable access to the underlying `T` which represents this
                    /// resource.
                    pub fn get_mut<T: GuestSeed>(&mut self) -> &mut T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_mut().unwrap()
                    }
                    /// Consumes this resource and returns the underlying `T`.
                    pub fn into_inner<T: GuestSeed>(self) -> T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.take().unwrap()
                    }
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
                    #[doc(hidden)]
                    fn type_guard<T: 'static>() {
                        use core::any::TypeId;
                        static mut LAST_TYPE: Option<TypeId> = None;
                        unsafe {
                            assert!(! cfg!(target_feature = "atomics"));
                            let id = TypeId::of::<T>();
                            match LAST_TYPE {
                                Some(ty) => {
                                    assert!(
                                        ty == id, "cannot use two types with this resource type"
                                    )
                                }
                                None => LAST_TYPE = Some(id),
                            }
                        }
                    }
                    #[doc(hidden)]
                    pub unsafe fn dtor<T: 'static>(handle: *mut u8) {
                        Self::type_guard::<T>();
                        let _ = _rt::Box::from_raw(handle as *mut _SeedRep<T>);
                    }
                    fn as_ptr<T: GuestSeed>(&self) -> *mut _SeedRep<T> {
                        Seed::type_guard::<T>();
                        T::_resource_rep(self.handle()).cast()
                    }
                }
                /// A borrowed version of [`Seed`] which represents a borrowed value
                /// with the lifetime `'a`.
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct SeedBorrow<'a> {
                    rep: *mut u8,
                    _marker: core::marker::PhantomData<&'a Seed>,
                }
                impl<'a> SeedBorrow<'a> {
                    #[doc(hidden)]
                    pub unsafe fn lift(rep: usize) -> Self {
                        Self {
                            rep: rep as *mut u8,
                            _marker: core::marker::PhantomData,
                        }
                    }
                    /// Gets access to the underlying `T` in this resource.
                    pub fn get<T: GuestSeed>(&self) -> &T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    fn as_ptr<T: 'static>(&self) -> *mut _SeedRep<T> {
                        Seed::type_guard::<T>();
                        self.rep.cast()
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
                                wasm_import_module = "[export]mrnerdhair:isolated-crypto/bip32@0.14.0"
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
                type _NodeRep<T> = Option<T>;
                impl Node {
                    /// Creates a new resource from the specified representation.
                    ///
                    /// This function will create a new resource handle by moving `val` onto
                    /// the heap and then passing that heap pointer to the component model to
                    /// create a handle. The owned handle is then returned as `Node`.
                    pub fn new<T: GuestNode>(val: T) -> Self {
                        Self::type_guard::<T>();
                        let val: _NodeRep<T> = Some(val);
                        let ptr: *mut _NodeRep<T> = _rt::Box::into_raw(
                            _rt::Box::new(val),
                        );
                        unsafe { Self::from_handle(T::_resource_new(ptr.cast())) }
                    }
                    /// Gets access to the underlying `T` which represents this resource.
                    pub fn get<T: GuestNode>(&self) -> &T {
                        let ptr = unsafe { &*self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    /// Gets mutable access to the underlying `T` which represents this
                    /// resource.
                    pub fn get_mut<T: GuestNode>(&mut self) -> &mut T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_mut().unwrap()
                    }
                    /// Consumes this resource and returns the underlying `T`.
                    pub fn into_inner<T: GuestNode>(self) -> T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.take().unwrap()
                    }
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
                    #[doc(hidden)]
                    fn type_guard<T: 'static>() {
                        use core::any::TypeId;
                        static mut LAST_TYPE: Option<TypeId> = None;
                        unsafe {
                            assert!(! cfg!(target_feature = "atomics"));
                            let id = TypeId::of::<T>();
                            match LAST_TYPE {
                                Some(ty) => {
                                    assert!(
                                        ty == id, "cannot use two types with this resource type"
                                    )
                                }
                                None => LAST_TYPE = Some(id),
                            }
                        }
                    }
                    #[doc(hidden)]
                    pub unsafe fn dtor<T: 'static>(handle: *mut u8) {
                        Self::type_guard::<T>();
                        let _ = _rt::Box::from_raw(handle as *mut _NodeRep<T>);
                    }
                    fn as_ptr<T: GuestNode>(&self) -> *mut _NodeRep<T> {
                        Node::type_guard::<T>();
                        T::_resource_rep(self.handle()).cast()
                    }
                }
                /// A borrowed version of [`Node`] which represents a borrowed value
                /// with the lifetime `'a`.
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct NodeBorrow<'a> {
                    rep: *mut u8,
                    _marker: core::marker::PhantomData<&'a Node>,
                }
                impl<'a> NodeBorrow<'a> {
                    #[doc(hidden)]
                    pub unsafe fn lift(rep: usize) -> Self {
                        Self {
                            rep: rep as *mut u8,
                            _marker: core::marker::PhantomData,
                        }
                    }
                    /// Gets access to the underlying `T` in this resource.
                    pub fn get<T: GuestNode>(&self) -> &T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    fn as_ptr<T: 'static>(&self) -> *mut _NodeRep<T> {
                        Node::type_guard::<T>();
                        self.rep.cast()
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
                                wasm_import_module = "[export]mrnerdhair:isolated-crypto/bip32@0.14.0"
                            )]
                            extern "C" {
                                #[link_name = "[resource-drop]node"]
                                fn drop(_: u32);
                            }
                            drop(_handle);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_seed_to_master_key_cabi<T: GuestSeed>(
                    arg0: *mut u8,
                    arg1: i32,
                    arg2: *mut u8,
                    arg3: usize,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result1 = T::to_master_key(
                        SeedBorrow::lift(arg0 as u32 as usize).get(),
                        match arg1 {
                            0 => None,
                            1 => {
                                let e = {
                                    let len0 = arg3;
                                    _rt::Vec::from_raw_parts(arg2.cast(), len0, len0)
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        },
                    );
                    (result1).take_handle() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_node_get_public_key_cabi<T: GuestNode>(
                    arg0: *mut u8,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_public_key(
                        NodeBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let super::super::super::super::exports::mrnerdhair::isolated_crypto::secp256k1::CompressedPoint {
                        x: x2,
                        is_y_odd: is_y_odd2,
                    } = result0;
                    let vec3 = (x2).into_boxed_slice();
                    let ptr3 = vec3.as_ptr().cast::<u8>();
                    let len3 = vec3.len();
                    ::core::mem::forget(vec3);
                    *ptr1.add(4).cast::<usize>() = len3;
                    *ptr1.add(0).cast::<*mut u8>() = ptr3.cast_mut();
                    *ptr1.add(8).cast::<u8>() = (match is_y_odd2 {
                        true => 1,
                        false => 0,
                    }) as u8;
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_node_get_public_key<T: GuestNode>(
                    arg0: *mut u8,
                ) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let base2 = l0;
                    let len2 = l1;
                    _rt::cabi_dealloc(base2, len2 * 1, 1);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_node_get_chain_code_cabi<T: GuestNode>(
                    arg0: *mut u8,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_chain_code(
                        NodeBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec2 = (result0).into_boxed_slice();
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();
                    ::core::mem::forget(vec2);
                    *ptr1.add(4).cast::<usize>() = len2;
                    *ptr1.add(0).cast::<*mut u8>() = ptr2.cast_mut();
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_node_get_chain_code<T: GuestNode>(
                    arg0: *mut u8,
                ) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let base2 = l0;
                    let len2 = l1;
                    _rt::cabi_dealloc(base2, len2 * 1, 1);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_node_derive_cabi<T: GuestNode>(
                    arg0: *mut u8,
                    arg1: i32,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::derive(
                        NodeBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                    (result0).take_handle() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_node_clone_cabi<T: GuestNode>(
                    arg0: *mut u8,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::clone(NodeBorrow::lift(arg0 as u32 as usize).get());
                    (result0).take_handle() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_node_into_secp256k1_ecdsa_key_cabi<
                    T: GuestNode,
                >(arg0: *mut u8) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::into_secp256k1_ecdsa_key(
                        NodeBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    (result0).take_handle() as i32
                }
                pub trait Guest {
                    type Seed: GuestSeed;
                    type Node: GuestNode;
                }
                pub trait GuestSeed: 'static {
                    #[doc(hidden)]
                    unsafe fn _resource_new(val: *mut u8) -> u32
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = val;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]mrnerdhair:isolated-crypto/bip32@0.14.0"
                            )]
                            extern "C" {
                                #[link_name = "[resource-new]seed"]
                                fn new(_: *mut u8) -> u32;
                            }
                            new(val)
                        }
                    }
                    #[doc(hidden)]
                    fn _resource_rep(handle: u32) -> *mut u8
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = handle;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]mrnerdhair:isolated-crypto/bip32@0.14.0"
                            )]
                            extern "C" {
                                #[link_name = "[resource-rep]seed"]
                                fn rep(_: u32) -> *mut u8;
                            }
                            unsafe { rep(handle) }
                        }
                    }
                    fn to_master_key(&self, hmac_key: Option<_rt::Vec<u8>>) -> Node;
                }
                pub trait GuestNode: 'static {
                    #[doc(hidden)]
                    unsafe fn _resource_new(val: *mut u8) -> u32
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = val;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]mrnerdhair:isolated-crypto/bip32@0.14.0"
                            )]
                            extern "C" {
                                #[link_name = "[resource-new]node"]
                                fn new(_: *mut u8) -> u32;
                            }
                            new(val)
                        }
                    }
                    #[doc(hidden)]
                    fn _resource_rep(handle: u32) -> *mut u8
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = handle;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]mrnerdhair:isolated-crypto/bip32@0.14.0"
                            )]
                            extern "C" {
                                #[link_name = "[resource-rep]node"]
                                fn rep(_: u32) -> *mut u8;
                            }
                            unsafe { rep(handle) }
                        }
                    }
                    fn get_public_key(&self) -> CompressedPoint;
                    fn get_chain_code(&self) -> U256be;
                    fn derive(&self, index: u32) -> Node;
                    fn clone(&self) -> Node;
                    fn into_secp256k1_ecdsa_key(&self) -> EcdsaKey;
                }
                #[doc(hidden)]
                macro_rules! __export_mrnerdhair_isolated_crypto_bip32_0_14_0_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "mrnerdhair:isolated-crypto/bip32@0.14.0#[method]seed.to-master-key"]
                        unsafe extern "C" fn export_method_seed_to_master_key(arg0 : *
                        mut u8, arg1 : i32, arg2 : * mut u8, arg3 : usize,) -> i32 {
                        $($path_to_types)*::
                        _export_method_seed_to_master_key_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Seed > (arg0, arg1, arg2, arg3) }
                        #[export_name =
                        "mrnerdhair:isolated-crypto/bip32@0.14.0#[method]node.get-public-key"]
                        unsafe extern "C" fn export_method_node_get_public_key(arg0 : *
                        mut u8,) -> * mut u8 { $($path_to_types)*::
                        _export_method_node_get_public_key_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Node > (arg0) } #[export_name =
                        "cabi_post_mrnerdhair:isolated-crypto/bip32@0.14.0#[method]node.get-public-key"]
                        unsafe extern "C" fn _post_return_method_node_get_public_key(arg0
                        : * mut u8,) { $($path_to_types)*::
                        __post_return_method_node_get_public_key::<<$ty as
                        $($path_to_types)*:: Guest >::Node > (arg0) } #[export_name =
                        "mrnerdhair:isolated-crypto/bip32@0.14.0#[method]node.get-chain-code"]
                        unsafe extern "C" fn export_method_node_get_chain_code(arg0 : *
                        mut u8,) -> * mut u8 { $($path_to_types)*::
                        _export_method_node_get_chain_code_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Node > (arg0) } #[export_name =
                        "cabi_post_mrnerdhair:isolated-crypto/bip32@0.14.0#[method]node.get-chain-code"]
                        unsafe extern "C" fn _post_return_method_node_get_chain_code(arg0
                        : * mut u8,) { $($path_to_types)*::
                        __post_return_method_node_get_chain_code::<<$ty as
                        $($path_to_types)*:: Guest >::Node > (arg0) } #[export_name =
                        "mrnerdhair:isolated-crypto/bip32@0.14.0#[method]node.derive"]
                        unsafe extern "C" fn export_method_node_derive(arg0 : * mut u8,
                        arg1 : i32,) -> i32 { $($path_to_types)*::
                        _export_method_node_derive_cabi::<<$ty as $($path_to_types)*::
                        Guest >::Node > (arg0, arg1) } #[export_name =
                        "mrnerdhair:isolated-crypto/bip32@0.14.0#[method]node.clone"]
                        unsafe extern "C" fn export_method_node_clone(arg0 : * mut u8,)
                        -> i32 { $($path_to_types)*::
                        _export_method_node_clone_cabi::<<$ty as $($path_to_types)*::
                        Guest >::Node > (arg0) } #[export_name =
                        "mrnerdhair:isolated-crypto/bip32@0.14.0#[method]node.into-secp256k1-ecdsa-key"]
                        unsafe extern "C" fn
                        export_method_node_into_secp256k1_ecdsa_key(arg0 : * mut u8,) ->
                        i32 { $($path_to_types)*::
                        _export_method_node_into_secp256k1_ecdsa_key_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Node > (arg0) } const _ : () = {
                        #[doc(hidden)] #[export_name =
                        "mrnerdhair:isolated-crypto/bip32@0.14.0#[dtor]seed"]
                        #[allow(non_snake_case)] unsafe extern "C" fn dtor(rep : * mut
                        u8) { $($path_to_types)*:: Seed::dtor::< <$ty as
                        $($path_to_types)*:: Guest >::Seed > (rep) } }; const _ : () = {
                        #[doc(hidden)] #[export_name =
                        "mrnerdhair:isolated-crypto/bip32@0.14.0#[dtor]node"]
                        #[allow(non_snake_case)] unsafe extern "C" fn dtor(rep : * mut
                        u8) { $($path_to_types)*:: Node::dtor::< <$ty as
                        $($path_to_types)*:: Guest >::Node > (rep) } }; };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_mrnerdhair_isolated_crypto_bip32_0_14_0_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 12]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 12],
                );
            }
            #[allow(dead_code, clippy::all)]
            pub mod bip39 {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type Seed = super::super::super::super::exports::mrnerdhair::isolated_crypto::bip32::Seed;
                pub type SeedBorrow<'a> = super::super::super::super::exports::mrnerdhair::isolated_crypto::bip32::SeedBorrow<
                    'a,
                >;
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct Mnemonic {
                    handle: _rt::Resource<Mnemonic>,
                }
                type _MnemonicRep<T> = Option<T>;
                impl Mnemonic {
                    /// Creates a new resource from the specified representation.
                    ///
                    /// This function will create a new resource handle by moving `val` onto
                    /// the heap and then passing that heap pointer to the component model to
                    /// create a handle. The owned handle is then returned as `Mnemonic`.
                    pub fn new<T: GuestMnemonic>(val: T) -> Self {
                        Self::type_guard::<T>();
                        let val: _MnemonicRep<T> = Some(val);
                        let ptr: *mut _MnemonicRep<T> = _rt::Box::into_raw(
                            _rt::Box::new(val),
                        );
                        unsafe { Self::from_handle(T::_resource_new(ptr.cast())) }
                    }
                    /// Gets access to the underlying `T` which represents this resource.
                    pub fn get<T: GuestMnemonic>(&self) -> &T {
                        let ptr = unsafe { &*self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    /// Gets mutable access to the underlying `T` which represents this
                    /// resource.
                    pub fn get_mut<T: GuestMnemonic>(&mut self) -> &mut T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_mut().unwrap()
                    }
                    /// Consumes this resource and returns the underlying `T`.
                    pub fn into_inner<T: GuestMnemonic>(self) -> T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.take().unwrap()
                    }
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
                    #[doc(hidden)]
                    fn type_guard<T: 'static>() {
                        use core::any::TypeId;
                        static mut LAST_TYPE: Option<TypeId> = None;
                        unsafe {
                            assert!(! cfg!(target_feature = "atomics"));
                            let id = TypeId::of::<T>();
                            match LAST_TYPE {
                                Some(ty) => {
                                    assert!(
                                        ty == id, "cannot use two types with this resource type"
                                    )
                                }
                                None => LAST_TYPE = Some(id),
                            }
                        }
                    }
                    #[doc(hidden)]
                    pub unsafe fn dtor<T: 'static>(handle: *mut u8) {
                        Self::type_guard::<T>();
                        let _ = _rt::Box::from_raw(handle as *mut _MnemonicRep<T>);
                    }
                    fn as_ptr<T: GuestMnemonic>(&self) -> *mut _MnemonicRep<T> {
                        Mnemonic::type_guard::<T>();
                        T::_resource_rep(self.handle()).cast()
                    }
                }
                /// A borrowed version of [`Mnemonic`] which represents a borrowed value
                /// with the lifetime `'a`.
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct MnemonicBorrow<'a> {
                    rep: *mut u8,
                    _marker: core::marker::PhantomData<&'a Mnemonic>,
                }
                impl<'a> MnemonicBorrow<'a> {
                    #[doc(hidden)]
                    pub unsafe fn lift(rep: usize) -> Self {
                        Self {
                            rep: rep as *mut u8,
                            _marker: core::marker::PhantomData,
                        }
                    }
                    /// Gets access to the underlying `T` in this resource.
                    pub fn get<T: GuestMnemonic>(&self) -> &T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    fn as_ptr<T: 'static>(&self) -> *mut _MnemonicRep<T> {
                        Mnemonic::type_guard::<T>();
                        self.rep.cast()
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
                                wasm_import_module = "[export]mrnerdhair:isolated-crypto/bip39@0.14.0"
                            )]
                            extern "C" {
                                #[link_name = "[resource-drop]mnemonic"]
                                fn drop(_: u32);
                            }
                            drop(_handle);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_static_mnemonic_new_cabi<T: GuestMnemonic>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let result1 = T::new(_rt::string_lift(bytes0));
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result1 {
                        Ok(e) => {
                            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                            *ptr2.add(4).cast::<i32>() = (e).take_handle() as i32;
                        }
                        Err(e) => {
                            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                            let vec3 = (e.into_bytes()).into_boxed_slice();
                            let ptr3 = vec3.as_ptr().cast::<u8>();
                            let len3 = vec3.len();
                            ::core::mem::forget(vec3);
                            *ptr2.add(8).cast::<usize>() = len3;
                            *ptr2.add(4).cast::<*mut u8>() = ptr3.cast_mut();
                        }
                    };
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_static_mnemonic_new<T: GuestMnemonic>(
                    arg0: *mut u8,
                ) {
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
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_mnemonic_to_seed_cabi<T: GuestMnemonic>(
                    arg0: *mut u8,
                    arg1: *mut u8,
                    arg2: usize,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let bytes0 = _rt::Vec::from_raw_parts(arg1.cast(), len0, len0);
                    let result1 = T::to_seed(
                        MnemonicBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::string_lift(bytes0),
                    );
                    (result1).take_handle() as i32
                }
                pub trait Guest {
                    type Mnemonic: GuestMnemonic;
                }
                pub trait GuestMnemonic: 'static {
                    #[doc(hidden)]
                    unsafe fn _resource_new(val: *mut u8) -> u32
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = val;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]mrnerdhair:isolated-crypto/bip39@0.14.0"
                            )]
                            extern "C" {
                                #[link_name = "[resource-new]mnemonic"]
                                fn new(_: *mut u8) -> u32;
                            }
                            new(val)
                        }
                    }
                    #[doc(hidden)]
                    fn _resource_rep(handle: u32) -> *mut u8
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = handle;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]mrnerdhair:isolated-crypto/bip39@0.14.0"
                            )]
                            extern "C" {
                                #[link_name = "[resource-rep]mnemonic"]
                                fn rep(_: u32) -> *mut u8;
                            }
                            unsafe { rep(handle) }
                        }
                    }
                    fn new(mnemonic: _rt::String) -> Result<Mnemonic, _rt::String>;
                    fn to_seed(&self, passphrase: _rt::String) -> Seed;
                }
                #[doc(hidden)]
                macro_rules! __export_mrnerdhair_isolated_crypto_bip39_0_14_0_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "mrnerdhair:isolated-crypto/bip39@0.14.0#[static]mnemonic.new"]
                        unsafe extern "C" fn export_static_mnemonic_new(arg0 : * mut u8,
                        arg1 : usize,) -> * mut u8 { $($path_to_types)*::
                        _export_static_mnemonic_new_cabi::<<$ty as $($path_to_types)*::
                        Guest >::Mnemonic > (arg0, arg1) } #[export_name =
                        "cabi_post_mrnerdhair:isolated-crypto/bip39@0.14.0#[static]mnemonic.new"]
                        unsafe extern "C" fn _post_return_static_mnemonic_new(arg0 : *
                        mut u8,) { $($path_to_types)*::
                        __post_return_static_mnemonic_new::<<$ty as $($path_to_types)*::
                        Guest >::Mnemonic > (arg0) } #[export_name =
                        "mrnerdhair:isolated-crypto/bip39@0.14.0#[method]mnemonic.to-seed"]
                        unsafe extern "C" fn export_method_mnemonic_to_seed(arg0 : * mut
                        u8, arg1 : * mut u8, arg2 : usize,) -> i32 { $($path_to_types)*::
                        _export_method_mnemonic_to_seed_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Mnemonic > (arg0, arg1, arg2) }
                        const _ : () = { #[doc(hidden)] #[export_name =
                        "mrnerdhair:isolated-crypto/bip39@0.14.0#[dtor]mnemonic"]
                        #[allow(non_snake_case)] unsafe extern "C" fn dtor(rep : * mut
                        u8) { $($path_to_types)*:: Mnemonic::dtor::< <$ty as
                        $($path_to_types)*:: Guest >::Mnemonic > (rep) } }; };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_mrnerdhair_isolated_crypto_bip39_0_14_0_cabi;
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
    pub use alloc_crate::boxed::Box;
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
    pub unsafe fn invalid_enum_discriminant<T>() -> T {
        if cfg!(debug_assertions) {
            panic!("invalid enum discriminant")
        } else {
            core::hint::unreachable_unchecked()
        }
    }
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub use alloc_crate::string::String;
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
macro_rules! __export_engine_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::mrnerdhair::isolated_crypto::types::__export_mrnerdhair_isolated_crypto_types_0_14_0_cabi!($ty
        with_types_in $($path_to_types_root)*::
        exports::mrnerdhair::isolated_crypto::types); $($path_to_types_root)*::
        exports::mrnerdhair::isolated_crypto::secp256k1::__export_mrnerdhair_isolated_crypto_secp256k1_0_14_0_cabi!($ty
        with_types_in $($path_to_types_root)*::
        exports::mrnerdhair::isolated_crypto::secp256k1); $($path_to_types_root)*::
        exports::mrnerdhair::isolated_crypto::bip32::__export_mrnerdhair_isolated_crypto_bip32_0_14_0_cabi!($ty
        with_types_in $($path_to_types_root)*::
        exports::mrnerdhair::isolated_crypto::bip32); $($path_to_types_root)*::
        exports::mrnerdhair::isolated_crypto::bip39::__export_mrnerdhair_isolated_crypto_bip39_0_14_0_cabi!($ty
        with_types_in $($path_to_types_root)*::
        exports::mrnerdhair::isolated_crypto::bip39);
    };
}
#[doc(inline)]
pub(crate) use __export_engine_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.30.0:engine:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 1369] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xdc\x09\x01A\x02\x01\
A\x0d\x01B\x04\x01p}\x04\0\x06u256be\x03\0\0\x01m\x02\x06sha256\x09keccak256\x04\
\0\x13digest-algorithm256\x03\0\x02\x04\x01'mrnerdhair:isolated-crypto/types@0.1\
4.0\x05\0\x02\x03\0\0\x06u256be\x02\x03\0\0\x13digest-algorithm256\x01B\x15\x02\x03\
\x02\x01\x01\x04\0\x06u256be\x03\0\0\x02\x03\x02\x01\x02\x04\0\x13digest-algorit\
hm256\x03\0\x02\x01r\x02\x01x\x01\x08is-y-odd\x7f\x04\0\x10compressed-point\x03\0\
\x04\x01n\x02\x08is-y-odd\x0cis-x-reduced\x04\0\x0brecovery-id\x03\0\x06\x01r\x02\
\x01r\x01\x01s\x01\x04\0\x09signature\x03\0\x08\x04\0\x09ecdsa-key\x03\x01\x01h\x0a\
\x01@\x01\x04self\x0b\0\x05\x04\0\x20[method]ecdsa-key.get-public-key\x01\x0c\x01\
p}\x01ky\x01o\x02\x09\x07\x01@\x04\x04self\x0b\x10digest-algorithm\x03\x07messag\
e\x0d\x07counter\x0e\0\x0f\x04\0\x16[method]ecdsa-key.sign\x01\x10\x01@\x03\x04s\
elf\x0b\x06digest\x01\x07counter\x0e\0\x0f\x04\0\x1a[method]ecdsa-key.sign-raw\x01\
\x11\x04\x01+mrnerdhair:isolated-crypto/secp256k1@0.14.0\x05\x03\x02\x03\0\x01\x10\
compressed-point\x02\x03\0\x01\x09ecdsa-key\x01B\x1a\x02\x03\x02\x01\x01\x04\0\x06\
u256be\x03\0\0\x02\x03\x02\x01\x04\x04\0\x10compressed-point\x03\0\x02\x02\x03\x02\
\x01\x05\x04\0\x09ecdsa-key\x03\0\x04\x04\0\x04seed\x03\x01\x04\0\x04node\x03\x01\
\x01h\x06\x01p}\x01k\x09\x01i\x07\x01@\x02\x04self\x08\x08hmac-key\x0a\0\x0b\x04\
\0\x1a[method]seed.to-master-key\x01\x0c\x01h\x07\x01@\x01\x04self\x0d\0\x03\x04\
\0\x1b[method]node.get-public-key\x01\x0e\x01@\x01\x04self\x0d\0\x01\x04\0\x1b[m\
ethod]node.get-chain-code\x01\x0f\x01@\x02\x04self\x0d\x05indexy\0\x0b\x04\0\x13\
[method]node.derive\x01\x10\x01@\x01\x04self\x0d\0\x0b\x04\0\x12[method]node.clo\
ne\x01\x11\x01i\x05\x01@\x01\x04self\x0d\0\x12\x04\0%[method]node.into-secp256k1\
-ecdsa-key\x01\x13\x04\x01'mrnerdhair:isolated-crypto/bip32@0.14.0\x05\x06\x02\x03\
\0\x02\x04seed\x01B\x0b\x02\x03\x02\x01\x07\x04\0\x04seed\x03\0\0\x04\0\x08mnemo\
nic\x03\x01\x01i\x02\x01j\x01\x03\x01s\x01@\x01\x08mnemonics\0\x04\x04\0\x14[sta\
tic]mnemonic.new\x01\x05\x01h\x02\x01i\x01\x01@\x02\x04self\x06\x0apassphrases\0\
\x07\x04\0\x18[method]mnemonic.to-seed\x01\x08\x04\x01'mrnerdhair:isolated-crypt\
o/bip39@0.14.0\x05\x08\x04\x01(mrnerdhair:isolated-crypto/engine@0.14.0\x04\0\x0b\
\x0c\x01\0\x06engine\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-comp\
onent\x070.215.0\x10wit-bindgen-rust\x060.30.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
