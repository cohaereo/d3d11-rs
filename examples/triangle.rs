use std::rc::Rc;

use anyhow::Context as _;
use d3d11::dxgi::*;
use d3d11::*;
#[cfg(target_os = "windows")]
use raw_window_handle::HasWindowHandle;

#[cfg(feature = "fxc")]
fn compile_shaders() -> anyhow::Result<Vec<u8>, Vec<u8>> {
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

    let vs_data = d3d11::shader::fxc_compile(
        SHADER_SRC.as_bytes(),
        Some("triangle_vs"),
        &[],
        "VSMain",
        ShaderTarget::Vertex,
    )?;

    let ps_data = d3d11::shader::fxc_compile(
        SHADER_SRC.as_bytes(),
        Some("triangle_ps"),
        &[],
        "PSMain",
        ShaderTarget::Pixel,
    )?;
}

#[cfg(not(feature = "fxc"))]
fn compile_shaders() -> anyhow::Result<(Vec<u8>, Vec<u8>)> {
    let vs_data = include_bytes!("shaders/triangle_vs.cso").to_vec();
    let ps_data = include_bytes!("shaders/triangle_ps.cso").to_vec();
    Ok((vs_data, ps_data))
}

fn main() -> anyhow::Result<()> {
    let sdl_context = Rc::new(sdl2::init().expect("Failed to initialize SDL"));
    let video_subsystem = sdl_context
        .video()
        .expect("Failed to initialize video subsystem");

    let window = {
        let mut builder = video_subsystem.window("Hello this is window", 800, 600);

        let mut builder_ref = builder.position_centered().resizable();

        if cfg!(not(target_os = "windows")) {
            builder_ref = builder_ref.vulkan();
        }

        builder_ref.build().expect("Failed to create window")
    };

    #[cfg(target_os = "windows")]
    let hwnd = {
        let rwh = window
            .window_handle()
            .expect("Failed to get window handle")
            .as_raw();
        HWND(match rwh {
            raw_window_handle::RawWindowHandle::Win32(handle) => handle.hwnd.get(),
            _ => anyhow::bail!("Invalid window handle type"),
        } as _)
    };
    #[cfg(not(target_os = "windows"))]
    let hwnd = HWND(window.raw() as _);

    let device = Device::create(None, true).context("Failed to create device")?;
    let ictx = device.get_immediate_context();

    let swapchain = match SwapChain::create(
        &device,
        &SwapChainDesc::builder()
            .buffer_count(1)
            .buffer_desc(
                ModeDesc::builder()
                    .width(1024)
                    .height(1024)
                    .refresh_rate(Rational::default())
                    .format(Format::R8g8b8a8UnormSrgb)
                    .build(),
            )
            .output_window(hwnd)
            .buffer_usage(DxgiUsage::RENDER_TARGET_OUTPUT)
            .swap_effect(SwapEffect::Discard)
            .build(),
    ) {
        Ok(o) => o,
        Err(e) => {
            anyhow::bail!("Failed to create swapchain: {} ({})", e, sdl2::get_error());
        }
    };

    let (vs_data, ps_data) = compile_shaders()?;

    let vs = device.create_vertex_shader(&vs_data)?;
    let ps = device.create_pixel_shader(&ps_data)?;

    let target: Texture2D = swapchain
        .get_buffer(0)
        .context("Failed to get swapchain buffer")?;

    let rtv = device.create_render_target_view(&target, None)?;

    let dctx = device.create_deferred_context()?;

    let mut event_pump = sdl_context.event_pump().unwrap();
    'app: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'app,
                _ => {}
            }
        }

        dctx.clear_render_target_view(&rtv, &[0.0, 0.0, 0.0, 1.0]);
        dctx.output_merger_set_render_targets(&[Some(rtv.clone())], None);
        dctx.rasterizer_set_viewports(&[Viewport::builder().width(1024.0).height(1024.0).build()]);

        dctx.vertex_set_shader(Some(&vs));
        dctx.pixel_set_shader(Some(&ps));
        dctx.input_assembler_set_primitive_topology(PrimitiveTopology::TriangleList);
        dctx.draw(3, 0);

        ictx.execute_command_list(&dctx.finish_command_list(false)?, false);

        swapchain.present(0, PresentFlags::empty());
    }

    Ok(())
}
