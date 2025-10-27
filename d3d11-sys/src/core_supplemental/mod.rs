pub mod link;
pub mod literals;
pub use literals::*;
use windows_core::{CopyType, Param, ParamValue, TypeKind};

pub type BOOL = i32;
pub type HRESULT = i32;
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PSTR(pub *mut u8);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PWSTR(pub *mut u16);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PCSTR(pub *const u8);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PCWSTR(pub *const u16);

impl Param<PCWSTR> for PWSTR {
    unsafe fn param(self) -> ParamValue<PCWSTR> {
        ParamValue::Owned(PCWSTR(self.0))
    }
}
impl PCWSTR {
    /// Construct a new `PCWSTR` from a raw pointer
    pub const fn from_raw(ptr: *const u16) -> Self {
        Self(ptr)
    }
}
impl PWSTR {
    /// Construct a new `PWSTR` from a raw pointer.
    pub const fn from_raw(ptr: *mut u16) -> Self {
        Self(ptr)
    }
}

impl Param<PCSTR> for PSTR {
    unsafe fn param(self) -> ParamValue<PCSTR> {
        ParamValue::Owned(PCSTR(self.0))
    }
}

impl PCSTR {
    /// Construct a new `PCSTR` from a raw pointer
    pub const fn from_raw(ptr: *const u8) -> Self {
        Self(ptr)
    }

    pub const fn null() -> Self {
        Self(core::ptr::null())
    }
}

impl TypeKind for PWSTR {
    type TypeKind = CopyType;
}

impl TypeKind for PSTR {
    type TypeKind = CopyType;
}

impl TypeKind for PCWSTR {
    type TypeKind = CopyType;
}

impl TypeKind for PCSTR {
    type TypeKind = CopyType;
}
