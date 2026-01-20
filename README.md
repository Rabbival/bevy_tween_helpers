# bevy_tween_helpers

### Optional, additional utilities for the [`bevy_tween`](https://github.com/Multirious/bevy_tween)

## Using The Crate

* First, you should add [BevyTweenHelpersPlugin](src/bevy_tween_helpers_plugin.rs), to which you can add a logging
  function of your choice.
* Then, register each of the following plugins for each interpolator type you wish to apply them to:
    * [AnimationParentDestroyerGenericPlugin](src/animation_parent_destoryer.rs)
        * Automatically despawns animation parents if it has no children left, for example a parent with no tweens
        * You may also add [EventAnimationParentTaggerPlugin](src/event_animation_parent_tagger.rs) to automatically tag animation parents to be destroyed even if they have event-emitting-tween children left
          * Alternatively, you may also tag them manually by adding [[AnimationParentToDestroyIfOnlyHasEventsLeft](src/animation_parent_destoryer.rs) to them
    * [TweenTargetRemover](src/tween_target_remover.rs)
        * Automatically removes entities from tween targets when their `AnimationTarget` component is removed
        * Listens to target removal tween requests and triggers
        * Combining it with `AnimationParentDestroyerGenericPlugin` results in automatic tween and parent clearing
    * [TweenPriorityHandler](src/tween_priority.rs)
        * Handles [TweenPriorityToOthersOfType](src/tween_priority.rs), when tweens or parents have this component,
          fight against other tweens of that type. The ones with the highest priority will survive.
        * If you're not sure what the previous bullet means, read [TweenPriorityToOthersOfType](src/tween_priority.rs)'s
          description

* A non generic plugin that you might useful is [TweenSkipTaggerPlugin](src/tween_skip_tagger.rs), which would listen to skip-tag requesting, allowing you to skip entire animations instead of tagging each tween in them
* I also added my tween combinators, feel free to open PRs requesting to add your own!

### Example

An example for registering the plugins into your app would be:

```rust
    app.add_plugins((
            DefaultTweenPlugins, //from bevy_tween
            BevyTweenHelpersPlugin::default(),
            TweenTargetRemover::<MyGloriousInterpolator>::default(),
            TweenPriorityHandler::<MyGloriousInterpolator>::default(),
            AnimationParentDestroyerGenericPlugin::<MyGloriousInterpolator>::default(),
            EventAnimationParentTaggerPlugin,
            TweenSkipTaggerPlugin
        ))
        .add_tween_systems(component_tween_system::<MyGloriousInterpolator>()); //from bevy_tween
```

### [Change Log](change_logs.md)

## Bevy Version Support

| `bevy` | `bevy_tween_helpers` |
|--------|----------------------|
| 0.17   | 0.2 - 0.4            |
| 0.16   | 0.1                  |

<br>

## Credits

- [`bevy_tween`](https://github.com/Multirious/bevy_tween)
  The crate this one is built upon.
