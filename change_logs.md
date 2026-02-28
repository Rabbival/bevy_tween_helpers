## Change Logs

[(Click me to go back to the main readme)](README.md)

### unreleased

* IMPORTANT: `AnimationParentDestroyerPlugin` is no longer registered defaultly by `BevyTweenHelpersPlugin`
* You may now register the `BevyTweenHelpersOnSchedulesPlugin` instead of `BevyTweenHelpersPlugin`,
  which would allow you to define the system sets on different schedules
* You may now register plugins for specific schedule and time step:
  * `AnimationParentDestroyerOnSchedulePlugin`
  * `EventAnimationParentTaggerOnSchedulePlugin`
  * `TweenPriorityHandlerOnSchedule`
  * `TweenTargetRemoverOnSchedule`
* Make `handle_tween_priority_on_spawn` observer instead of scheduled system
* Replace `TweenTargetOf` with `TargetingTweens` as a single entity may be targetted by many tweens
* `TweenSkipTaggerPlugin` now only registers the tween event on `PostUpdate`. If you want it to listen to requests from tweens of time runners that run on other schedules, you'd have to `.add_plugins(TweenEventPlugin::<TweenSkipTagTweenRequest>::in_schedule(
     YOUR_SCHEDULE_INTERN_HERE
  ))`
* Update to bevy 0.18

### 0.4.0

* Add `TweenSkipTaggerPlugin` for automatic skipping of entire animations instead of tagging each tween manually

### 0.3.0

* Fix logic that destroys animation parents with just event-emitting-tweens left to work for custom user events as well
* Add `EventAnimationParentTaggerPlugin` which, if added to the app, automatically tags animation parents with event-emitting-tween children to be destroyed if they only have the event emitter children left

### 0.2.5-6 - yanked due to logic bug, see fixed changed in 0.3.0

### 0.2.4

* Add `AnimationParentToDestroyIfOnlyHasEventsLeft` to let users mark animation parents that should be destroyed if they only have `event()` tweens left

### 0.2.3

* Add `RemoveTargetsFromAllTweensOfType<T: Sendable>` to let users request removal of targets from tweens of specific types

### 0.2.1

* Add `ExtraTransformTweenMakers`, which are currently "delta to" tweens for transform

### 0.2.0

* Update to bevy 0.17

### 0.1.2 -> 0.1.3

* Tag tween targets with `TweenTargetOf` so that they can be automatically tracked
* add `tween_with_components` and `wait_for` custom combinators

### 0.1.1 -> 0.1.2

* Expose `get_tween_targets` function
* Add `tween_with_priority` custom combinator

[(Click me to go back to the main readme)](README.md)
