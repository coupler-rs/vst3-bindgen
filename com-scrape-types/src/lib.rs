mod class;
mod ptr;

#[cfg(test)]
mod tests;

use std::ffi::c_void;

pub use class::{Class, ComWrapper, Construct, Header, InterfaceList, MakeHeader, Wrapper};
pub use ptr::{ComPtr, ComRef, SmartPtr};

pub type Guid = [u8; 16];

pub trait Unknown {
    unsafe fn query_interface(this: *mut Self, iid: &Guid) -> Option<*mut c_void>;
    unsafe fn add_ref(this: *mut Self) -> usize;
    unsafe fn release(this: *mut Self) -> usize;
}

pub unsafe trait Interface: Unknown {
    type Vtbl;

    const IID: Guid;

    fn inherits(iid: &Guid) -> bool;
}
pub unsafe trait Inherits<I: Interface>: Interface {}

unsafe impl<I: Interface> Inherits<I> for I {}
