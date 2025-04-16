use anyhow::Context;
use d3d11::dxgi::*;
use d3d11::*;
use glfw::{Action, Context as _, Key};

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

fn main() -> anyhow::Result<()> {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    let (mut window, events) = glfw
        .create_window(800, 600, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    let hwnd = HWND(window.get_win32_window());

    let device = Device::create(None, true).context("Failed to create device")?;
    let ictx = device.get_immediate_context();

    let swapchain = SwapChain::create(
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
    )
    .context("Failed to create swapchain")?;

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

    let vs = device.create_vertex_shader(&vs_data)?;
    let ps = device.create_pixel_shader(&ps_data)?;

    let target: Texture2D = swapchain
        .get_buffer(0)
        .context("Failed to get swapchain buffer")?;

    let rtv = device.create_render_target_view(&target, None)?;

    let dctx = device.create_deferred_context()?;

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                window.set_should_close(true)
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
