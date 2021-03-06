/// Shorthand macros for use inside editor.with() closures.
#[macro_export]
macro_rules! get_contour {
    ($v:ident, $idx:expr) => {
        $v.outline[$idx].inner
    };
}

#[macro_export]
macro_rules! get_contour_mut {
    ($v:ident, $idx:expr) => {
        &mut $v.outline[$idx].inner
    };
}

#[macro_export]
macro_rules! get_contour_len {
    ($v:ident, $idx:expr) => {
        $v.outline[$idx].inner.len()
    };
}

#[macro_export]
macro_rules! get_contour_type {
    ($v:ident, $idx:expr) => {
        $v.outline[$idx].inner.first().unwrap().ptype
    };
}

#[macro_export]
macro_rules! get_point {
    ($v:ident, $cidx:expr, $pidx:expr) => {
        $v.outline[$cidx].inner[$pidx]
    };
}

// This re-import is here because I think it's messy to refer to these macros using the top-level
// crate::. This allows me to have in modules e.g. `use crate::editor::macros::get_point`, which is
// our preferred way of importing them.
pub use {get_contour, get_contour_mut, get_contour_len, get_contour_type, get_point};
