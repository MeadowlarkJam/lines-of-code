use super::{audio_type::PriorityAudioType, AudioType};

#[derive(Debug, PartialEq, Eq)]
pub struct AudioEvent(pub AudioType);

#[derive(Debug, PartialEq, Eq)]
pub struct PriorityAudioEvent(pub PriorityAudioType);
