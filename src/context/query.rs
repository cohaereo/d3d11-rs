use crate::query::Asynchronous;

use super::DeviceContext;

impl DeviceContext {
    pub fn begin(&self, async_: &impl Asynchronous) {
        unsafe {
            self.0.Begin(&async_.to_ffi_async());
        }
    }

    pub fn end(&self, async_: &impl Asynchronous) {
        unsafe {
            self.0.End(&async_.to_ffi_async());
        }
    }
}
