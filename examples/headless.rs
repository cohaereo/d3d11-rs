use anyhow::Context as _;
use d3d11::dxgi::*;
use d3d11::*;

#[cfg(feature = "fxc")]
fn compile_shaders() -> anyhow::Result<(Vec<u8>, Vec<u8>)> {
    use d3d11::fxc::ShaderTarget;

    const SHADER_SRC: &str = r#"
    struct VSOutput {
        float4 position : SV_POSITION;
        float4 color : COLOR;
    };

    static const float4 vertices[3] = {
        float4(0.0, 0.5, 0.0, 1.0),
        float4(0.5, -0.5, 0.0, 1.0),
        float4(-0.5, -0.5, 0.0, 1.0)
    };

    static const float4 colors[3] = {
        float4(1.0, 0.0, 0.0, 1.0),
        float4(0.0, 1.0, 0.0, 1.0),
        float4(0.0, 0.0, 1.0, 1.0)
    };

    VSOutput VSMain(uint vertexID : SV_VertexID) {
        VSOutput output;
        output.position = vertices[vertexID];
        output.color = colors[vertexID];
        return output;
    }

    float4 PSMain(VSOutput input) : SV_Target {
        return input.color;
    }
    "#;

    let vs_data = d3d11::fxc::compile(
        SHADER_SRC.as_bytes(),
        Some("triangle_vs"),
        &[],
        "VSMain",
        ShaderTarget::Vertex,
    )?;

    let ps_data = d3d11::fxc::compile(
        SHADER_SRC.as_bytes(),
        Some("triangle_ps"),
        &[],
        "PSMain",
        ShaderTarget::Pixel,
    )?;

    Ok((vs_data, ps_data))
}

#[cfg(all(not(feature = "fxc"), not(feature = "fxc_wine")))]
fn compile_shaders() -> anyhow::Result<(Vec<u8>, Vec<u8>)> {
    let vs_data = include_bytes!("shaders/triangle_vs.cso").to_vec();
    let ps_data = include_bytes!("shaders/triangle_ps.cso").to_vec();
    Ok((vs_data, ps_data))
}

fn main() -> anyhow::Result<()> {
    let device = Device::create(None, true).context("Failed to create device")?;
    let ctx = device.get_immediate_context();

    let (vs_data, ps_data) = compile_shaders()?;

    let vs = device.create_vertex_shader(&vs_data)?;
    let ps = device.create_pixel_shader(&ps_data)?;

    let output_rtv = device.create_texture2d(
        &Texture2dDesc::builder()
            .width(1024)
            .height(1024)
            .mip_levels(1)
            .array_size(1)
            .format(Format::R8g8b8a8Unorm)
            .bind_flags(BindFlags::RENDER_TARGET)
            .cpu_access_flags(CpuAccessFlags::READ)
            .usage(Usage::Dynamic)
            .sample_desc(SampleDesc::builder().count(1).quality(0).build())
            .build(),
        None,
    )?;

    let rtv = device.create_render_target_view(&output_rtv, None)?;

    ctx.clear_render_target_view(&rtv, &[0.0, 0.0, 0.0, 1.0]);
    ctx.output_merger_set_render_targets(&[Some(&rtv)], None);
    ctx.rasterizer_set_viewports(&[Viewport::builder().width(1024.0).height(1024.0).build()]);

    ctx.vertex_set_shader(Some(&vs));
    ctx.pixel_set_shader(Some(&ps));
    ctx.input_assembler_set_primitive_topology(PrimitiveTopology::TriangleList);
    ctx.draw(3, 0);

    let rtv_map = ctx
        .map(&output_rtv, 0, MapType::Read, false)
        .context("Failed to map texture")?;

    let mut data = vec![0u8; 4 * 1024 * 1024];
    unsafe {
        std::ptr::copy_nonoverlapping(rtv_map.data as *const u8, data.as_mut_ptr(), data.len());
    }
    drop(rtv_map);

    let img = image::RgbaImage::from_raw(1024, 1024, data)
        .context("Failed to create image from raw data")?;
    img.save("output.bmp").context("Failed to save image")?;
    println!("Saved output.bmp");

    Ok(())
}
