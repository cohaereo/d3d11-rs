#[cfg(not(target_os = "windows"))]
pub fn set_wsi_driver_env() {
    #[cfg(feature = "dxvk_wsi_glfw")]
    const DXVK_WSI_DRIVER: &str = "GLFW";
    #[cfg(feature = "dxvk_wsi_sdl2")]
    const DXVK_WSI_DRIVER: &str = "SDL2";
    #[cfg(feature = "dxvk_wsi_sdl3")]
    const DXVK_WSI_DRIVER: &str = "SDL3";
    #[cfg(not(any(
        feature = "dxvk_wsi_glfw",
        feature = "dxvk_wsi_sdl2",
        feature = "dxvk_wsi_sdl3"
    )))]
    compile_error!("You must set one of the DXVK WSI driver features");

    std::env::set_var("DXVK_WSI_DRIVER", DXVK_WSI_DRIVER);
}

#[cfg(target_os = "windows")]
pub fn set_wsi_driver_env() {}
