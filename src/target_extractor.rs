use crate::prelude::*;
use bevy_tween::prelude::ComponentTween;
use bevy_tween::tween::TargetComponent;

/// extracts the tweens targets as a vec
pub fn get_tween_targets<T: Sendable>(tween: &ComponentTween<T>) -> Vec<Entity> {
    match &tween.target {
        TargetComponent::Entity(target) => vec![*target],
        TargetComponent::Entities(targets) => targets.clone(),
        _ => Vec::new(),
    }
}
