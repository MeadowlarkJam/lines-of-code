use super::GameState;
use bevy::prelude::*;
use std::collections::VecDeque;

pub struct ScheduleTimer(pub Timer);

#[derive(Default)]
pub struct ScheduleQueue(pub VecDeque<GameState>);
