mod car;
mod road;


use bevy::prelude::*;
use crate::{simulator::Simulator, traffic_logic::road::{Road, Direction}};

use road::RoadPlugin;

pub fn run(){
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .insert_resource(WindowDescriptor{
        title: "Traffic Simulator!".to_string(),
        width: 598.0,
        height: 676.0,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_startup_system(simulator_startup_system)
    .add_plugin(RoadPlugin)
    .run();
}


fn simulator_startup_system(mut commands: Commands){
    commands.spawn_bundle(Camera2dBundle::default());
    let mut road = Road::new();
    road.add_connection(0, 1, 5, Direction::North);
    road.add_connection(1, 2, 5, Direction::North);
    commands.insert_resource(Simulator::new(2, road))
}




