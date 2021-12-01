//== Include comments ==========================================================
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
// --snip--

//== Basics ====================================================================
// $ cargo doc --open

// `///` is documentation comment
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, my_crate::add_one(5));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

//== General infos =============================================================
// - Panics
// - Errors
// - Safety: when unsafe

//== Test documentation comments  ==============================================
// $ cargo test



