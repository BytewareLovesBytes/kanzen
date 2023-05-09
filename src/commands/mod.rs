mod anilist;
mod misc;
mod schedule;

pub fn commands() -> Vec<crate::Command> {
    anilist::commands()
        .into_iter()
        .chain(schedule::commands())
        .chain(misc::commands())
        .collect()
}
