use crate::prelude::*;
use bevy_tween::{
    combinator::TransformTargetState,
    interpolate::{Rotation, Scale, Translation},
    prelude::ComponentTween,
};

pub trait ExtraTransformTweenMakers {
    fn translation_delta_to(&mut self, to: Vec3) -> ComponentTween<Translation>;

    fn scale_delta_to(&mut self, to: Vec3) -> ComponentTween<Scale>;

    fn rotation_delta_to(&mut self, to: Quat) -> ComponentTween<Rotation>;
}

impl ExtraTransformTweenMakers for TransformTargetState {
    fn translation_delta_to(&mut self, to: Vec3) -> ComponentTween<Translation> {
        self.translation_with(translation_delta_to(to))
    }

    fn scale_delta_to(&mut self, to: Vec3) -> ComponentTween<Scale> {
        self.scale_with(scale_delta_to(to))
    }

    fn rotation_delta_to(&mut self, to: Quat) -> ComponentTween<Rotation> {
        self.rotation_with(rotation_delta_to(to))
    }
}

pub fn translation_delta_to(to: Vec3) -> impl Fn(&mut Vec3) -> Translation {
    move |state| {
        let start = *state;
        let end = to;
        Translation {
            start,
            end,
            delta: true,
        }
    }
}

pub fn scale_delta_to(to: Vec3) -> impl Fn(&mut Vec3) -> Scale {
    move |state| {
        let start = *state;
        let end = to;
        Scale {
            start,
            end,
            delta: true,
        }
    }
}

pub fn rotation_delta_to(to: Quat) -> impl Fn(&mut Quat) -> Rotation {
    move |state| {
        let start = *state;
        let end = to;
        Rotation {
            start,
            end,
            delta: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_tween::{combinator::parallel, prelude::*};

    #[test]
    fn test_translation_delta_to_1() {
        test_translation_delta_to_inner(Vec3::ONE, Vec3::ONE, Vec3::splat(2.0));
    }

    #[test]
    fn test_translation_delta_to_2() {
        test_translation_delta_to_inner(Vec3::ONE, Vec3::NEG_ONE, Vec3::ZERO);
    }

    fn test_translation_delta_to_inner(
        first_to: Vec3,
        second_to: Vec3,
        expected_final_location: Vec3,
    ) {
        #[derive(Component)]
        struct MovedEntityTag;

        let tween_duration = Duration::from_secs_f32(1.0);

        let mut app = App::new();

        app.init_resource::<Time>().add_plugins(DefaultTweenPlugins);

        let entity_to_move = app
            .world_mut()
            .spawn((MovedEntityTag, Transform::default()))
            .id();
        let animation_target = entity_to_move.into_target();
        let mut transform_state = animation_target.transform_state(Transform::default());

        app.world_mut()
            .commands()
            .spawn(())
            .animation()
            .insert(parallel((
                named_tween(
                    tween_duration,
                    EaseKind::Linear,
                    transform_state.translation_delta_to(first_to),
                    String::from("first"),
                ),
                named_tween(
                    tween_duration,
                    EaseKind::Linear,
                    transform_state.translation_delta_to(second_to),
                    String::from("second"),
                ),
            )));

        app.world_mut()
            .resource_mut::<Time>()
            .advance_by(tween_duration);
        app.update();

        assert_eq!(
            app.world_mut()
                .query_filtered::<&Transform, With<MovedEntityTag>>()
                .single(app.world())
                .unwrap()
                .translation,
            expected_final_location
        );
    }
}
