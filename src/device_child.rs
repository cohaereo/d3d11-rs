use windows::Win32::Graphics::{
    Direct3D::WKPDID_D3DDebugObjectName, Direct3D11::ID3D11DeviceChild,
};

use crate::device::Device;

pub trait DeviceChild {
    fn as_device_child(&self) -> &ID3D11DeviceChild;

    fn get_device(&self) -> Device {
        unsafe {
            let device = self.as_device_child().GetDevice().unwrap();
            Device(device)
        }
    }

    fn set_debug_name(&self, name: impl AsRef<str>) -> crate::Result<()> {
        let name_cstr = std::ffi::CString::new(name.as_ref()).unwrap();
        unsafe {
            self.as_device_child().SetPrivateData(
                &WKPDID_D3DDebugObjectName,
                name_cstr.to_bytes().len() as _,
                Some(name_cstr.as_ptr() as _),
            )?;
        }

        Ok(())
    }
}

#[macro_export]
macro_rules! impl_device_child {
    ($name:ident) => {
        impl crate::device_child::DeviceChild for $name {
            fn as_device_child(&self) -> &ID3D11DeviceChild {
                &self.0
            }
        }
    };
}
