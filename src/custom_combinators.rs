use std::time::Duration;

use bevy_time_runner::TimeSpan;
use combinator::AnimationCommands;

use crate::prelude::*;

pub fn named_tween<I, T>(
    duration: Duration,
    interpolation: I,
    tween: T,
    name: String,
) -> impl FnOnce(&mut AnimationCommands, &mut Duration)
where
    I: Bundle,
    T: Bundle,
{
    move |a, pos| {
        let start = *pos;
        let end = start + duration;
        a.spawn((
            TimeSpan::try_from(start..end).unwrap(),
            interpolation,
            tween,
            Name::new(name),
        ));
        *pos = end;
    }
}

pub fn tween_with_priority<I, T>(
    duration: Duration,
    interpolation: I,
    tween: T,
    priority: u32,
) -> impl FnOnce(&mut AnimationCommands, &mut Duration)
where
    I: Bundle,
    T: Bundle,
{
    move |a, pos| {
        let start = *pos;
        let end = start + duration;
        a.spawn((
            TimeSpan::try_from(start..end).unwrap(),
            interpolation,
            tween,
            TweenPriorityToOthersOfType(priority),
        ));
        *pos = end;
    }
}

pub fn wait_for(wait_duration: Duration) -> impl FnOnce(&mut AnimationCommands, &mut Duration) {
    move |a, pos| {
        let start = *pos;
        let end = start + wait_duration;
        a.spawn(TimeSpan::try_from(start..end).unwrap());
        *pos = end;
    }
}

pub fn tween_with_components<I, T, B>(
    duration: Duration,
    interpolation: I,
    tween: T,
    additional_components: B,
) -> impl FnOnce(&mut AnimationCommands, &mut Duration)
where
    I: Bundle,
    T: Bundle,
    B: Bundle,
{
    move |a, pos| {
        let start = *pos;
        let end = start + duration;
        a.spawn((
            TimeSpan::try_from(start..end).unwrap(),
            interpolation,
            tween,
            additional_components,
        ));
        *pos = end;
    }
}
