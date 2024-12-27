mod draw;
mod input_assembler;
mod output_merger;
mod query;
mod rasterizer;
mod shader;

use std::os::raw::c_void;

use bitflags::bitflags;
pub use input_assembler::PrimitiveTopology;
pub use rasterizer::Viewport;

use windows::{
    core::Interface,
    Win32::{
        Foundation::{S_FALSE, S_OK},
        Graphics::Direct3D11::*,
    },
};

use crate::{
    impl_device_child, query::Asynchronous, rtv::RenderTargetView, util::wrap_out_ptr,
    DepthStencilView, Resource,
};

#[derive(Clone)]
pub struct DeviceContext(pub(crate) ID3D11DeviceContext);
impl_device_child!(DeviceContext);

impl DeviceContext {
    pub fn clear_render_target_view(&self, rtv: &RenderTargetView, color: &[f32; 4]) {
        unsafe {
            self.0.ClearRenderTargetView(&rtv.0, color);
        }
    }

    pub fn clear_depth_stencil_view(
        &self,
        dsv: &DepthStencilView,
        flags: ClearFlags,
        depth: f32,
        stencil: u8,
    ) {
        unsafe {
            self.0
                .ClearDepthStencilView(&dsv.0, flags.bits(), depth, stencil);
        }
    }

    pub fn flush(&self) {
        unsafe {
            self.0.Flush();
        }
    }

    // TODO(cohae): consider a safer alternative to messing around with raw pointers?
    // eg. split this to map_unchecked, add map_checked which maps the region to a u8 slice

    /// Maps a resource to a CPU-accessible memory region.
    ///
    /// SAFETY: This function is safe to call, but operations on the mapped memory region are not.
    pub fn map<T: Resource + Clone>(
        &self,
        resource: &T,
        subresource: u32,
        map_type: MapType,
        do_not_wait: bool,
    ) -> crate::Result<SubresourceMapGuard<T>> {
        let flags = if do_not_wait {
            D3D11_MAP_FLAG_DO_NOT_WAIT.0
        } else {
            0
        };

        let mapped = wrap_out_ptr(|out| unsafe {
            self.0.Map(
                &resource.to_ffi_resource(),
                subresource,
                D3D11_MAP(map_type as i32),
                flags as _,
                Some(out),
            )
        })?;

        Ok(SubresourceMapGuard {
            data: mapped.pData,
            row_pitch: mapped.RowPitch,
            depth_pitch: mapped.DepthPitch,
            context: self.clone(),
            resource: resource.clone(),
            subresource,
        })
    }

    pub unsafe fn get_data<T: Sized>(
        &self,
        query: &impl Asynchronous,
        dont_flush: bool,
    ) -> GetDataResult<T> {
        let mut data = std::mem::zeroed();
        let result = (windows::core::Interface::vtable(&self.0).GetData)(
            windows::core::Interface::as_raw(&self.0),
            query.to_ffi_async().as_raw(),
            &mut data as *mut T as *mut _,
            std::mem::size_of::<T>() as u32,
            if dont_flush {
                D3D11_ASYNC_GETDATA_DONOTFLUSH.0 as u32
            } else {
                0
            },
        );

        match result {
            S_OK => GetDataResult::Ok(data),
            S_FALSE => GetDataResult::Pending,
            e => GetDataResult::Error(crate::Error::Win32(e.into())),
        }
    }

    pub fn is_query_ready(&self, query: &impl Asynchronous) -> bool {
        let result = unsafe {
            (windows::core::Interface::vtable(&self.0).GetData)(
                windows::core::Interface::as_raw(&self.0),
                query.to_ffi_async().as_raw(),
                std::ptr::null_mut(),
                0,
                0,
            )
        };

        result == S_OK
    }
}

pub enum GetDataResult<T> {
    Ok(T),
    Pending,
    Error(crate::Error),
}

#[repr(i32)]
#[derive(Clone, Copy, Debug)]
pub enum MapType {
    Read = D3D11_MAP_READ.0,
    Write = D3D11_MAP_WRITE.0,
    ReadWrite = D3D11_MAP_READ_WRITE.0,
    WriteDiscard = D3D11_MAP_WRITE_DISCARD.0,
    WriteNoOverwrite = D3D11_MAP_WRITE_NO_OVERWRITE.0,
}

bitflags! {
    pub struct ClearFlags: u32 {
        const DEPTH = D3D11_CLEAR_DEPTH.0;
        const STENCIL = D3D11_CLEAR_STENCIL.0;
    }
}

pub struct SubresourceMapGuard<T: Resource> {
    pub data: *mut c_void,
    pub row_pitch: u32,
    pub depth_pitch: u32,

    context: DeviceContext,
    resource: T,
    subresource: u32,
}

impl<T: Resource> Drop for SubresourceMapGuard<T> {
    fn drop(&mut self) {
        unsafe {
            self.context
                .0
                .Unmap(&self.resource.to_ffi_resource(), self.subresource);
        }
    }
}
