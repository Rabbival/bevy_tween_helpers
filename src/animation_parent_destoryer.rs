use crate::plugin_for_implementors_of_trait;
use crate::prelude::*;
use bevy_time_runner::TimeRunnerEnded;
use bevy_tween::bevy_time_runner::{TimeContext, TimeRunner};
use bevy_tween::prelude::ComponentTween;

#[derive(Component)]
pub struct AnimationParentToDestroyIfOnlyHasEventsLeft;

pub struct AnimationParentDestroyerPlugin;

impl Plugin for AnimationParentDestroyerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AnimationParentDestroyerOnSchedulePlugin::<()>::on_schedule(
            Update.intern(),
        ));
    }
}

pub struct AnimationParentDestroyerOnSchedulePlugin<TimeCtx>
where
    TimeCtx: Default + Send + Sync + 'static,
{
    pub schedule: InternedScheduleLabel,
    time_step_marker: PhantomData<TimeCtx>,
}
impl<TimeCtx> AnimationParentDestroyerOnSchedulePlugin<TimeCtx>
where
    TimeCtx: Default + Send + Sync + 'static,
{
    pub fn on_schedule(schedule: InternedScheduleLabel) -> Self {
        Self {
            schedule,
            time_step_marker: PhantomData::default(),
        }
    }
}
impl<TimeCtx> Plugin for AnimationParentDestroyerOnSchedulePlugin<TimeCtx>
where
    TimeCtx: Default + Send + Sync + 'static,
{
    fn build(&self, app: &mut App) {
        app.add_systems(self.schedule.clone(), despawn_done_time_runners::<TimeCtx>);
    }
}

plugin_for_implementors_of_trait!(AnimationParentDestroyerGenericPlugin, Sendable);

impl<T: Sendable> Plugin for AnimationParentDestroyerGenericPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_observer(despawn_time_runners_with_no_children::<T>);
    }
}

pub fn despawn_done_time_runners<TimeCtx>(
    mut time_runner_ended_reader: MessageReader<TimeRunnerEnded>,
    time_step_marked: Query<(), With<TimeContext<TimeCtx>>>,
    mut commands: Commands,
) where
    TimeCtx: Default + Send + Sync + 'static,
{
    for event in time_runner_ended_reader.read() {
        if event.is_completed() && time_step_marked.contains(event.entity) {
            if let Ok(mut entity_commands) = commands.get_entity(event.entity) {
                entity_commands.try_despawn();
            }
        }
    }
}

pub fn despawn_time_runners_with_no_children<T: Sendable>(
    trigger: On<Remove, ComponentTween<T>>,
    time_runners: Query<
        (
            &Children,
            Entity,
            Has<AnimationParentToDestroyIfOnlyHasEventsLeft>,
        ),
        With<TimeRunner>,
    >,
    event_tweens: Query<(), With<EventEmittingTween>>,
    mut commands: Commands,
) {
    'time_runners_for: for (
        time_runner_children,
        time_runner_entity,
        should_destroy_if_only_has_events,
    ) in &time_runners
    {
        for child in time_runner_children.iter() {
            if child != trigger.entity
                && (!should_destroy_if_only_has_events || !event_tweens.contains(child))
            {
                continue 'time_runners_for;
            }
        }
        if let Ok(mut entity_commands) = commands.get_entity(time_runner_entity) {
            entity_commands.try_despawn();
        }
    }
}
