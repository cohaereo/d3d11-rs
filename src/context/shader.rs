use std::mem::transmute;

use crate::{
    buffer::Buffer, sampler::SamplerState, shader::*, util::OptionalParam, ShaderResourceView,
    UnorderedAccessView,
};
use d3d11_ffi::Direct3D11::*;

use super::DeviceContext;

// generate_common_stage_methods!(get_constant_buffers, GetConstantBuffers, (arg1: &ID3D11Buffer, arg2: u32) -> () {
//  // ...
// });

type ID3D11DeviceContext0 = ID3D11DeviceContext;
macro_rules! generate_common_stage_methods {
    (
        api($api_version:expr),
        $method:ident,
        $method_ffi:ident,
        ($($arg:ident: $arg_ty:ty),*) -> $ret:ty $impl:block
    ) => {
        generate_common_stage_methods!(single_stage api($api_version), $method, $method_ffi, vertex, VS, ($($arg: $arg_ty),*) -> $ret $impl);
        generate_common_stage_methods!(single_stage api($api_version), $method, $method_ffi, pixel, PS, ($($arg: $arg_ty),*) -> $ret $impl);
        generate_common_stage_methods!(single_stage api($api_version), $method, $method_ffi, geometry, GS, ($($arg: $arg_ty),*) -> $ret $impl);
        generate_common_stage_methods!(single_stage api($api_version), $method, $method_ffi, hull, HS, ($($arg: $arg_ty),*) -> $ret $impl);
        generate_common_stage_methods!(single_stage api($api_version), $method, $method_ffi, domain, DS, ($($arg: $arg_ty),*) -> $ret $impl);
        generate_common_stage_methods!(single_stage api($api_version), $method, $method_ffi, compute, CS, ($($arg: $arg_ty),*) -> $ret $impl);
    };
    (
        single_stage
        api($api_version:expr),
        $method:ident,
        $method_ffi:ident,
        $prefix:ident, // eg vs/ps
        $prefix_ffi:ident, // eg. VS/PS
        ($($arg:ident: $arg_ty:ty),*) -> $ret:ty $impl:block
    ) => {
        paste::paste! {
            #[doc(alias = "" [<$prefix_ffi $method_ffi>])]
            pub fn [<$prefix _ $method>](&self, $($arg: $arg_ty),*) -> $ret {
                let ctx = &self.$api_version;
                let method = [<ID3D11DeviceContext $api_version>]::[<$prefix_ffi $method_ffi>];
                let inner = $impl;

                inner(ctx, method)
            }
        }
    };
}

macro_rules! generate_shader_getters_setters {
    () => {
        generate_shader_getters_setters!(single_stage VertexShader, vertex, VS);
        generate_shader_getters_setters!(single_stage PixelShader, pixel, PS);
        generate_shader_getters_setters!(single_stage GeometryShader, geometry, GS);
        generate_shader_getters_setters!(single_stage HullShader, hull, HS);
        generate_shader_getters_setters!(single_stage DomainShader, domain, DS);
        generate_shader_getters_setters!(single_stage ComputeShader, compute, CS);
    };
    (
        single_stage
        $shader_type:ident,
        $prefix:ident, // eg vs/ps
        $prefix_ffi:ident // eg. VS/PS
    ) => {
        paste::paste! {
            #[doc(alias = "" [<$prefix_ffi SetShader>])]
            pub fn [<$prefix _ set_shader>]<S>(&self, shader: S)
            where S: OptionalParam<Output = $shader_type> {
                unsafe {
                    self.0.[<$prefix_ffi SetShader>](shader.as_option().map(|s| &s.0), None);
                }
            }

            // TODO: implement GetShader
        }
    };
}

type FuncSetConstantBuffers = unsafe fn(
    ctx: &ID3D11DeviceContext,
    start_slot: u32,
    buffers: Option<&[Option<ID3D11Buffer>]>,
) -> ();

type FuncSetConstantBuffers1 = unsafe fn(
    ctx: &ID3D11DeviceContext1,
    start_slot: u32,
    num_buffers: u32,
    buffers: Option<*const Option<ID3D11Buffer>>,
    first_constant: Option<*const u32>,
    num_constants: Option<*const u32>,
) -> ();

type FuncGetConstantBuffers = unsafe fn(
    ctx: &ID3D11DeviceContext,
    start_slot: u32,
    buffers: Option<&mut [Option<ID3D11Buffer>]>,
) -> ();

type FuncSetSamplers = unsafe fn(
    ctx: &ID3D11DeviceContext,
    start_slot: u32,
    samplers: Option<&[Option<ID3D11SamplerState>]>,
) -> ();

type FuncGetSamplers = unsafe fn(
    ctx: &ID3D11DeviceContext,
    start_slot: u32,
    samplers: Option<&mut [Option<ID3D11SamplerState>]>,
) -> ();

