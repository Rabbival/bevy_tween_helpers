#![allow(clippy::type_complexity)]

/*!
## Using The Crate

* First, you should add `BevyTweenHelpersPlugin`, to which you can add a logging function of your choice.
* Then, register each of the following plugins for each interpolator type you wish to apply them to:
  * `AnimationParentDestroyerGenericPlugin`
    * Automatically despawns animation parents if it has no children left, for example a parent with no tweens
  * `TweenTargetRemover`
    * Automatically removes entities from tween targets when their `AnimationTarget` component is removed
    * Listens to target removal tween requests and triggers
    * Combining it with `AnimationParentDestroyerGenericPlugin` results in automatic tween and parent clearing
  * `TweenPriorityHandler`
    * Handles `TweenPriorityToOthersOfType`, when tweens or parents have this component,
    fight against other tweens of that type. The ones with the highest priority will survive.
    * If you're not sure what the previous bullet means, read `TweenPriorityToOthersOfType`'s description

* I also added my tween combinators, feel free to open PRs requesting to add your own!


### Example

An example for registering the plugins into your app would be:
```rust
    app.add_plugins((
            DefaultTweenPlugins, //from bevy_tween
            BevyTweenHelpersPlugin,
            TweenTargetRemover::<MyGloriousInterpolator>::default(),
            TweenPriorityHandler::<MyGloriousInterpolator>::default(),
            AnimationParentDestroyerGenericPlugin::<MyGloriousInterpolator>::default(),
        ))
        .add_tween_systems(component_tween_system::<MyGloriousInterpolator>()); //from bevy_tween
```
*/

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
