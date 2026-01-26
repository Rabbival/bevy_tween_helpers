use crate::prelude::bevy_time_runner::TimeRunner;
use crate::prelude::*;
use bevy_tween::tween::{SkipTween, TweenInterpolationValue};

#[derive(Debug, Clone, Message, EntityEvent)]
pub struct TweenSkipTagRequest {
    #[event_target]
    pub animation_parent: Entity,
    pub request_type: TweenSkipTagRequestType,
}

#[derive(Debug, Clone, Default)]
pub struct TweenSkipTagTweenRequest {
    pub animation_parent: Option<Entity>,
    pub request_type: TweenSkipTagRequestType,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum TweenSkipTagRequestType {
    #[default]
    Insert,
    Remove,
}

pub struct TweenSkipTaggerPlugin;

impl Plugin for TweenSkipTaggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TweenEventPlugin::<TweenSkipTagTweenRequest>::default())
            .add_observer(listen_to_regular_event_trigger)
            .add_observer(listen_to_triggers_from_tweens);
    }
}

fn listen_to_regular_event_trigger(
    trigger: On<TweenSkipTagRequest>,
    animation_parents: Query<&Children, With<TimeRunner>>,
    tweens: Query<(), With<TweenInterpolationValue>>,
    mut commands: Commands,
) {
    tag_or_untag_to_skip_by_request_type(
        trigger.animation_parent,
        trigger.request_type,
        &animation_parents,
        &tweens,
        &mut commands,
    );
}

fn listen_to_triggers_from_tweens(
    trigger: On<TweenEvent<TweenSkipTagTweenRequest>>,
    animation_parents: Query<&Children, With<TimeRunner>>,
    tweens: Query<(), With<TweenInterpolationValue>>,
    mut commands: Commands,
) {
    if let Some(animation_parent) = trigger.data.animation_parent {
        tag_or_untag_to_skip_by_request_type(
            animation_parent,
            trigger.data.request_type,
            &animation_parents,
            &tweens,
            &mut commands,
        );
    }
}

fn tag_or_untag_to_skip_by_request_type(
    animation_parent: Entity,
    request_type: TweenSkipTagRequestType,
    animation_parents: &Query<&Children, With<TimeRunner>>,
    tweens: &Query<(), With<TweenInterpolationValue>>,
    commands: &mut Commands,
) {
    if let Ok(children) = animation_parents.get(animation_parent) {
        for child_entity in children.iter() {
            if tweens.contains(child_entity) {
                match request_type {
                    TweenSkipTagRequestType::Insert => {
                        commands.entity(child_entity).try_insert(SkipTween);
                    }
                    TweenSkipTagRequestType::Remove => {
                        commands.entity(child_entity).try_remove::<SkipTween>();
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_tween::prelude::*;

    #[test]
    fn test_skip_tagging_and_untagging() {
        #[derive(Component)]
        struct MovedEntityTag;
        #[derive(Component)]
        struct AnimationParent;

        let tween_duration = Duration::from_secs_f32(3.0);
        let movement_target = Vec3::splat(3.0);

        let mut app = App::new();

        app.init_resource::<Time>()
            .add_plugins((DefaultTweenPlugins, TweenSkipTaggerPlugin));

        let entity_to_move = app
            .world_mut()
            .spawn((MovedEntityTag, Transform::default()))
            .id();
        let animation_target = entity_to_move.into_target();
        let mut transform_state = animation_target.transform_state(Transform::default());

        let animation_parent = app.world_mut().commands().spawn(AnimationParent).id();
        app.world_mut()
            .commands()
            .entity(animation_parent)
            .animation()
            .insert(named_tween(
                tween_duration,
                EaseKind::Linear,
                transform_state.translation_delta_to(movement_target),
                String::from("mover tween"),
            ));

        app.world_mut()
            .resource_mut::<Time>()
            .advance_by(tween_duration / 3);
        app.update();
        let location_after_first_half = app
            .world_mut()
            .query_filtered::<&Transform, With<MovedEntityTag>>()
            .single(app.world())
            .unwrap()
            .translation;

        app.world_mut().commands().trigger(TweenSkipTagRequest {
            animation_parent,
            request_type: TweenSkipTagRequestType::Insert,
        });

        app.world_mut()
            .resource_mut::<Time>()
            .advance_by(tween_duration / 3);
        app.update();
        let location_after_skip_and_advancement = app
            .world_mut()
            .query_filtered::<&Transform, With<MovedEntityTag>>()
            .single(app.world())
            .unwrap()
            .translation;

        app.world_mut().commands().trigger(TweenSkipTagRequest {
            animation_parent,
            request_type: TweenSkipTagRequestType::Remove,
        });

        app.world_mut()
            .resource_mut::<Time>()
            .advance_by(tween_duration / 3);
        app.update();
        let location_after_finishing_tween = app
            .world_mut()
            .query_filtered::<&Transform, With<MovedEntityTag>>()
            .single(app.world())
            .unwrap()
            .translation;

        assert_eq!(location_after_first_half, Vec3::ONE);
        assert_eq!(location_after_skip_and_advancement, Vec3::ONE);
        assert_eq!(location_after_finishing_tween, movement_target);
    }
}
