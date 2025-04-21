use d3d11_sys::{core::PWSTR, Direct3D11::ID3DUserDefinedAnnotation};

#[derive(Clone)]
pub struct UserDefinedAnnotation(pub(crate) ID3DUserDefinedAnnotation);
// impl_device_child!(UserDefinedAnnotation);

#[cfg_attr(feature = "profiling", profiling::all_functions)]
impl UserDefinedAnnotation {
    /// Returns the number of previous calls to BeginEvent that have not yet been finalized by calls to the UserDefinedAnnotation::end_event method.
    ///
    /// The return value is –1 if the calling application is not running under a Direct3D profiling tool.
    pub fn begin_event(&self, name: &str) -> i32 {
        let name_wide = name.encode_utf16().chain(Some(0)).collect::<Vec<_>>();
        let name_ptr = PWSTR::from_raw(name_wide.as_ptr() as _);
        unsafe { self.0.BeginEvent(name_ptr) }
    }

    /// Returns the number of previous calls to BeginEvent that have not yet been finalized by calls to the UserDefinedAnnotation::begin_event method.
    ///
    /// The return value is –1 if the calling application is not running under a Direct3D profiling tool.
    pub fn end_event(&self) -> i32 {
        unsafe { self.0.EndEvent() }
    }

    pub fn set_marker(&self, name: &str) {
        let name_wide = name.encode_utf16().chain(Some(0)).collect::<Vec<_>>();
        let name_ptr = PWSTR::from_raw(name_wide.as_ptr() as _);
        unsafe {
            self.0.SetMarker(name_ptr);
        }
    }
}
