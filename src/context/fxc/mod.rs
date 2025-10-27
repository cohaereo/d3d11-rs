#[cfg(target_os = "windows")]
mod native;
#[cfg(target_os = "windows")]
pub use native::*;

// #[cfg(all(not(target_os = "windows"), feature = "fxc_wine"))]
// mod wine;
// #[cfg(all(not(target_os = "windows"), feature = "fxc_wine"))]
// pub use wine::*;

pub enum ShaderTarget {
    Vertex,
    Pixel,
    Geometry,
    Hull,
    Domain,
    Compute,
}

impl ShaderTarget {
    pub fn profile(&self) -> &'static str {
        match self {
            Self::Vertex => "vs_5_0",
            Self::Pixel => "ps_5_0",
            Self::Geometry => "gs_5_0",
            Self::Hull => "hs_5_0",
            Self::Domain => "ds_5_0",
            Self::Compute => "cs_5_0",
        }
    }
}
