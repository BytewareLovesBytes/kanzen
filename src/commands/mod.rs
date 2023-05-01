mod anilist;
mod schedule;

pub fn commands() -> Vec<crate::Command> {
    anilist::commands()
        .into_iter()
        .chain(schedule::commands())
        .collect()
}
