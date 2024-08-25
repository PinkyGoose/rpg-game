use std::time::Duration;
use bevy::prelude::Component;

#[derive(Default, Component, Debug)]
pub struct NextUpdate {
    pub time: Duration,
}
