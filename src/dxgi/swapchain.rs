use bitflags::bitflags;
use bon::Builder;
use d3d11_sys::{
    Direct3D11::ID3D11Resource,
    Dxgi::{Common::*, *},
    Foundation::{
        BOOL, DXGI_STATUS_CLIPPED, DXGI_STATUS_DDA_WAS_STILL_DRAWING,
        DXGI_STATUS_GRAPHICS_VIDPN_SOURCE_IN_USE, DXGI_STATUS_MODE_CHANGED,
        DXGI_STATUS_MODE_CHANGE_IN_PROGRESS, DXGI_STATUS_NO_DESKTOP_ACCESS,
        DXGI_STATUS_NO_REDIRECTION, DXGI_STATUS_OCCLUDED, DXGI_STATUS_PRESENT_REQUIRED,
        DXGI_STATUS_UNOCCLUDED, HWND,
    },
};

use crate::{
    device::Device, error::validate_input, resource::Resource, util::wrap_out_result,
    verify_ffi_struct, FormatSupport,
};

use super::{Format, SampleDesc};

#[repr(transparent)]
pub struct SwapChain(pub(crate) IDXGISwapChain);

impl SwapChain {
    pub fn create(device: &Device, desc: &SwapChainDesc) -> crate::Result<Self> {
        let format_support = device.check_format_support(desc.buffer_desc.format);
        validate_input!(
            format_support.contains(FormatSupport::DISPLAY),
            "Invalid format for swap chain, {:?} does not support being used as display buffer",
            desc.buffer_desc.format
        );

        let dxgi = unsafe { CreateDXGIFactory::<IDXGIFactory>()? };
        let swap_chain = wrap_out_result(|out| unsafe {
            dxgi.CreateSwapChain(&device.0, desc.as_ffi(), out).ok()
        })?;

        Ok(Self(swap_chain))
    }

    pub fn present(&self, sync_interval: u32, flags: PresentFlags) -> Option<SwapChainStatus> {
        let result = unsafe { self.0.Present(sync_interval, DXGI_PRESENT(flags.bits())) };

        Some(match result {
            DXGI_STATUS_CLIPPED => SwapChainStatus::Clipped,
            DXGI_STATUS_DDA_WAS_STILL_DRAWING => SwapChainStatus::DdaWasStillDrawing,
            DXGI_STATUS_GRAPHICS_VIDPN_SOURCE_IN_USE => SwapChainStatus::GraphicsVidpnSourceInUse,
            DXGI_STATUS_MODE_CHANGED => SwapChainStatus::ModeChanged,
            DXGI_STATUS_MODE_CHANGE_IN_PROGRESS => SwapChainStatus::ModeChangeInProgress,
            DXGI_STATUS_NO_DESKTOP_ACCESS => SwapChainStatus::NoDesktopAccess,
            DXGI_STATUS_NO_REDIRECTION => SwapChainStatus::NoRedirection,
            DXGI_STATUS_OCCLUDED => SwapChainStatus::Occluded,
            DXGI_STATUS_PRESENT_REQUIRED => SwapChainStatus::PresentRequired,
            DXGI_STATUS_UNOCCLUDED => SwapChainStatus::Unoccluded,
            _ => return None,
            // r if r.is_ok() => SwapChainStatus::Ok,
            // e => return Err(crate::Error::Win32(e.into())),
        })
    }

    pub fn get_buffer<T: Resource>(&self, buffer: u32) -> crate::Result<T> {
        let resource = unsafe { self.0.GetBuffer::<ID3D11Resource>(buffer)? };
        T::from_ffi_resource(resource)
            .ok_or_else(|| crate::Error::InvalidInput("Invalid buffer".into()))
    }

    pub fn resize_buffers(
        &self,
        buffercount: u32,
        width: u32,
        height: u32,
        newformat: Format,
        swapchainflags: SwapChainFlags,
    ) -> crate::Result<()> {
        unsafe {
            Ok(self.0.ResizeBuffers(
                buffercount,
                width,
                height,
                newformat.into(),
                DXGI_SWAP_CHAIN_FLAG(swapchainflags.bits()),
            )?)
        }
    }
}

#[repr(C)]
#[derive(Clone, Debug, Builder)]
pub struct SwapChainDesc {
    pub buffer_desc: ModeDesc,
    #[builder(default)]
    pub sample_desc: SampleDesc,
    pub buffer_usage: DxgiUsage,
    pub buffer_count: u32,
    pub output_window: HWND,
    #[builder(default = true, into)]
    pub windowed: BOOL,
    pub swap_effect: SwapEffect,
    #[builder(default = SwapChainFlags::empty())]
    pub flags: SwapChainFlags,
}
verify_ffi_struct!(SwapChainDesc, DXGI_SWAP_CHAIN_DESC);

#[repr(C)]
#[derive(Clone, Debug, Builder)]
pub struct ModeDesc {
    #[builder(default)]
    pub width: u32,
    #[builder(default)]
    pub height: u32,
    #[builder(default)]
    pub refresh_rate: Rational,
    pub format: Format,
    #[builder(default)]
    pub scanline_ordering: ScanlineOrder,
    #[builder(default)]
    pub scaling: ScalingMode,
}
verify_ffi_struct!(ModeDesc, DXGI_MODE_DESC);

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct Rational {
    pub numerator: u32,
    pub denominator: u32,
}

impl Rational {
    pub fn new(numerator: u32, denominator: u32) -> Self {
        Self {
            numerator,
            denominator,
        }
    }
}

verify_ffi_struct!(Rational, DXGI_RATIONAL);

