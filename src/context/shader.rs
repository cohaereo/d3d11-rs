use std::mem::transmute;

use crate::{
    buffer::Buffer, sampler::SamplerState, shader::*, util::OptionalParam, ShaderResourceView,
};
use windows::Win32::Graphics::Direct3D11::*;

use super::DeviceContext;

// generate_common_stage_methods!(get_constant_buffers, GetConstantBuffers, (arg1: &ID3D11Buffer, arg2: u32) -> () {
//  // ...
// });

macro_rules! generate_common_stage_methods {
    (
        $method:ident,
        $method_ffi:ident,
        ($($arg:ident: $arg_ty:ty),*) -> $ret:ty $impl:block
    ) => {
        generate_common_stage_methods!(single_stage $method, $method_ffi, vertex, VS, ($($arg: $arg_ty),*) -> $ret $impl);
        generate_common_stage_methods!(single_stage $method, $method_ffi, pixel, PS, ($($arg: $arg_ty),*) -> $ret $impl);
        generate_common_stage_methods!(single_stage $method, $method_ffi, geometry, GS, ($($arg: $arg_ty),*) -> $ret $impl);
        generate_common_stage_methods!(single_stage $method, $method_ffi, hull, HS, ($($arg: $arg_ty),*) -> $ret $impl);
        generate_common_stage_methods!(single_stage $method, $method_ffi, domain, DS, ($($arg: $arg_ty),*) -> $ret $impl);
        generate_common_stage_methods!(single_stage $method, $method_ffi, compute, CS, ($($arg: $arg_ty),*) -> $ret $impl);
    };
    (
        single_stage
        $method:ident,
        $method_ffi:ident,
        $prefix:ident, // eg vs/ps
        $prefix_ffi:ident, // eg. VS/PS
        ($($arg:ident: $arg_ty:ty),*) -> $ret:ty $impl:block
    ) => {
        paste::paste! {
            #[doc(alias = "" [<$prefix_ffi $method_ffi>])]
            pub fn [<$prefix _ $method>](&self, $($arg: $arg_ty),*) -> $ret {
                let ctx = &self.0;
                let method = ID3D11DeviceContext::[<$prefix_ffi $method_ffi>];
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

type FuncSetSamplers = unsafe fn(
    ctx: &ID3D11DeviceContext,
    start_slot: u32,
    samplers: Option<&[Option<ID3D11SamplerState>]>,
) -> ();

type FuncSetShaderResources = unsafe fn(
    ctx: &ID3D11DeviceContext,
    start_slot: u32,
    views: Option<&[Option<ID3D11ShaderResourceView>]>,
) -> ();

impl DeviceContext {
    // generate_common_stage_methods!(get_constant_buffers, GetConstantBuffers);
    generate_common_stage_methods!(set_constant_buffers, SetConstantBuffers, (start_slot: u32, buffers: &[Option<Buffer>]) -> () {
        |ctx, method: FuncSetConstantBuffers|
        unsafe {
            // SAFETY: `Buffer` is a transparent wrapper around `ID3D11Buffer`
            method(ctx, start_slot, Some(transmute(buffers)));
        }
    });
    // generate_common_stage_methods!(get_samplers, GetSamplers);
    generate_common_stage_methods!(set_samplers, SetSamplers, (start_slot: u32, samplers: &[Option<SamplerState>]) -> () {
        |ctx, method: FuncSetSamplers|
        unsafe {
            // SAFETY: `Sampler` is a transparent wrapper around `ID3D11SamplerState`
            method(ctx, start_slot, Some(transmute(samplers)));
        }
    });
    // generate_common_stage_methods!(get_shader_resources, GetShaderResources);
    generate_common_stage_methods!(set_shader_resources, SetShaderResources, (start_slot: u32, views: &[Option<ShaderResourceView>]) -> () {
        |ctx, method: FuncSetShaderResources|
        unsafe {
            method(ctx, start_slot, Some(transmute(views)));
        }
    });

    generate_shader_getters_setters!();
}
