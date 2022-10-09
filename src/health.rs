use std::time::Duration;

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Bundle)]
pub struct HealthBundle {
    pub health: Health,
    pub delay: HealthTimer
}

impl HealthBundle {
    pub fn new(health_value: i8, hit_delay: Duration) -> HealthBundle {
        HealthBundle {
            health: Health::new(health_value),
            delay: HealthTimer::new(hit_delay)
        }
    }
}

#[derive(Component, Inspectable)]
pub struct Health(i8);

#[derive(Component)]
pub struct HealthTimer(Timer);

impl Health {
    pub fn new(value: i8) -> Health {
        Health(value)
    }

    pub fn decrease(&mut self) {
        self.0 -= 1;
    }
}

impl HealthTimer {
    pub fn new(delay: Duration) -> HealthTimer {
        let mut new_timer = Timer::new(delay, false);
        // We need to make the player being able to be hit, from the start
        new_timer.set_elapsed(delay);

        HealthTimer(new_timer)
    }

    pub fn update(&mut self, delta: Duration) {
        self.0.tick(delta);
    }

    pub fn finished(&self) -> bool {
        self.0.finished()
    }
    pub fn reset(&mut self) {
        self.0.reset();
    }
}

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_timers);
    }
}

fn update_timers(
    time: Res<Time>,
    mut health_timers: Query<&mut HealthTimer>,
) {
    for mut timer in &mut health_timers {
        timer.update(time.delta());
    }
}