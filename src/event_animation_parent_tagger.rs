use crate::prelude::bevy_time_runner::{TimeRunner, TimeStepMarker};
use crate::prelude::*;

pub struct EventAnimationParentTaggerPlugin;

impl Plugin for EventAnimationParentTaggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            EventAnimationParentTaggerOnSchedulePlugin::<()>::on_schedule(Update.intern()),
        );
    }
}

pub struct EventAnimationParentTaggerOnSchedulePlugin<TimeStep>
where
    TimeStep: Default + Send + Sync + 'static,
{
    pub schedule: InternedScheduleLabel,
    time_step_marker: PhantomData<TimeStep>,
}
impl<TimeStep> EventAnimationParentTaggerOnSchedulePlugin<TimeStep>
where
    TimeStep: Default + Send + Sync + 'static,
{
    pub fn on_schedule(schedule: InternedScheduleLabel) -> Self {
        Self {
            schedule,
            time_step_marker: PhantomData::default(),
        }
    }
}
impl<TimeStep> Plugin for EventAnimationParentTaggerOnSchedulePlugin<TimeStep>
where
    TimeStep: Default + Send + Sync + 'static,
{
    fn build(&self, app: &mut App) {
        app.add_systems(
            self.schedule.clone(),
            tag_animation_parents_with_destruction_marker::<TimeStep>
                .in_set(TweenHelpersSystemSet::PreTargetRemoval),
        );
    }
}

fn tag_animation_parents_with_destruction_marker<TimeStep>(
    new_time_runners: Query<
        (&Children, Entity),
        (Added<TimeRunner>, With<TimeStepMarker<TimeStep>>),
    >,
    event_tweens: Query<(), With<EventEmittingTween>>,
    mut commands: Commands,
) where
    TimeStep: Default + Send + Sync + 'static,
{
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
