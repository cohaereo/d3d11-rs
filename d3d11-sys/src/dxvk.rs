#[cfg(not(target_os = "windows"))]
pub fn set_wsi_driver_env(for_swapchain: bool) {
    #[cfg(feature = "dxvk_wsi_glfw")]
    let driver = "GLFW";
    #[cfg(feature = "dxvk_wsi_sdl2")]
    let driver = "SDL2";
    #[cfg(feature = "dxvk_wsi_sdl3")]
    let driver = "SDL3";
    #[cfg(not(any(
        feature = "dxvk_wsi_glfw",
        feature = "dxvk_wsi_sdl2",
        feature = "dxvk_wsi_sdl3"
    )))]
    let driver = if for_swapchain {
        panic!("You must enable one of the dxvk_wsi_* features to create a swapchain on non-Windows platforms!")
    } else {
        "SDL3" // Fallback driver for non-swapchain usage
    };

    std::env::set_var("DXVK_WSI_DRIVER", driver);
}

#[cfg(target_os = "windows")]
pub fn set_wsi_driver_env(_for_swapchain: bool) {}
