use windows::Win32::Graphics::Direct3D11::*;

#[repr(i32)]
#[derive(Clone, Debug)]
pub enum ComparisonFunc {
    Always = D3D11_COMPARISON_ALWAYS.0,
    Equal = D3D11_COMPARISON_EQUAL.0,
    Greater = D3D11_COMPARISON_GREATER.0,
    GreaterEqual = D3D11_COMPARISON_GREATER_EQUAL.0,
    Less = D3D11_COMPARISON_LESS.0,
    LessEqual = D3D11_COMPARISON_LESS_EQUAL.0,
    Never = D3D11_COMPARISON_NEVER.0,
    NotEqual = D3D11_COMPARISON_NOT_EQUAL.0,
}