type FuncSetShaderResources = unsafe fn(
    ctx: &ID3D11DeviceContext,
    start_slot: u32,
    views: Option<&[Option<ID3D11ShaderResourceView>]>,
) -> ();

type FuncGetShaderResources = unsafe fn(
    ctx: &ID3D11DeviceContext,
    start_slot: u32,
    views: Option<&mut [Option<ID3D11ShaderResourceView>]>,
) -> ();

#[cfg_attr(feature = "profiling", profiling::all_functions)]
impl DeviceContext {
    pub const SAMPLER_SLOT_COUNT: usize = D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT as usize;
    pub const SHADER_RESOURCE_SLOT_COUNT: usize =
        D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT as usize;
    pub const CONSTANT_BUFFER_SLOT_COUNT: usize =
        D3D11_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT as usize;

    // generate_common_stage_methods!(get_constant_buffers, GetConstantBuffers);
    generate_common_stage_methods!(api(0), set_constant_buffers, SetConstantBuffers, (start_slot: u32, buffers: &[Option<Buffer>]) -> () {
        |ctx, method: FuncSetConstantBuffers|
        unsafe {
            // SAFETY: `Buffer` is a transparent wrapper around `ID3D11Buffer`
            method(ctx, start_slot, Some(transmute(buffers)));
        }
    });
    generate_common_stage_methods!(api(1), set_constant_buffers1, SetConstantBuffers1, (start_slot: u32, buffers: &[Option<Buffer>], first_constant: Option<&[u32]>, num_constants: Option<&[u32]>) -> () {
        |ctx, method: FuncSetConstantBuffers1|
        unsafe {
            let num_buffers = buffers.len() as u32;
            if let Some(first_constant) = first_constant {
                assert_eq!(
                    buffers.len(),
                    first_constant.len(),
                    "Mismatched buffer and first constant lengths"
                );
            }
            if let Some(num_constants) = num_constants {
                assert_eq!(
                    buffers.len(),
                    num_constants.len(),
                    "Mismatched buffer and num constants lengths"
                );
            }

            // SAFETY: `Buffer` is a transparent wrapper around `ID3D11Buffer`
            method(ctx, start_slot, num_buffers, Some(buffers.as_ptr() as *const Option<ID3D11Buffer>), first_constant.map(|f| f.as_ptr()), num_constants.map(|n| n.as_ptr()));
        }
    });
    generate_common_stage_methods!(api(0), get_constant_buffers, GetConstantBuffers, () -> [Option<Buffer>; D3D11_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT as usize] {
        |ctx, method: FuncGetConstantBuffers|
        unsafe {
            let mut buffers = [const { None }; D3D11_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT as usize];
            method(ctx, 0, Some(&mut buffers));
            buffers.map(|b| b.map(Buffer))
        }
    });
    // generate_common_stage_methods!(get_samplers, GetSamplers);
    generate_common_stage_methods!(api(0), set_samplers, SetSamplers, (start_slot: u32, samplers: &[Option<SamplerState>]) -> () {
        |ctx, method: FuncSetSamplers|
        unsafe {
            // SAFETY: `Sampler` is a transparent wrapper around `ID3D11SamplerState`
            method(ctx, start_slot, Some(transmute(samplers)));
        }
    });
    generate_common_stage_methods!(api(0), get_samplers, GetSamplers, () -> [Option<SamplerState>; D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT as usize] {
        |ctx, method: FuncGetSamplers|
        unsafe {
            let mut samplers = [const { None }; D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT as usize];
            method(ctx, 0, Some(&mut samplers));
            samplers.map(|s| s.map(SamplerState))
        }
    });
    // generate_common_stage_methods!(get_shader_resources, GetShaderResources);
    generate_common_stage_methods!(api(0), set_shader_resources, SetShaderResources, (start_slot: u32, views: &[Option<ShaderResourceView>]) -> () {
        |ctx, method: FuncSetShaderResources|
        unsafe {
            method(ctx, start_slot, Some(transmute(views)));
        }
    });
    generate_common_stage_methods!(api(0), get_shader_resources, GetShaderResources, () -> [Option<ShaderResourceView>; D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT as usize] {
        |ctx, method: FuncGetShaderResources|
        unsafe {
            let mut views = [const { None }; D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT as usize];
            method(ctx, 0, Some(&mut views));
            views.map(|v| v.map(ShaderResourceView))
        }
    });

    pub fn compute_set_unordered_access_views(
        &self,
        start_slot: u32,
        views: &[Option<UnorderedAccessView>],
        initial_counts: Option<&[u32]>,
    ) {
        if let Some(counts) = initial_counts {
            assert_eq!(
                views.len(),
                counts.len(),
                "Mismatched view and initial counts lengths"
            );
        }

        unsafe {
            self.0.CSSetUnorderedAccessViews(
                start_slot,
                views.len() as u32,
                Some(views.as_ptr() as _),
                initial_counts.map(|c| c.as_ptr()),
            );
        }
    }

    generate_shader_getters_setters!();
}
