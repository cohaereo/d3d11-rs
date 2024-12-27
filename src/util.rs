use std::ffi::CString;

use windows::core::PCSTR;

// Helper for working with DX11 methods that return results as HRESULT and out parameters as an Option<T>
pub fn wrap_option_out_result<T, F>(f: F) -> windows::core::Result<T>
where
    F: FnOnce(Option<*mut Option<T>>) -> windows::core::Result<()>,
{
    let mut result = None;
    f(Some(&raw mut result))?;
    Ok(result.unwrap())
}

pub fn wrap_out_result<T, F>(f: F) -> windows::core::Result<T>
where
    F: FnOnce(*mut Option<T>) -> windows::core::Result<()>,
{
    let mut result = None;
    f(&raw mut result)?;
    Ok(result.unwrap())
}

pub fn wrap_out_ptr<T: Default, F>(f: F) -> windows::core::Result<T>
where
    F: FnOnce(*mut T) -> windows::core::Result<()>,
{
    let mut result = T::default();
    f(&raw mut result)?;
    Ok(result)
}

/// Verifies that the given struct has the same size as the corresponding FFI struct, and generates an `as_ffi` method that returns a pointer to the FFI struct.
#[macro_export]
macro_rules! verify_ffi_struct {
    ($struct:ident, $ffi:ident) => {
        static_assertions::assert_eq_size!($struct, $ffi);

        impl $struct {
            #[allow(dead_code)]
            pub(crate) fn as_ffi(&self) -> *const $ffi {
                self as *const _ as _
            }
        }
    };
}

pub fn to_pcstr<S>(s: S) -> (CString, PCSTR)
where
    S: AsRef<str>,
{
    let cstr = CString::new(s.as_ref()).expect("Failed to convert string to CString");
    let pcstr = cstr.as_ptr();
    (cstr, PCSTR::from_raw(pcstr as _))
}

// pub fn to_pcwstr<S>(s: S) -> (Vec<u16>, PCWSTR)
// where
//     S: AsRef<str>,
// {
//     let wstr: Vec<u16> = s.as_ref().encode_utf16().chain(Some(0)).collect();
//     let pwstr = wstr.as_ptr();
//     (wstr, PCWSTR::from_raw(pwstr))
// }

pub trait OptionalParam: Sized {
    type Output;

    fn as_option(&self) -> Option<&Self::Output>;
}

impl<T> OptionalParam for &T {
    type Output = T;

    fn as_option(&self) -> Option<&Self::Output> {
        Some(self)
    }
}

impl<T> OptionalParam for Option<&T> {
    type Output = T;
    fn as_option(&self) -> Option<&Self::Output> {
        *self
    }
}
