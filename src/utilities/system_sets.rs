use crate::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum TweenHelpersSystemSet {
    PreTargetRemoval,
    TargetRemoval,
}

pub struct BevyTweenHelpersSystemSetsPlugin;


impl Plugin for BevyTweenHelpersSystemSetsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            ((
                 TweenHelpersSystemSet::PreTargetRemoval,
                 TweenHelpersSystemSet::TargetRemoval,
             )
                 .chain(),),
        );
    }
}

