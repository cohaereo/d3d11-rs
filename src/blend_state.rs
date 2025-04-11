use bon::Builder;
use d3d11_sys::{Direct3D11::*, Foundation::BOOL};

use crate::{impl_device_child, verify_ffi_struct};

#[repr(transparent)]
#[derive(Clone)]
pub struct BlendState(pub(crate) ID3D11BlendState);
impl_device_child!(BlendState);

#[repr(C)]
#[derive(Builder, Clone, Default, Debug)]
pub struct BlendDesc {
    #[builder(into)]
    pub alpha_to_coverage_enable: BOOL,
    #[builder(into)]
    pub independent_blend_enable: BOOL,
    pub render_target: [RenderTargetBlendDesc; 8],
}
verify_ffi_struct!(BlendDesc, D3D11_BLEND_DESC);

impl BlendDesc {
    pub const DISABLED: Self = Self {
        alpha_to_coverage_enable: BOOL(0),
        independent_blend_enable: BOOL(0),
        render_target: [RenderTargetBlendDesc::DISABLED; 8],
    };

    pub fn from_single_target(target: RenderTargetBlendDesc) -> Self {
        Self {
            alpha_to_coverage_enable: false.into(),
            independent_blend_enable: true.into(),
            render_target: std::array::from_fn(|_| target.clone()),
        }
    }
}

#[repr(C)]
#[derive(Builder, Clone, Debug)]
pub struct RenderTargetBlendDesc {
    #[builder(into)]
    pub blend_enable: BOOL,
    pub src_blend: Blend,
    pub dest_blend: Blend,
    pub blend_op: BlendOp,
    pub src_blend_alpha: Blend,
    pub dest_blend_alpha: Blend,
    pub blend_op_alpha: BlendOp,
    pub render_target_write_mask: u8, // TODO: Bitflags
}
verify_ffi_struct!(RenderTargetBlendDesc, D3D11_RENDER_TARGET_BLEND_DESC);

impl RenderTargetBlendDesc {
    pub const DISABLED: Self = Self {
        blend_enable: BOOL(0),
        src_blend: Blend::One,
        dest_blend: Blend::Zero,
        blend_op: BlendOp::Add,
        src_blend_alpha: Blend::One,
        dest_blend_alpha: Blend::Zero,
        blend_op_alpha: BlendOp::Add,
        render_target_write_mask: 0,
    };
}

impl Default for RenderTargetBlendDesc {
    fn default() -> Self {
        Self::DISABLED
    }
}

#[repr(i32)]
#[derive(Clone, Debug)]
pub enum Blend {
    Zero = D3D11_BLEND_ZERO.0,
    One = D3D11_BLEND_ONE.0,
    SrcColor = D3D11_BLEND_SRC_COLOR.0,
    InvSrcColor = D3D11_BLEND_INV_SRC_COLOR.0,
    SrcAlpha = D3D11_BLEND_SRC_ALPHA.0,
    InvSrcAlpha = D3D11_BLEND_INV_SRC_ALPHA.0,
    DestAlpha = D3D11_BLEND_DEST_ALPHA.0,
    InvDestAlpha = D3D11_BLEND_INV_DEST_ALPHA.0,
    DestColor = D3D11_BLEND_DEST_COLOR.0,
    InvDestColor = D3D11_BLEND_INV_DEST_COLOR.0,
    SrcAlphaSat = D3D11_BLEND_SRC_ALPHA_SAT.0,
    BlendFactor = D3D11_BLEND_BLEND_FACTOR.0,
    InvBlendFactor = D3D11_BLEND_INV_BLEND_FACTOR.0,
    Src1Color = D3D11_BLEND_SRC1_COLOR.0,
    InvSrc1Color = D3D11_BLEND_INV_SRC1_COLOR.0,
    Src1Alpha = D3D11_BLEND_SRC1_ALPHA.0,
    InvSrc1Alpha = D3D11_BLEND_INV_SRC1_ALPHA.0,
}

#[repr(i32)]
#[derive(Clone, Debug)]
pub enum BlendOp {
    Add = D3D11_BLEND_OP_ADD.0,
    Subtract = D3D11_BLEND_OP_SUBTRACT.0,
    RevSubtract = D3D11_BLEND_OP_REV_SUBTRACT.0,
    Min = D3D11_BLEND_OP_MIN.0,
    Max = D3D11_BLEND_OP_MAX.0,
}
