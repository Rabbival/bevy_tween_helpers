use crate::plugin_for_implementors_of_trait;
use crate::prelude::*;
use bevy::prelude::Component;
use bevy_tween::prelude::ComponentTween;

/// When there's a conflict between two existing tweens of the same type
/// (say, two position tweens on the same entity)
/// one of them is destroyed (either the one with the lesser priority and if equal- the older one).
///
/// This component can be attached to either the parent animation
/// (applies to all tween children) or the specific tween.
/// If the tween has a specified priority, it overrides that of its parent
///
/// Possible bug causer to be aware of:
/// Note that if you spawn two tweens with the same priority at the same time, both will be destoryed.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct TweenPriorityToOthersOfType(pub u32);

plugin_for_implementors_of_trait!(TweenPriorityHandler, Sendable);

impl<T: Sendable> Plugin for TweenPriorityHandler<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_tween_priority_on_spawn::<T>.in_set(TweenHelpersSystemSet::PreTargetRemoval),
        );
    }
}

/// The entire logic of keeping one tween over the other only runs when a new tween with priority is spawned
/// or a new tween is spawned as a child to a parent with a priority. If a tween has no `TweenPriorityToOthersOfType`,
/// the tween priority logic ignores it.
fn handle_tween_priority_on_spawn<T: Sendable>(
    mut tween_request_writer: EventWriter<TweenRequest>,
    tween_priorities_query: Query<&TweenPriorityToOthersOfType>,
    all_tweens_of_type: Query<(
        &ComponentTween<T>,
        &ChildOf,
        Option<&TweenPriorityToOthersOfType>,
        Entity,
    )>,
    newborn_tweens_query: Query<
        (
            &ComponentTween<T>,
            &ChildOf,
            Entity,
            Option<&TweenPriorityToOthersOfType>,
            Option<&Name>,
        ),
        Added<ComponentTween<T>>,
    >,
    logging_function: Res<TweeningLoggingFunction>,
) {
    for (newborn_tween, child_of, newborn_tween_entity, maybe_tween_priority, maybe_tween_name) in
        &newborn_tweens_query
    {
        let maybe_priority = if let Some(tween_priority) = maybe_tween_priority {
            Some(tween_priority)
        } else if let Ok(parent_priority) = tween_priorities_query.get(child_of.parent()) {
            Some(parent_priority)
        } else {
            None
        };
        if let Some(priority) = maybe_priority {
            if let Some(logger) = logging_function.0 {
                logger(format!(
                    "{} spawned, looking for tweens to destroy by priority",
                    maybe_tween_name.unwrap_or(&Name::new("A nameless tween with priority"))
                ));
            }
            handle_tween_priority_to_others_of_type(
                &mut tween_request_writer,
                priority,
                newborn_tween,
                newborn_tween_entity,
                child_of,
                &all_tweens_of_type,
                &tween_priorities_query,
            );
        }
    }
}

fn handle_tween_priority_to_others_of_type<T: Sendable>(
    tween_request_writer: &mut EventWriter<TweenRequest>,
    tween_priority: &TweenPriorityToOthersOfType,
    newborn_tween: &ComponentTween<T>,
    newborn_tween_entity: Entity,
    newborn_tween_child_of: &ChildOf,
    all_tweens_of_type: &Query<(
        &ComponentTween<T>,
        &ChildOf,
        Option<&TweenPriorityToOthersOfType>,
        Entity,
    )>,
    tween_priorities_query: &Query<&TweenPriorityToOthersOfType>,
) {
    for (other_tween, child_of, maybe_other_priority, other_tween_entity) in all_tweens_of_type {
        let sibling_tweens = newborn_tween_child_of.parent() == child_of.parent();
        if other_tween_entity != newborn_tween_entity && !sibling_tweens {
            if let Some(other_priority_level) = try_get_other_tween_priority(
                maybe_other_priority,
                child_of.parent(),
                tween_priorities_query,
            ) {
                if other_priority_level <= tween_priority.0 {
                    remove_intersecting_targets_for_weaker_tween(
                        tween_request_writer,
                        newborn_tween,
                        other_tween_entity,
                    );
                } else {
                    remove_intersecting_targets_for_weaker_tween(
                        tween_request_writer,
                        other_tween,
                        newborn_tween_entity,
                    );
                }
            }
        }
    }
}

fn try_get_other_tween_priority(
    maybe_other_tween_priority: Option<&TweenPriorityToOthersOfType>,
    other_tween_parent_entity: Entity,
    tween_policies_query: &Query<&TweenPriorityToOthersOfType>,
) -> Option<u32> {
    match maybe_other_tween_priority {
        Some(TweenPriorityToOthersOfType(other_priority_level)) => Some(*other_priority_level),
        _ => match tween_policies_query.get(other_tween_parent_entity) {
            Ok(TweenPriorityToOthersOfType(other_parent_priority_level)) => {
                Some(*other_parent_priority_level)
            }
            _ => None,
        },
    }
}

pub fn remove_intersecting_targets_for_weaker_tween<T: Sendable>(
    tween_request_writer: &mut EventWriter<TweenRequest>,
    dominant_tween: &ComponentTween<T>,
    weaker_tween_entity: Entity,
) {
    let dominant_tween_targets = get_tween_targets(dominant_tween);
    if !dominant_tween_targets.is_empty() {
        tween_request_writer.write(TweenRequest::RemoveEntity(RemoveTweenTargets {
            tween_entity: weaker_tween_entity,
            targets_to_remove: dominant_tween_targets,
        }));
    }
}
