mod anilist;

pub fn commands() -> Vec<crate::Command> {
    anilist::commands().into_iter().collect()
}
