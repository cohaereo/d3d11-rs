use std::mem::transmute;

use crate::{
    blend_state::BlendState, rtv::RenderTargetView, util::OptionalParam, DepthStencilState,
    DepthStencilView,
};

use super::DeviceContext;

impl DeviceContext {
    pub fn output_merger_set_render_targets(
        &self,
        render_target_views: &[Option<RenderTargetView>],
        depth_stencil_view: Option<&DepthStencilView>,
    ) {
        unsafe {
            self.0.OMSetRenderTargets(
                Some(transmute(render_target_views)),
                depth_stencil_view.map(|d| &d.0),
            );
        }
    }

    pub fn output_merger_get_render_targets(
        &self,
    ) -> ([Option<RenderTargetView>; 8], Option<DepthStencilView>) {
        let mut rtvs = [const { None }; 8];
        let mut dsv = None;
        unsafe {
            self.0.OMGetRenderTargets(Some(&mut rtvs), Some(&mut dsv));
        }
        (
            rtvs.map(|r| r.map(RenderTargetView)),
            dsv.map(DepthStencilView),
        )
    }

    pub fn output_merger_set_blend_state(
        &self,
        blend_state: impl OptionalParam<Output = BlendState>,
        blend_factor: Option<&[f32; 4]>,
        sample_mask: u32,
    ) {
        unsafe {
            self.0.OMSetBlendState(
                blend_state.as_option().map(|bs| &bs.0),
                blend_factor,
                sample_mask,
            );
        }
    }

    pub fn output_merger_set_depth_stencil_state(
        &self,
        depth_stencil_state: impl OptionalParam<Output = DepthStencilState>,
        stencil_ref: u32,
    ) {
        unsafe {
            self.0.OMSetDepthStencilState(
                depth_stencil_state.as_option().map(|dss| &dss.0),
                stencil_ref,
            );
        }
    }
}
