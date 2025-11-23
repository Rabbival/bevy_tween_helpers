use crate::prelude::*;

#[derive(Debug, Clone, Message, Event)]
pub enum TweenRequest {
    RemoveEntity(RemoveTweenTargets),
    RemoveTargetsFromAllTweensTargetingThem(Vec<Entity>),
}

#[derive(Debug, Clone)]
pub struct RemoveTweenTargets {
    pub tween_entity: Entity,
    pub targets_to_remove: Vec<Entity>,
}

#[derive(Debug, Clone, Message, Event, Default)]
pub struct RemoveTargetsFromAllTweensOfType<T: Sendable> {
    pub targets: Vec<Entity>,
    _phantom: PhantomData<T>,
}

pub struct TweenRequestPlugin;

impl Plugin for TweenRequestPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<TweenRequest>();
    }
}
