#![macro_use]

/// Macro to get c strings from literals without runtime overhead
/// Literal must not contain any interior null bytes!
#[macro_export]
macro_rules! c_str {
    ($literal:expr) => {
        CStr::from_bytes_with_nul_unchecked(concat!($literal, "\0").as_bytes())
    };
}

/// Similar to `offset_of` in C/C++
macro_rules! offset_of {
    ($ty:ty, $field:ident) => {
        &(*(ptr::null() as *const $ty)).$field as *const _ as usize
    };
}
