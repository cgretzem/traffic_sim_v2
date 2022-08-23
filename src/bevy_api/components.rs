use bevy::prelude::*;

use super::{road::road_startup_system, simulator_startup_system};



// region:    --- General Components
#[derive(Component)]
pub struct Moveable;

#[derive(Component)]
pub struct Scaleable;
// endregion: --- General Components

// region:    --- Plugins

pub struct SimulatorPlugin;

impl Plugin for SimulatorPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup,simulator_startup_system);
        app.add_startup_system(road_startup_system.after(simulator_startup_system));
    }
}

// endregion: --- Plugins
