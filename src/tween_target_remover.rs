use crate::{plugin_for_implementors_of_trait, prelude::*, read_single_field_variant};
use tween::{ComponentTween, TargetComponent};

#[derive(Component)]
pub struct TweenTargetOf(pub Entity);

plugin_for_implementors_of_trait!(TweenTargetRemover, Sendable);

impl<T: Sendable> Plugin for TweenTargetRemover<T> {
    fn build(&self, app: &mut App) {
        app.add_message::<RemoveTargetsFromAllTweensOfType<T>>()
            .add_observer(remove_tween_target_on_target_despawn::<T>)
            .add_observer(on_remove_targets_from_tweens_of_type::<T>)
            .add_observer(on_remove_targets_from_all_tweens_targeting_them_request::<T>)
            .add_systems(
                Update,
                (
                    track_newborn_tween_targets::<T>
                        .in_set(TweenHelpersSystemSet::PreTargetRemoval),
                    listen_to_target_removal_requests::<T>
                        .in_set(TweenHelpersSystemSet::TargetRemoval),
                ),
            );
    }
}

fn track_newborn_tween_targets<T: Sendable>(
    newborn_tweens: Query<(&ComponentTween<T>, Entity), Added<ComponentTween<T>>>,
    mut commands: Commands,
) {
    for (tween, tween_entity) in &newborn_tweens {
        for target in get_tween_targets(tween) {
            if let Ok(mut entity_commands) = commands.get_entity(target) {
                entity_commands.try_insert(TweenTargetOf(tween_entity));
            }
        }
    }
}

fn on_remove_targets_from_tweens_of_type<T: Sendable>(
    trigger: On<RemoveTargetsFromAllTweensOfType<T>>,
    mut tweens_of_type: Query<(&mut ComponentTween<T>, Entity, Option<&Name>)>,
    logging_function: Res<TweeningLoggingFunction>,
    mut commands: Commands,
) {
    let entities = &trigger.targets;
    if entities.is_empty() {
        return;
    }
    for (mut tween, tween_entity, maybe_tween_name) in &mut tweens_of_type {
        remove_target_and_destroy_if_has_none(
            entities,
            tween_entity,
            &mut tween,
            maybe_tween_name,
            &logging_function.0,
            &mut commands,
        );
    }
}

fn on_remove_targets_from_all_tweens_targeting_them_request<T: Sendable>(
    trigger: On<TweenRequest>,
    mut tweens_of_type: Query<(&mut ComponentTween<T>, Entity, Option<&Name>)>,
    logging_function: Res<TweeningLoggingFunction>,
    mut commands: Commands,
) {
    if let TweenRequest::RemoveTargetsFromAllTweensTargetingThem(entities) = trigger.event() {
        if entities.is_empty() {
            return;
        }
        for (mut tween, tween_entity, maybe_tween_name) in &mut tweens_of_type {
            remove_target_and_destroy_if_has_none(
                &entities,
                tween_entity,
                &mut tween,
                maybe_tween_name,
                &logging_function.0,
                &mut commands,
            );
        }
    }
}

fn remove_tween_target_on_target_despawn<T: Sendable>(
    trigger: On<Remove, TweenTargetOf>,
    mut query: Query<(&mut ComponentTween<T>, Option<&Name>, Entity)>,
    logging_function: Res<TweeningLoggingFunction>,
    mut commands: Commands,
) {
    for (mut tween, maybe_tween_name, tween_entity) in &mut query {
        remove_target_and_destroy_if_has_none(
            &vec![trigger.entity],
            tween_entity,
            &mut tween,
            maybe_tween_name,
            &logging_function.0,
            &mut commands,
        );
    }
}

fn listen_to_target_removal_requests<T: Sendable>(
    mut tween_request_reader: MessageReader<TweenRequest>,
    mut tweens_of_type: Query<(&mut ComponentTween<T>, Option<&Name>)>,
    logging_function: Res<TweeningLoggingFunction>,
    mut commands: Commands,
) {
    for remove_request in
        read_single_field_variant!(tween_request_reader, TweenRequest::RemoveEntity)
    {
        if let Ok((mut tween, maybe_name)) = tweens_of_type.get_mut(remove_request.tween_entity) {
            remove_target_and_destroy_if_has_none(
                &remove_request.targets_to_remove,
                remove_request.tween_entity,
                &mut tween,
                maybe_name,
                &logging_function.0,
                &mut commands,
            );
        }
    }
}

fn remove_target_and_destroy_if_has_none<T: Sendable>(
    targets_to_match: &Vec<Entity>,
    tween_entity: Entity,
    tween: &mut ComponentTween<T>,
    maybe_tween_name: Option<&Name>,
    logging_function: &Option<fn(String) -> ()>,
    commands: &mut Commands,
) {
    let mut despawned_tween = false;
    match &mut tween.target {
        TargetComponent::Entity(tween_target) => {
            if targets_to_match.contains(tween_target) {
                if let Ok(mut entity_commands) = commands.get_entity(tween_entity) {
                    entity_commands.try_despawn();
                    despawned_tween = true;
                }
            }
        }
        TargetComponent::Entities(tween_targets) => {
            tween_targets.retain(|target| !targets_to_match.contains(target));
            if let Some(logger) = logging_function {
                logger(format!(
                    "removing targets {:?} from tween: {}",
                    targets_to_match,
                    maybe_tween_name.unwrap_or(&Name::new("(nameless)"))
                ));
            }
            if tween_targets.is_empty() {
                if let Ok(mut entity_commands) = commands.get_entity(tween_entity) {
                    entity_commands.try_despawn();
                    despawned_tween = true;
                }
            }
        }
        _ => {}
    }
    if despawned_tween && let Some(logger) = logging_function {
        logger(format!(
            "destroying tween: {}",
            maybe_tween_name.unwrap_or(&Name::new("(nameless)"))
        ));
    }
}
