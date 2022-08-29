#[derive(Debug, PartialEq, Eq)]
pub enum AudioType {
    Hit,
    Laser,
    Explosion,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PriorityAudioType {
    Intro,
    Music,
    Death,
}
