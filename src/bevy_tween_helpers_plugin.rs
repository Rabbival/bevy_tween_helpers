use crate::prelude::*;

#[derive(Debug, Default)]
pub struct BevyTweenHelpersPlugin {
    /// Here you can insert your own function for logging BevyTweenHelpersPlugin
    pub logging_function: Option<fn(String)>,
}

#[derive(Resource)]
pub struct TweeningLoggingFunction(pub Option<fn(String)>);

impl Plugin for BevyTweenHelpersPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TweeningLoggingFunction(self.logging_function.clone()))
            .add_plugins((
                TweenRequestPlugin,
                AnimationParentDestroyerPlugin,
                BevyTweenHelpersSystemSetsPlugin
            ));
    }
}
