mod car;
mod road;
mod components;

use bevy::{prelude::*, input::mouse::{MouseWheel, MouseScrollUnit}};
use crate::{simulator::Simulator, traffic_logic::road::{Road, Direction}};



use self::{components::{Moveable, SimulatorPlugin, Scaleable}, car::CarComponent};


// region:    --- Game Textures
const ROAD_SPRITE:&str = "road.png";
const ROAD_SPRITE_SIZE:(f32, f32) = (154., 258.);
pub const ROAD_UNIT_DISTANCE: f32 = 50.;
const INTERSECTION_SIZE: f32 = 50.;

const CAR_SPRITE:&str = "car_small.png";
const CAR_SPRITE_SIZE:(f32, f32) = (376., 695.);
const CAR_SPRITE_SCALE:f32 = ROAD_UNIT_DISTANCE/CAR_SPRITE_SIZE.1;
// endregion: --- Game Textures`


// region:    --- Game Constants
const SCROLL_SPEED:f32 = 10.;

const SCALE_FACTOR:f32 = 0.05;
// endregion: --- Game Constants


pub struct GameTextures{
    road: Handle<Image>,
    car: Handle<Image>
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
    .add_system(drag_background_system)
    .add_plugin(ScalePlugin)
    .add_plugin(SimulatorPlugin)
    .run();
}



pub struct ScalePlugin;

impl Plugin for ScalePlugin{
    fn build(&self, app: &mut App) {
        app.add_system(scale_cars);
    }
}

fn scale_cars(
    mut scroll_evr: EventReader<MouseWheel>,
    mut query : Query<&mut Transform, (With<Scaleable>, With<Scaleable>)>
){
    for ev in scroll_evr.iter(){
        match ev.unit{
            MouseScrollUnit::Line => {
                for mut transform in query.iter_mut(){
                    transform.scale = Vec3::new(transform.scale.x + 0.05*ev.y, transform.scale.y + 0.05*ev.y, 1.);
                    //transform.translation = Vec3::new(transform.translation.x * 0.05*ev.y, transform.translation.y * 0.05*ev.y, 1.);
                }
            },
            _=> {
                for mut transform in query.iter_mut(){
                    transform.scale = Vec3::new(transform.scale.x + 0.05*ev.y, transform.scale.y + 0.05*ev.y, 1.);
                    //transform.translation = Vec3::new(transform.translation.x + 0.05*ev.y, transform.translation.y + 0.05*ev.y, 1.);
                }
            }
        };
    }
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


pub fn simulator_startup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    let gt = GameTextures{
        road: asset_server.load(ROAD_SPRITE),
        car: asset_server.load(CAR_SPRITE)
    };
    commands.insert_resource(gt);
    commands.spawn_bundle(Camera2dBundle::default());
    let mut road = Road::new();
    road.add_connection(0, 1, 5, Direction::North);
    road.add_connection(1, 2, 5, Direction::North);
    road.add_connection(1, 3, 4, Direction::East);
    road.add_connection(1, 4, 7, Direction::West);
    commands.insert_resource(Simulator::new(10, road));
    //sim_state.overwrite_set(SimState::Loaded).unwrap();
    //sim_state.set(SimState::Loaded).unwrap();

}




