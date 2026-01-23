use crate::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum TweenHelpersSystemSet {
    PreTargetRemoval,
    TargetRemoval,
}

pub struct BevyTweenHelpersSystemSetsPlugin;

impl Plugin for BevyTweenHelpersSystemSetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BevyTweenHelpersSystemSetsForSchedulesPlugin {
            schedules: vec![Update.intern()],
        });
    }
}

pub struct BevyTweenHelpersSystemSetsForSchedulesPlugin {
    pub schedules: Vec<InternedScheduleLabel>,
}

impl Plugin for BevyTweenHelpersSystemSetsForSchedulesPlugin {
    fn build(&self, app: &mut App) {
        for schedule in self.schedules.clone() {
            app.configure_sets(
                schedule,
                ((
                    TweenHelpersSystemSet::PreTargetRemoval,
                    TweenHelpersSystemSet::TargetRemoval,
                )
                    .chain(),),
            );
        }
    }
}
