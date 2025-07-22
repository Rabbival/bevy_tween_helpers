use bevy_tween::prelude::ComponentTween;
use bevy_tween::tween::TargetComponent;
use crate::prelude::*;

/// extracts the tweens targets as a vec
pub fn get_tween_targets<T: Sendable>(tween: &ComponentTween<T>) -> Vec<Entity>{
    match &tween.target {
        TargetComponent::Entity(dominant_target) => vec![*dominant_target],
        TargetComponent::Entities(dominant_targets) => dominant_targets.clone(),
        _ => Vec::new()
    }
}