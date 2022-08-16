use bevy::prelude::{Plugin, Commands};

pub struct CarPlugin;

impl Plugin for CarPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        
    }
}


fn car_startup_system(
    mut commands: Commands
){

}