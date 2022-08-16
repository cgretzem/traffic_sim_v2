use bevy::prelude::*;

use crate::simulator::Simulator;

pub struct CarPlugin;

impl Plugin for CarPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, car_startup_system);
    }
}


fn car_startup_system(
    mut commands: Commands,
    sim: Res<Simulator>
){
    
}