#[repr(i32)]
#[derive(Clone, Debug, Default)]
pub enum ScanlineOrder {
    #[default]
    Unspecified = DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED.0,
    Progressive = DXGI_MODE_SCANLINE_ORDER_PROGRESSIVE.0,
    UpperFieldFirst = DXGI_MODE_SCANLINE_ORDER_UPPER_FIELD_FIRST.0,
    LowerFieldFirst = DXGI_MODE_SCANLINE_ORDER_LOWER_FIELD_FIRST.0,
}

#[repr(i32)]
#[derive(Clone, Debug, Default)]
pub enum ScalingMode {
    #[default]
    Unspecified = DXGI_MODE_SCALING_UNSPECIFIED.0,
    Centered = DXGI_MODE_SCALING_CENTERED.0,
    Stretched = DXGI_MODE_SCALING_STRETCHED.0,
}

#[repr(i32)]
#[derive(Clone, Debug)]
pub enum SwapEffect {
    Discard = DXGI_SWAP_EFFECT_DISCARD.0,
    FlipDiscard = DXGI_SWAP_EFFECT_FLIP_DISCARD.0,
    FlipSequential = DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL.0,
    Sequential = DXGI_SWAP_EFFECT_SEQUENTIAL.0,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SwapChainStatus {
    Ok,
    Clipped,                  // DXGI_STATUS_CLIPPED
    DdaWasStillDrawing,       // DXGI_STATUS_DDA_WAS_STILL_DRAWING
    GraphicsVidpnSourceInUse, // DXGI_STATUS_GRAPHICS_VIDPN_SOURCE_IN_USE
    ModeChanged,              // DXGI_STATUS_MODE_CHANGED
    ModeChangeInProgress,     // DXGI_STATUS_MODE_CHANGE_IN_PROGRESS
    NoDesktopAccess,          // DXGI_STATUS_NO_DESKTOP_ACCESS
    NoRedirection,            // DXGI_STATUS_NO_REDIRECTION
    Occluded,                 // DXGI_STATUS_OCCLUDED
    PresentRequired,          // DXGI_STATUS_PRESENT_REQUIRED
    Unoccluded,               // DXGI_STATUS_UNOCCLUDED
}

bitflags! {
    #[derive(Clone, Debug)]
    pub struct DxgiUsage: u32 {
        const BACK_BUFFER = DXGI_USAGE_BACK_BUFFER.0;
        const DISCARD_ON_PRESENT = DXGI_USAGE_DISCARD_ON_PRESENT.0;
        const READ_ONLY = DXGI_USAGE_READ_ONLY.0;
        const RENDER_TARGET_OUTPUT = DXGI_USAGE_RENDER_TARGET_OUTPUT.0;
        const SHADER_INPUT = DXGI_USAGE_SHADER_INPUT.0;
        const SHARED = DXGI_USAGE_SHARED.0;
        const UNORDERED_ACCESS = DXGI_USAGE_UNORDERED_ACCESS.0;
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct PresentFlags: u32 {
        const ALLOW_TEARING = DXGI_PRESENT_ALLOW_TEARING.0;
        const DO_NOT_SEQUENCE = DXGI_PRESENT_DO_NOT_SEQUENCE.0;
        const DO_NOT_WAIT = DXGI_PRESENT_DO_NOT_WAIT.0;
        const RESTART = DXGI_PRESENT_RESTART.0;
        const RESTRICT_TO_OUTPUT = DXGI_PRESENT_RESTRICT_TO_OUTPUT.0;
        const STEREO_PREFER_RIGHT = DXGI_PRESENT_STEREO_PREFER_RIGHT.0;
        const STEREO_TEMPORARY_MONO = DXGI_PRESENT_STEREO_TEMPORARY_MONO.0;
        const TEST = DXGI_PRESENT_TEST.0;
        const USE_DURATION = DXGI_PRESENT_USE_DURATION.0;
    }
}

bitflags! {
    #[derive(Clone, Debug)]
    pub struct SwapChainFlags: i32 {
        const ALLOW_MODE_SWITCH = DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH.0;
        const ALLOW_TEARING = DXGI_SWAP_CHAIN_FLAG_ALLOW_TEARING.0;
        const DISPLAY_ONLY = DXGI_SWAP_CHAIN_FLAG_DISPLAY_ONLY.0;
        const FOREGROUND_LAYER = DXGI_SWAP_CHAIN_FLAG_FOREGROUND_LAYER.0;
        const FRAME_LATENCY_WAITABLE_OBJECT = DXGI_SWAP_CHAIN_FLAG_FRAME_LATENCY_WAITABLE_OBJECT.0;
        const FULLSCREEN_VIDEO = DXGI_SWAP_CHAIN_FLAG_FULLSCREEN_VIDEO.0;
        const GDI_COMPATIBLE = DXGI_SWAP_CHAIN_FLAG_GDI_COMPATIBLE.0;
        const HW_PROTECTED = DXGI_SWAP_CHAIN_FLAG_HW_PROTECTED.0;
        const NONPREROTATED = DXGI_SWAP_CHAIN_FLAG_NONPREROTATED.0;
        const RESTRICTED_CONTENT = DXGI_SWAP_CHAIN_FLAG_RESTRICTED_CONTENT.0;
        const RESTRICTED_TO_ALL_HOLOGRAPHIC_DISPLAYS = DXGI_SWAP_CHAIN_FLAG_RESTRICTED_TO_ALL_HOLOGRAPHIC_DISPLAYS.0;
        const RESTRICT_SHARED_RESOURCE_DRIVER = DXGI_SWAP_CHAIN_FLAG_RESTRICT_SHARED_RESOURCE_DRIVER.0;
    }
}
