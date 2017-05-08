/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RefPtr<T> {
    pub use_of_t: T,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl <T> Default for RefPtr<T> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UsesRefPtrWithAliasedTypeParam<U> {
    pub member: RefPtr<UsesRefPtrWithAliasedTypeParam_V<U>>,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<U>>,
}
pub type UsesRefPtrWithAliasedTypeParam_V<U> = U;
impl <U> Default for UsesRefPtrWithAliasedTypeParam<U> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
