#![allow(clippy::type_complexity)]


pub mod animation_parent_destoryer;
pub mod custom_combinators;
pub mod tween_priority;
pub mod tween_request;
pub mod tween_target_remover;
pub mod bevy_tween_helpers_plugin;

#[macro_use]
pub mod macros;
pub mod utilities;

pub mod prelude {
    pub use crate::animation_parent_destoryer::*;
    pub use crate::custom_combinators::*;
    pub use crate::tween_priority::*;
    pub use crate::tween_request::*;
    pub use crate::tween_target_remover::*;
    pub use crate::bevy_tween_helpers_plugin::*;
    pub use crate::utilities::{system_sets::*, trait_unions::*};
    pub use bevy::{platform::collections::HashMap, prelude::*};
    pub use bevy_tween::*;
    pub use std::marker::PhantomData;
    pub use tween_event::*;
}
