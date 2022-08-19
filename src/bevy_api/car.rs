use bevy::{prelude::*, ecs::query};

use crate::{simulator::Simulator, traffic_logic::road::Direction, bevy_api::{components::{Moveable, Scaleable}, CAR_SPRITE_SCALE, FONT}};

use super::{road::{RoadComponent, road_startup_system}, ROAD_SPRITE_SIZE, CAR_SPRITE_SIZE, GameTextures, simulator_startup_system, AppState};

pub struct CarPlugin;

impl Plugin for CarPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, car_startup_system.after(road_startup_system).after(simulator_startup_system));
    }
}


// region:    --- Helper Structs
struct CarRotation{
    pub north : Quat,
    pub east : Quat,
    pub south : Quat,
    pub west : Quat
}


// endregion: --- Helper Structs


#[derive(Component)]
pub struct CarComponent(pub u32);


pub fn car_startup_system(
    
    mut commands: Commands,
    sim: Res<Simulator>,
    mut query : Query<(&mut Transform, &mut RoadComponent), With<RoadComponent>>,
    textures: Res<GameTextures>,
    //sim_state: Res<State<SimState>>
){

    // if let SimState::Loading = sim_state.current(){
    //     println!("{:?}", sim_state.current());
    //     return
    // }
    let car_facing = CarRotation{
        south : Quat::from_rotation_z(3.14159),
        north: Quat::from_rotation_z(0.),
        east : Quat::from_rotation_z(4.71239),
        west: Quat::from_rotation_z(1.5708)
    };

    for car in sim.get_cars().iter(){
        let (road_transform, mut road_comp)= query
            .iter_mut()
            .find(|(_transform, road_comp)|{
                let int_id = car
                    .get_position()
                    .current
                    .as_ref()
                    .unwrap()
                    .id;
                road_comp.intersection1 == int_id || road_comp.intersection2 == int_id
            })
            .unwrap();
        let center = road_transform.translation;
        let size = Vec3::new(ROAD_SPRITE_SIZE.0 * road_transform.scale.x, ROAD_SPRITE_SIZE.1 * road_transform.scale.y, road_transform.scale.z);
        let num_cars = road_comp.num_cars;
        let car_offset = 0.;
        let dir = {
            let curr_intersection = car
            .get_position()
            .current
            .as_ref()
            .unwrap();
            if road_comp.intersection1 == curr_intersection.id{
                road_comp.direction1
            }
            else{
                road_comp.direction2
            }
        };
        let (x,y, rotation) = match dir{
                Direction::North => {
                    //south end of road
                    let (bot_x, bot_y) = (center.x, center.y - (size.y/2.));
                    (bot_x, bot_y + car_offset + (CAR_SPRITE_SIZE.1* CAR_SPRITE_SCALE/2.), car_facing.north)
                },
                Direction::East =>{
                    //car is at west end of road
                    let left_x = center.x - (size.y/2.);
                    (left_x - car_offset - (CAR_SPRITE_SIZE.1 * CAR_SPRITE_SCALE/2.) + (CAR_SPRITE_SIZE.1* CAR_SPRITE_SCALE), center.y, car_facing.east)
                },
                Direction::South => {
                    let top_y = center.y + (size.y/2.);
                    (center.x, top_y - car_offset - (CAR_SPRITE_SIZE.1* CAR_SPRITE_SCALE/2.), car_facing.south)
                }
                _ => {
                    let right_x = center.x + (size.y/2.);
                    
                    (right_x + car_offset + (CAR_SPRITE_SIZE.1* CAR_SPRITE_SCALE/2.) - (CAR_SPRITE_SIZE.1* CAR_SPRITE_SCALE), center.y, car_facing.west)
                }
                
            };
           

            commands.spawn_bundle(SpriteBundle{
                texture: textures.car.clone(),
                transform:Transform{
                    scale: Vec3::new(CAR_SPRITE_SCALE, CAR_SPRITE_SCALE, 1.),
                    translation: Vec3::new(x,y,15.),
                    rotation: rotation,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Moveable)
            .insert(Scaleable)
            .insert(CarComponent(car.get_id()));
            road_comp.num_cars += 1;

           

    }

}


fn car_movement_system(
    mut commands : Commands,
    mut query : Query<(&mut Transform, &CarComponent)>,
    mut query_road : Query<(&mut Transform, &mut RoadComponent)>,
    sim : Res<Simulator>,
    mut state : ResMut<State<AppState>>
){
    if *state.current() != AppState::MovingCars{
        return
    }

    for (mut transform, car_comp) in query.iter_mut(){
        let id = car_comp.0;
        let car_position = match sim
        .get_cars()
        .iter()
        .find(|car| car.get_id() == id){
            None => continue,
            Some(car) => car.get_position()
        };

        if let Some(current) = &car_position.current{

        }
        

    }

}

// fn car_numbering_system(
//     mut commands: Commands,
//     query: Query<&Transform, With<CarComponent>>

// ){
                                    
//     let rect = Vec2::new(15., 15.);
//     commands.spawn_bundle(SpriteBundle{
//         sprite: Sprite{
//             color: Color::rgb(1.,0.,0.),
//             custom_size : Some(Vec2::new(rect.x, rect.y)),
//             ..Default::default()
//         },
//         transform:Transform{
//             translation: Vec3::new(x,y,16.),
//             ..Default::default()
//         },
//         ..Default::default()
//     })
//     .insert(Scaleable)
//     .insert(Moveable);

//     commands.spawn_bundle(Text2dBundle{
//         text : Text::from_section(num_cars.to_string(), TextStyle {
//             font : textures.font.clone(),
//             font_size: 20.0,
//             color: Color::WHITE,
//             ..Default::default()
//         }).with_alignment(TextAlignment::CENTER),
//         transform:Transform{
//             translation: Vec3::new(x-1.,y+1.,17.),
//             ..Default::default()
//         },
//         ..Default::default()
//     })
//     .insert(Scaleable)
//     .insert(Moveable);
// }