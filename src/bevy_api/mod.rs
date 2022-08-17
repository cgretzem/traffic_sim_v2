mod car;
mod road;
mod components;

use bevy::prelude::*;
use crate::{simulator::Simulator, traffic_logic::road::{Road, Direction}};

use road::RoadPlugin;

use self::components::Moveable;


// region:    --- Game Textures
const ROAD_SPRITE:&str = "road.png";
const ROAD_SPRITE_SIZE:(f32, f32) = (154., 258.);
const SCROLL_SPEED:f32 = 10.;


// endregion: --- Game Textures`


pub struct GameTextures{
    road: Handle<Image>
}

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
    .add_system(drag_background_system)
    .add_plugin(RoadPlugin)
    .run();
}



fn drag_background_system(
    kb: Res<Input<KeyCode>>,
    mut query : Query<&mut Transform, With<Moveable>>
){
    let vel = if kb.pressed(KeyCode::Up){
        (0., SCROLL_SPEED)
    }
    else if kb.pressed(KeyCode::Down){
        (0., -SCROLL_SPEED)
    }
    else if kb.pressed(KeyCode::Left){
        (-SCROLL_SPEED, 0.)
    }
    else if kb.pressed(KeyCode::Right){
        (SCROLL_SPEED, 0.)
    }
    else{
        (0.,0.)
    };

    if vel == (0., 0.){
        return
    }
    for mut transform in query.iter_mut(){
        transform.translation.x += vel.0;
        transform.translation.y += vel.1;
    }
}


fn simulator_startup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>
){
    let gt = GameTextures{
        road: asset_server.load(ROAD_SPRITE),
    };
    commands.insert_resource(gt);
    commands.spawn_bundle(Camera2dBundle::default());
    let mut road = Road::new();
    road.add_connection(0, 1, 5, Direction::North);
    road.add_connection(1, 2, 5, Direction::North);
    road.add_connection(1, 3, 4, Direction::East);
    road.add_connection(1, 4, 7, Direction::West);
    commands.insert_resource(Simulator::new(2, road))
}




