use bevy::ecs::system::ScheduleSystem;
use bevy_tween::combinator::AnimationBuilderExt;
use bevy_tween::interpolation::EaseKind;
use bevy_tween::prelude::{Interpolator, IntoTarget};
use bevy_tween::tween::ComponentTween;
use bevy_tween_helpers::prelude::*;
use std::time::Duration;

#[test]
fn test_two_tween_with_priorities() {
    let mut app = make_app_with_systems(spawn_tweens_with_priorities, assert_destruction_of_b);
    app.update();
}

#[test]
fn test_tween_with_priority_and_one_without() {
    let mut app = make_app_with_systems(
        spawn_tween_with_priority_and_one_without,
        assert_both_still_exist,
    );
    app.update();
}

#[test]
fn test_tween_priority_through_parent() {
    let mut app = make_app_with_systems(
        spawn_tween_with_priority_and_one_with_parent_priority,
        assert_destruction_of_b,
    );
    app.update();
}

fn make_app_with_systems<M, N>(
    setup_system: impl IntoScheduleConfigs<ScheduleSystem, M>,
    assertion_system: impl IntoScheduleConfigs<ScheduleSystem, N>,
) -> App {
    let mut app = App::new();

    app.add_systems(
        Update,
        (
            setup_system.before(TweenHelpersSystemSet::PreTargetRemoval),
            assertion_system.after(TweenHelpersSystemSet::TargetRemoval),
        ),
    );
    app.insert_resource(TweeningLoggingFunction(Some(log)));
    app.add_plugins((
        TweenRequestPlugin,
        BevyTweenHelpersSystemSetsPlugin,
        TweenTargetRemover::<MePolator>::default(),
        TweenPriorityHandler::<MePolator>::default(),
    ));
    app
}

fn log(log_me: String) {
    println!("{}", log_me);
}

fn spawn_tweens_with_priorities(mut commands: Commands) {
    let entity = commands.spawn(()).id();
    let animation_target = entity.into_target();
    let mut state = animation_target.state(());

    commands.spawn(()).animation().insert(tween_with_components(
        TWEEN_DURATION,
        EaseKind::Linear,
        state.with(move |_state| MePolator),
        (TweenPriorityToOthersOfType(10), A),
    ));
    commands.spawn(()).animation().insert(tween_with_components(
        TWEEN_DURATION,
        EaseKind::Linear,
        state.with(move |_state| MePolator),
        (TweenPriorityToOthersOfType(9), B),
    ));
}

fn assert_destruction_of_b(
    a_interpolators: Query<&ComponentTween<MePolator>, With<A>>,
    b_interpolators: Query<&ComponentTween<MePolator>, With<B>>,
) {
    assert_eq!(a_interpolators.iter().count(), 1);
    assert_eq!(b_interpolators.iter().count(), 0);
}

fn spawn_tween_with_priority_and_one_without(mut commands: Commands) {
    let entity = commands.spawn(()).id();
    let animation_target = entity.into_target();
    let mut state = animation_target.state(());

    commands.spawn(()).animation().insert(tween_with_components(
        TWEEN_DURATION,
        EaseKind::Linear,
        state.with(move |_state| MePolator),
        (TweenPriorityToOthersOfType(10), A),
    ));
    commands.spawn(()).animation().insert(tween_with_components(
        TWEEN_DURATION,
        EaseKind::Linear,
        state.with(move |_state| MePolator),
        B,
    ));
}

fn assert_both_still_exist(
    a_interpolators: Query<&ComponentTween<MePolator>, With<A>>,
    b_interpolators: Query<&ComponentTween<MePolator>, With<B>>,
) {
    assert_eq!(a_interpolators.iter().count(), 1);
    assert_eq!(b_interpolators.iter().count(), 1);
}

fn spawn_tween_with_priority_and_one_with_parent_priority(mut commands: Commands) {
    let entity = commands.spawn(()).id();
    let animation_target = entity.into_target();
    let mut state = animation_target.state(());

    commands.spawn(()).animation().insert(tween_with_components(
        TWEEN_DURATION,
        EaseKind::Linear,
        state.with(move |_state| MePolator),
        (TweenPriorityToOthersOfType(10), A),
    ));
    commands
        .spawn(TweenPriorityToOthersOfType(9))
        .animation()
        .insert(tween_with_components(
            TWEEN_DURATION,
            EaseKind::Linear,
            state.with(move |_state| MePolator),
            B,
        ));
}

// utils for these tests

const TWEEN_DURATION: Duration = Duration::from_secs(5);

#[derive(Clone, Copy, Debug, Component)]
struct A;

#[derive(Clone, Copy, Debug, Component)]
struct B;

#[derive(Clone, Copy, Debug)]
struct MePolator;

impl Interpolator for MePolator {
    type Item = ();

    fn interpolate(&self, _item: &mut Self::Item, _value: f32, _previous_value: f32) {}
}
