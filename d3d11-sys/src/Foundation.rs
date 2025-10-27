#![rustfmt::skip]

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HANDLE(pub *mut core::ffi::c_void);
impl HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
// impl windows_core::Free for HANDLE {
//     #[inline]
//     unsafe fn free(&mut self) {
//         if !self.is_invalid() {
//             windows_targets::link!("kernel32.dll" "system" fn CloseHandle(hobject : *mut core::ffi::c_void) -> i32);
//             unsafe {
//                 CloseHandle(self.0);
//             }
//         }
//     }
// }
impl Default for HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HMODULE(pub *mut core::ffi::c_void);
impl HMODULE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
// impl windows_core::Free for HMODULE {
//     #[inline]
//     unsafe fn free(&mut self) {
//         if !self.is_invalid() {
//             windows_targets::link!("kernel32.dll" "system" fn FreeLibrary(hlibmodule : *mut core::ffi::c_void) -> i32);
//             unsafe {
//                 FreeLibrary(self.0);
//             }
//         }
//     }
// }
impl Default for HMODULE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
// impl windows_core::imp::CanInto<HINSTANCE> for HMODULE {}
// impl From<HMODULE> for HINSTANCE {
//     fn from(value: HMODULE) -> Self {
//         Self(value.0)
//     }
// }

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LUID {
    pub LowPart: u32,
    pub HighPart: i32,
}
impl Default for LUID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[must_use]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct BOOL(pub i32);

impl BOOL {
    #[inline]
    pub fn as_bool(self) -> bool {
        self.0 != 0
    }
    #[inline]
    pub fn ok(self) -> windows_core::Result<()> {
        if self.as_bool() {
            Ok(())
        } else {
            Err(windows_core::Error::from_win32())
        }
    }
    #[inline]
    #[track_caller]
    pub fn unwrap(self) {
        self.ok().unwrap();
    }
    #[inline]
    #[track_caller]
    pub fn expect(self, msg: &str) {
        self.ok().expect(msg);
    }
}
impl From<BOOL> for bool {
    fn from(value: BOOL) -> Self {
        value.as_bool()
    }
}
impl From<&BOOL> for bool {
    fn from(value: &BOOL) -> Self {
        value.as_bool()
    }
}
impl From<bool> for BOOL {
    fn from(value: bool) -> Self {
        if value {
            Self(1)
        } else {
            Self(0)
        }
    }
}
impl From<&bool> for BOOL {
    fn from(value: &bool) -> Self {
        (*value).into()
    }
}
impl PartialEq<bool> for BOOL {
    fn eq(&self, other: &bool) -> bool {
        self.as_bool() == *other
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
impl Default for RECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SIZE {
    pub cx: i32,
    pub cy: i32,
}
impl Default for SIZE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}
impl Default for POINT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HWND(pub *mut core::ffi::c_void);
impl HWND {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HWND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::imp::CanInto<HANDLE> for HWND {}
impl From<HWND> for HANDLE {
    fn from(value: HWND) -> Self {
        Self(value.0)
    }
}
pub const S_FALSE: windows_core::HRESULT = windows_core::HRESULT(0x1_u32 as _);
pub const S_OK: windows_core::HRESULT = windows_core::HRESULT(0x0_u32 as _);
pub const E_INVALIDARG: windows_core::HRESULT = windows_core::HRESULT(0x80070057_u32 as _);
pub const E_FAIL: windows_core::HRESULT = windows_core::HRESULT(0x80004005_u32 as _);
pub const DXGI_DDI_ERR_NONEXCLUSIVE: windows_core::HRESULT = windows_core::HRESULT(0x887B0003_u32 as _);
pub const DXGI_DDI_ERR_UNSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x887B0002_u32 as _);
pub const DXGI_DDI_ERR_WASSTILLDRAWING: windows_core::HRESULT = windows_core::HRESULT(0x887B0001_u32 as _);
pub const DXGI_STATUS_CLIPPED: windows_core::HRESULT = windows_core::HRESULT(0x87A0002_u32 as _);
pub const DXGI_STATUS_DDA_WAS_STILL_DRAWING: windows_core::HRESULT = windows_core::HRESULT(0x87A000A_u32 as _);
pub const DXGI_STATUS_GRAPHICS_VIDPN_SOURCE_IN_USE: windows_core::HRESULT = windows_core::HRESULT(0x87A0006_u32 as _);
pub const DXGI_STATUS_MODE_CHANGED: windows_core::HRESULT = windows_core::HRESULT(0x87A0007_u32 as _);
pub const DXGI_STATUS_MODE_CHANGE_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0x87A0008_u32 as _);
pub const DXGI_STATUS_NO_DESKTOP_ACCESS: windows_core::HRESULT = windows_core::HRESULT(0x87A0005_u32 as _);
pub const DXGI_STATUS_NO_REDIRECTION: windows_core::HRESULT = windows_core::HRESULT(0x87A0004_u32 as _);
pub const DXGI_STATUS_OCCLUDED: windows_core::HRESULT = windows_core::HRESULT(0x87A0001_u32 as _);
pub const DXGI_STATUS_PRESENT_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x87A002F_u32 as _);
pub const DXGI_STATUS_UNOCCLUDED: windows_core::HRESULT = windows_core::HRESULT(0x87A0009_u32 as _);
