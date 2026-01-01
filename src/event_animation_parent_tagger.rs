use crate::prelude::bevy_time_runner::TimeRunner;
use crate::prelude::*;

pub struct EventAnimationParentTaggerPlugin;

impl Plugin for EventAnimationParentTaggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            tag_animation_parents_with_destruction_marker
                .in_set(TweenHelpersSystemSet::PreTargetRemoval),
        );
    }
}

fn tag_animation_parents_with_destruction_marker(
    new_time_runners: Query<(&Children, Entity), Added<TimeRunner>>,
    event_tweens: Query<(), With<EventEmittingTween>>,
    mut commands: Commands,
) {
    for (tween_children, parent_entity) in &new_time_runners {
        for tween_entity in tween_children.iter() {
            if event_tweens.contains(tween_entity) {
                commands
                    .entity(parent_entity)
                    .try_insert(AnimationParentToDestroyIfOnlyHasEventsLeft);
                break;
            }
        }
    }
}
