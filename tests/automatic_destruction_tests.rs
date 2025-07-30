use bevy_tween::combinator::AnimationBuilderExt;
use bevy_tween::interpolation::EaseKind;
use bevy_tween::prelude::{Interpolator, IntoTarget};
use bevy_tween_helpers::prelude::*;
use std::time::Duration;

#[test]
fn test_automatic_tween_destruction() {
    let mut app = App::new();

    app.add_systems(
        Update,
        spawn_tween.before(TweenHelpersSystemSet::PreTargetRemoval),
    );
    app.insert_resource(TweeningLoggingFunction(Some(log)));
    app.add_plugins((
        TweenRequestPlugin,
        BevyTweenHelpersSystemSetsPlugin,
        TweenTargetRemover::<MePolator>::default(),
        AnimationParentDestroyerGenericPlugin::<MePolator>::default(),
    ));

    app.update();

    let tween_before_despawn = app.world_mut().query::<&TweenTag>().iter(app.world()).len();
    let parents_before_despawn = app
        .world_mut()
        .query::<&AnimationParentTag>()
        .iter(app.world())
        .len();

    app.add_systems(
        Update,
        despawn_target_entity
            .after(spawn_tween)
            .before(TweenHelpersSystemSet::PreTargetRemoval),
    );

    app.update();

    let tween_after_despawn = app.world_mut().query::<&TweenTag>().iter(app.world()).len();
    let parents_after_despawn = app
        .world_mut()
        .query::<&AnimationParentTag>()
        .iter(app.world())
        .len();

    assert_eq!(tween_before_despawn, 1);
    assert_eq!(parents_before_despawn, 1);
    assert_eq!(tween_after_despawn, 0);
    assert_eq!(parents_after_despawn, 0);
}

fn log(log_me: String) {
    println!("{}", log_me);
}

fn spawn_tween(mut commands: Commands, target_entities: Query<&TargetEntityTag>) {
    if !target_entities.is_empty() {
        return;
    }
    let entity = commands.spawn(TargetEntityTag).id();
    let animation_target = entity.into_target();
    let mut state = animation_target.state(());

    commands
        .spawn(AnimationParentTag)
        .animation()
        .insert(tween_with_components(
            Duration::from_secs_f32(30.0),
            EaseKind::Linear,
            state.with(move |_state| MePolator),
            TweenTag,
        ));
}

fn despawn_target_entity(
    target_entities: Query<Entity, With<TargetEntityTag>>,
    mut commands: Commands,
) {
    for target in &target_entities {
        commands.entity(target).despawn();
    }
}

#[derive(Component)]
struct TargetEntityTag;

#[derive(Component)]
struct TweenTag;

#[derive(Component)]
struct AnimationParentTag;

#[derive(Clone, Copy, Debug)]
struct MePolator;

impl Interpolator for MePolator {
    type Item = ();

    fn interpolate(&self, _item: &mut Self::Item, _value: f32, _previous_value: f32) {}
}
