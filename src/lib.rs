//! # `Antler`
//!
//! `Antler` is a textureless rendering engine designed for high-quality rendering of 3D scenes.

// #![deny(absolute_paths_not_starting_with_crate)]
// #![deny(ambiguous_negative_literals)]
// #![deny(dead_code)]
// #![deny(deprecated_safe_2024)]
// #![deny(deref_into_dyn_supertrait)]
// #![deny(edition_2024_expr_fragment_specifier)]
// #![deny(elided_lifetimes_in_paths)]
// #![deny(explicit_outlives_requirements)]
// #![deny(ffi_unwind_calls)]
// #![deny(future_incompatible)]
// #![deny(if_let_rescope)]
// #![deny(impl_trait_overcaptures)]
// #![deny(impl_trait_redundant_captures)]
// #![deny(improper_ctypes)]
// #![deny(keyword_idents_2018)]
// #![deny(keyword_idents_2024)]
// #![deny(keyword_idents)]
// #![deny(let_underscore_drop)]
// #![deny(macro_use_extern_crate)]
// #![deny(meta_variable_misuse)]
// #![deny(missing_copy_implementations)]
// #![deny(missing_debug_implementations)]
// #![deny(missing_docs)]
// #![deny(missing_unsafe_on_extern)]
// #![deny(non_ascii_idents)]
// #![deny(nonstandard_style)]
// #![deny(path_statements)]
// #![deny(redundant_imports)]
// #![deny(redundant_lifetimes)]
// #![deny(rustdoc::broken_intra_doc_links)]
// #![deny(single_use_lifetimes)]
// #![deny(tail_expr_drop_order)]
// #![deny(trivial_casts)]
// #![deny(trivial_numeric_casts)]
// #![deny(unit_bindings)]
// #![deny(unnameable_types)]
// #![deny(unreachable_code)]
// #![deny(unreachable_pub)]
// #![deny(unsafe_attr_outside_unsafe)]
// #![deny(unsafe_code)]
// #![deny(unsafe_op_in_unsafe_fn)]
// #![deny(unstable_features)]
// #![deny(unused_assignments)]
// #![deny(unused_crate_dependencies)]
// #![deny(unused_extern_crates)]
// #![deny(unused_import_braces)]
// #![deny(unused_imports)]
// #![deny(unused_lifetimes)]
// #![deny(unused_macro_rules)]
// #![deny(unused_must_use)]
// #![deny(unused_mut)]
// #![deny(unused_qualifications)]
// #![deny(unused_results)]
// #![deny(unused_variables)]
// #![deny(unused)]
// #![deny(variant_size_differences)]
// #![deny(warnings)]
// #![deny(clippy::all)]
// #![deny(clippy::cargo)]
// #![deny(clippy::complexity)]
// #![deny(clippy::correctness)]
// #![deny(clippy::nursery)]
// #![deny(clippy::pedantic)]
// #![deny(clippy::perf)]
// #![deny(clippy::restriction)]
// #![deny(clippy::style)]
// #![deny(clippy::suspicious)]
// #![allow(
//     clippy::arbitrary_source_item_ordering,
//     reason = "Alphabetical ordering is not always the most readable."
// )]
// #![allow(clippy::arithmetic_side_effects, reason = "Too restrictive for this crate.")]
// #![allow(clippy::blanket_clippy_restriction_lints, reason = "Prefer more lints.")]
// #![allow(clippy::default_numeric_fallback, reason = "Numeric type fallback should not be required.")]
// #![allow(clippy::else_if_without_else, reason = "Eliding final else is idiomatic in Rust.")]
// #![allow(clippy::float_arithmetic, reason = "Too restrictive for this crate.")]
// #![allow(clippy::implicit_return, reason = "Implicit returns are idiomatic in Rust.")]
// #![allow(clippy::indexing_slicing, reason = "Too restrictive for this crate.")]
// #![allow(clippy::integer_division_remainder_used, reason = "Too restrictive for this crate.")]
// #![allow(
//     clippy::min_ident_chars,
//     reason = "Whilst short variable names are not always ideal they are often clear in context."
// )]
// #![allow(
//     clippy::missing_trait_methods,
//     reason = "Traits should be able to provide default method implementations."
// )]
// #![allow(clippy::mod_module_files, reason = "Prefer to use mod.rs files for consistency.")]
// #![allow(clippy::pub_use, reason = "It is intended to expose some types at the crate level.")]
// #![allow(
//     clippy::pub_with_shorthand,
//     reason = "Rustfmt automatically shortens pub(in crate) to pub(crate)."
// )]
// #![allow(clippy::question_mark_used, reason = "The question mark operator is idiomatic in Rust.")]
// #![allow(
//     clippy::separated_literal_suffix,
//     reason = "Must chose between separated and unseparated literal suffixes."
// )]
// #![allow(clippy::std_instead_of_core, reason = "Prefer std for consistency.")]
// #![allow(
//     clippy::unreadable_literal,
//     reason = "Prefer no underscores in numeric literals for consistency."
// )]
// #![allow(clippy::unwrap_in_result, reason = "In some cases unwrap can be guaranteed to succeed.")]
// #![allow(clippy::unwrap_used, reason = "In some cases unwrap can be guaranteed to succeed.")]

mod camera;
mod geometry;

pub mod prelude {
    pub use crate::{
        camera::Camera,
        geometry::{Aabb, Mesh, Ray, Triangle},
    };
}
