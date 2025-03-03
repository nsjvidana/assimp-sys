#[macro_use]
extern crate bitflags;
extern crate libc;

pub use anim::*;
pub use camera::*;
pub use cexport::*;
pub use cfileio::*;
pub use cimport::*;
pub use importerdesc::*;
pub use light::*;
pub use material::*;
pub use mesh::*;
pub use metadata::*;
pub use postprocess::*;
pub use scene::*;
pub use texture::*;
pub use crate::types::*;
pub use version::*;

/// A macro that makes a bitflag struct's Self consts into consts of the module it is in.
/// This lets the assimp crate do CONSTANT_NAME instead of Struct::CONSTANT_NAME
macro_rules! mod_consts_bitflag {
    (
        $(#[$outer:meta])*
        $vis:vis struct $BitFlags:ident: $T:ty {
            $(
                $(#[$inner:ident $($args:tt)*])*
                const $Flag:tt = $value:expr;
            )*
        }

        $($t:tt)*
    ) => {
        $(
            $(#[$inner $($args)*])*
            $vis const $Flag: $BitFlags = $BitFlags::$Flag;
        )*
        bitflags! {
            $(#[$outer])*
            $vis struct $BitFlags: $T {
                $(
                    $(#[$inner $($args:tt)*])*
                    const $Flag = $value;
                )*
            }

            $($t)*
        }
    };
}
 
mod anim;
mod camera;
mod cexport;
mod cfileio;
mod cimport;
pub mod config;
mod importerdesc;
mod light;
mod material;
mod mesh;
mod metadata;
mod postprocess;
mod scene;
mod texture;
mod types;
mod version;
