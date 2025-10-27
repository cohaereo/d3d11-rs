#[cfg(target_os = "windows")]
#[macro_export]
macro_rules! link {
    ($library_win:literal $library_dxvk:literal $abi:literal $($link_name:literal)? $(#[$doc:meta])? fn $($function:tt)*) => (
        #[link(name = $library_win, kind = "raw-dylib", modifiers = "+verbatim")]
        extern $abi {
            $(#[link_name=$link_name])?
            pub fn $($function)*;
        }
    )
}

#[cfg(not(target_os = "windows"))]
#[macro_export]
macro_rules! link {
    ($library_win:literal $library_dxvk:literal $abi:literal $($link_name:literal)? $(#[$doc:meta])? fn $($function:tt)*) => (
        // #[link(name = $library_win, kind = "raw-dylib", modifiers = "+verbatim", import_name_type = "undecorated")]
        #[link(name = $library_dxvk)]
        extern $abi {
            $(#[link_name=$link_name])?
            pub fn $($function)*;
        }
    )
}
