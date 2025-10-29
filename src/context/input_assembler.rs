use std::mem::transmute;

use crate::{
    cast_optional_resource_refs, dxgi, error::validate_input, util::OptionalParam, Buffer,
    InputLayout,
};

use super::DeviceContext;
use d3d11_ffi::Direct3D::*;

impl DeviceContext {
    pub fn input_assembler_set_primitive_topology(&self, topology: PrimitiveTopology) {
        unsafe {
            self.0.IASetPrimitiveTopology(topology.into());
        }
    }

    pub fn input_assembler_set_input_layout(
        &self,
        layout: impl OptionalParam<Output = InputLayout>,
    ) {
        unsafe {
            self.0.IASetInputLayout(layout.as_option().map(|l| &l.0));
        }
    }

    pub fn input_assembler_set_vertex_buffers(
        &self,
        start_slot: u32,
        buffers: &[Option<&Buffer>],
        strides: Option<&[u32]>,
        offsets: Option<&[u32]>,
    ) -> crate::Result<()> {
        if let Some(strides) = strides {
            validate_input!(
                buffers.len() == strides.len(),
                "Stride count mismatch, got {} buffers but {} strides",
                buffers.len(),
                strides.len()
            );
        }

        if let Some(offsets) = offsets {
            validate_input!(
                buffers.len() == offsets.len(),
                "Offset count mismatch, got {} buffers but {} offsets",
                buffers.len(),
                offsets.len()
            );
        }

        unsafe {
            let raw_buffers = cast_optional_resource_refs!(8, buffers);
            self.0.IASetVertexBuffers(
                start_slot,
                buffers.len() as _,
                // SAFETY: d3d11::Buffer is repr(transparent) over ID3D11Buffer
                Some(transmute(raw_buffers.as_ptr())),
                strides.map(|s| s.as_ptr()),
                offsets.map(|o| o.as_ptr()),
            );
        }

        Ok(())
    }

    pub fn input_assembler_set_index_buffer(
        &self,
        buffer: impl OptionalParam<Output = Buffer>,
        format: dxgi::Format,
        offset: u32,
    ) {
        unsafe {
            self.0
                .IASetIndexBuffer(buffer.as_option().map(|b| &b.0), format.into(), offset);
        }
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug)]
pub enum PrimitiveTopology {
    PointList = D3D_PRIMITIVE_TOPOLOGY_POINTLIST.0,
    LineList = D3D_PRIMITIVE_TOPOLOGY_LINELIST.0,
    LineStrip = D3D_PRIMITIVE_TOPOLOGY_LINESTRIP.0,
    TriangleList = D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST.0,
    TriangleStrip = D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP.0,
    LineListAdj = D3D_PRIMITIVE_TOPOLOGY_LINELIST_ADJ.0,
    LineStripAdj = D3D_PRIMITIVE_TOPOLOGY_LINESTRIP_ADJ.0,
    TriangleListAdj = D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST_ADJ.0,
    TriangleStripAdj = D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP_ADJ.0,
    PatchList = D3D_PRIMITIVE_TOPOLOGY_1_CONTROL_POINT_PATCHLIST.0,
}

impl From<PrimitiveTopology> for D3D_PRIMITIVE_TOPOLOGY {
    fn from(topology: PrimitiveTopology) -> Self {
        Self(topology as i32)
    }
}
