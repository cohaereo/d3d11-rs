use bon::Builder;
use d3d11_ffi::Direct3D11::*;

use crate::{dxgi, impl_device_child};

#[repr(transparent)]
#[derive(Clone)]
pub struct InputLayout(pub(crate) ID3D11InputLayout);
impl_device_child!(InputLayout);

#[derive(Builder, Clone)]
pub struct InputElementDesc {
    #[builder(into)]
    pub semantic_name: String,
    pub semantic_index: u32,
    pub format: dxgi::Format,
    #[builder(default = 0)]
    pub input_slot: u32,

    #[builder(default)]
    pub aligned_byte_offset: ElementOffset,
    #[builder(default = InputClassification::PerVertexData)]
    pub input_slot_class: InputClassification,
    #[builder(default)]
    pub instance_data_step_rate: u32,
}

#[derive(Clone, Default)]
pub enum ElementOffset {
    /// Element will be appended right after the previous one
    #[default]
    Append,
    /// Element will be placed at the given absolute offset, relative to the start of the vertex
    Absolute(u32),
}

#[repr(i32)]
#[derive(Clone)]
pub enum InputClassification {
    PerVertexData = D3D11_INPUT_PER_VERTEX_DATA.0,
    PerInstanceData = D3D11_INPUT_PER_INSTANCE_DATA.0,
}

impl From<InputClassification> for D3D11_INPUT_CLASSIFICATION {
    fn from(classification: InputClassification) -> Self {
        Self(classification as i32)
    }
}
