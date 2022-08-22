use bevy::{prelude::*, ecs::query};

use crate::{simulator::Simulator, traffic_logic::{road::Direction, car}, bevy_api::{components::{Moveable, Scaleable}, CAR_SPRITE_SCALE, FONT}};

use super::{road::{RoadComponent, road_startup_system}, ROAD_SPRITE_SIZE, CAR_SPRITE_SIZE, GameTextures, simulator_startup_system, AppState, CAR_SPEED};

pub struct CarPlugin;

impl Plugin for CarPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(
            SystemSet::on_update(AppState::CalculatingCars)
            .with_system(car_removal_system)
            .with_system(car_movement_init_system)
        );
        app.add_system_set(
            SystemSet::on_update(AppState::MovingCars)
            .with_system(car_movement_system)
            .with_system(movement_waiting_system)
        );
        app.add_system_set(
            SystemSet::on_update(AppState::Waiting)
            .with_system(temp_tick_system)
        );
    }
}


// region:    --- Helper Structs
struct CarRotation{
    pub north : Quat,
    pub east : Quat,
    pub south : Quat,
    pub west : Quat
}

impl CarRotation{
    pub fn new() -> Self{
        CarRotation{
            north : Quat::from_rotation_z(3.14159),
            south: Quat::from_rotation_z(0.),
            west : Quat::from_rotation_z(4.71239),
            east: Quat::from_rotation_z(1.5708)
        }
    }
}


// endregion: --- Helper Structs

#[derive(Component)]
pub struct MovementComponent{
    end_x_coord:f32,
    end_y_coord:f32,
    start_rotation: Quat,
    end_rotation: Quat,
}

#[derive(Component)]
pub struct CurrentComponent;

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
    

    for car in sim.get_cars().iter(){
        let dir = {
            car
            .get_position()
            .current
            .as_ref()
            .unwrap()
            .direction            
        };
        let (road_transform, mut road_comp)= query
            .iter_mut()
            .find(|(_transform, road_comp)|{
                println!("{:?}", road_comp);
                let int_id = car
                    .get_position()
                    .current
                    .as_ref()
                    .unwrap()
                    .id;
                    
                road_comp.intersection1 == int_id && (dir == road_comp.direction1 || dir.get_straight_dir() == road_comp.direction1)
                    || road_comp.intersection2 == int_id && (dir == road_comp.direction2 || dir.get_straight_dir() == road_comp.direction2)
            })
            .unwrap();
        let center = road_transform.translation;
        let size = Vec3::new(ROAD_SPRITE_SIZE.0 * road_transform.scale.x, ROAD_SPRITE_SIZE.1 * road_transform.scale.y, road_transform.scale.z);
        let car_offset = 0.;
        
        let car_facing = CarRotation::new();
        let (x,y, rotation) = match dir.get_straight_dir(){
                Direction::North => {
                    //car is heading north
                    let top_y = center.y + (size.y/2.);
                    (center.x, top_y - car_offset - (CAR_SPRITE_SIZE.1* CAR_SPRITE_SCALE/2.), car_facing.north)
                    
                },
                Direction::East =>{
                    //car is at east end of road
                    let right_x = center.x + (size.y/2.);
                    (right_x + car_offset + (CAR_SPRITE_SIZE.1* CAR_SPRITE_SCALE/2.) - (CAR_SPRITE_SIZE.1* CAR_SPRITE_SCALE), center.y, car_facing.east)
                },
                Direction::South => {
                    let (bot_x, bot_y) = (center.x, center.y - (size.y/2.));
                    (bot_x, bot_y + car_offset + (CAR_SPRITE_SIZE.1* CAR_SPRITE_SCALE/2.), car_facing.south)
                }
                _ => {
                    let left_x = center.x - (size.y/2.);
                    (left_x - car_offset - (CAR_SPRITE_SIZE.1 * CAR_SPRITE_SCALE/2.) + (CAR_SPRITE_SIZE.1* CAR_SPRITE_SCALE), center.y, car_facing.west)
                    
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
            .insert(CarComponent(car.get_id()))
            .insert(CurrentComponent);
            road_comp.num_cars += 1;

           

    }

}


fn car_movement_init_system(
    mut commands : Commands,
    mut query : Query<(Entity, &Transform, &CarComponent), With<CarComponent>>,
    mut query_road : Query<(&Transform, &mut RoadComponent), Without<CarComponent>>,
    sim : Res<Simulator>,
    mut state : ResMut<State<AppState>>
){
    if *state.current() != AppState::CalculatingCars{
        return
    }

    for (entity, transform, car_comp) in query.iter_mut(){
        let id = car_comp.0;
        let car_position = match sim
        .get_cars()
        .iter()
        .find(|car| car.get_id() == id){
            None => continue,
            Some(car) => car.get_position()
        };
        //finding road that car is on
        let (road_transform, mut road_comp)= match query_road
        .iter_mut()
        .find(|(_transform, road_comp)|{
            let int_id = id;
            road_comp.intersection1 == int_id || road_comp.intersection2 == int_id
        })
        {
            None => continue,
            Some(road) => road
        };
        let center = road_transform.translation;
        let size = Vec3::new(ROAD_SPRITE_SIZE.0 * road_transform.scale.x, ROAD_SPRITE_SIZE.1 * road_transform.scale.y, road_transform.scale.z);
        let car_offset = 0.;
        let direction = if let Some(curr) = &car_position.current{
            if road_comp.intersection1 == curr.id{
                road_comp.direction1
            }
            else{
                road_comp.direction2
            }
        }
        else{
            let between = car_position.between.as_ref().unwrap();
            between.arrival_direction
        };
        let car_facing = CarRotation::new();
        let (x,y, rotation) = match direction{
            Direction::North => {
                //south end of road
                if let Some(_curr) = &car_position.current{
                    let (bot_x, bot_y) = (center.x, center.y - (size.y/2.));
                    (bot_x, bot_y + car_offset + (CAR_SPRITE_SIZE.1* CAR_SPRITE_SCALE/2.), car_facing.north)
                }
                else{
                    let between = &car_position.between.as_ref().unwrap();
                    
                    
                    let begin = {
                        let (bot_x, bot_y) = (center.x, center.y - (size.y/2.));
                        (bot_x, bot_y + car_offset + (CAR_SPRITE_SIZE.1* CAR_SPRITE_SCALE/2.), car_facing.north)
                        
                    };
                    let end = {
                        let top_y = center.y + (size.y/2.);
                        (center.x, top_y - car_offset - (CAR_SPRITE_SIZE.1* CAR_SPRITE_SCALE/2.), car_facing.north)
                    };
                    let new_y = begin.1 + ((between.progress/between.distance_to_next_intersection) as f32 *  (begin.1 - end.1).abs());
                    (center.x, new_y - car_offset - (CAR_SPRITE_SIZE.1* CAR_SPRITE_SCALE/2.), car_facing.north)
                }
                
            },
            Direction::East =>{
                //car is at west end of road
                if let Some(_curr) = &car_position.current{
                    let left_x = center.x - (size.y/2.);
                    (left_x - car_offset - (CAR_SPRITE_SIZE.1 * CAR_SPRITE_SCALE/2.) + (CAR_SPRITE_SIZE.1* CAR_SPRITE_SCALE), center.y, car_facing.east)
                }
                else{
                    let between = &car_position.between.as_ref().unwrap();
                    

                    let begin = {
                        let left_x = center.x - (size.y/2.);
                        (left_x - car_offset - (CAR_SPRITE_SIZE.1 * CAR_SPRITE_SCALE/2.) + (CAR_SPRITE_SIZE.1* CAR_SPRITE_SCALE), center.y, car_facing.east)
                        
                    };
                    let end = {
                        let right_x = center.x + (size.y/2.);
                        (right_x + car_offset + (CAR_SPRITE_SIZE.1* CAR_SPRITE_SCALE/2.) - (CAR_SPRITE_SIZE.1* CAR_SPRITE_SCALE), center.y, car_facing.east)
                    };
                    let new_x = begin.0 + ((between.progress/between.distance_to_next_intersection) as f32 *  (begin.0 - end.0).abs());
                    (new_x, center.y - car_offset - (CAR_SPRITE_SIZE.1* CAR_SPRITE_SCALE/2.), car_facing.east)
                }
               
            },
            Direction::South => {
                if let Some(_curr) = &car_position.current{
                    let top_y = center.y + (size.y/2.);
                    (center.x, top_y - car_offset - (CAR_SPRITE_SIZE.1* CAR_SPRITE_SCALE/2.), car_facing.south)
                }
                else{
                    let between = &car_position.between.as_ref().unwrap();

                    let begin = {
                        let top_y = center.y + (size.y/2.);
                        (center.x, top_y - car_offset - (CAR_SPRITE_SIZE.1* CAR_SPRITE_SCALE/2.), car_facing.south)
                    };
                    let end = {
                        let (bot_x, bot_y) = (center.x, center.y - (size.y/2.));
                        (bot_x, bot_y + car_offset + (CAR_SPRITE_SIZE.1* CAR_SPRITE_SCALE/2.), car_facing.south)
                        
                    };
                    let new_y = begin.1 + ((between.progress/between.distance_to_next_intersection) as f32 *  (begin.1 - end.1).abs());
                    (center.x, new_y - car_offset - (CAR_SPRITE_SIZE.1* CAR_SPRITE_SCALE/2.), car_facing.south)
                }
                
               
            }
            _ => {
                if let Some(_curr) = &car_position.current{
                    let left_x = center.x + (size.y/2.);
                    (left_x + car_offset + (CAR_SPRITE_SIZE.1* CAR_SPRITE_SCALE/2.) - (CAR_SPRITE_SIZE.1* CAR_SPRITE_SCALE), center.y, car_facing.west)
                }
                else{
                    let between = &car_position.between.as_ref().unwrap();
                    

                    let begin = {
                        let right_x = center.x + (size.y/2.);
                        (right_x + car_offset + (CAR_SPRITE_SIZE.1* CAR_SPRITE_SCALE/2.) - (CAR_SPRITE_SIZE.1* CAR_SPRITE_SCALE), center.y, car_facing.west)
                    };
                    let end = {
                        let left_x = center.x - (size.y/2.);
                        (left_x - car_offset - (CAR_SPRITE_SIZE.1 * CAR_SPRITE_SCALE/2.) + (CAR_SPRITE_SIZE.1* CAR_SPRITE_SCALE), center.y, car_facing.west)
                        
                    };
                    let new_x = begin.0 + ((between.progress/between.distance_to_next_intersection) as f32 *  (begin.0 - end.0).abs());
                    (new_x, center.y - car_offset - (CAR_SPRITE_SIZE.1* CAR_SPRITE_SCALE/2.), car_facing.west)
                }
                
            }
            
        };

        commands
        .entity(entity)
        .insert(MovementComponent{end_x_coord : x,
            end_y_coord: y,
            start_rotation : rotation,
            end_rotation : rotation
        });
        
        
    }
    state.set(AppState::MovingCars).unwrap_or_else(|_|println!("{:?}", state));
}



fn car_at_intersection_system(
    mut commands : Commands,
    query : Query<(Entity, &Transform, &CarComponent), With<CarComponent>>,
    sim : Res<Simulator>
){

}

//  let new_y = begin.1 + ((between.progress/between.distance_to_next_intersection) as f32 *  (begin.1 - end.1).abs());
//     (center.x, new_y - car_offset - (CAR_SPRITE_SIZE.1* CAR_SPRITE_SCALE/2.), car_facing.north)

fn temp_tick_system(
    mut sim : ResMut<Simulator>,
    mut kb: ResMut<Input<KeyCode>>,
    mut state : ResMut<State<AppState>>
    
){
   
    if kb.just_pressed(KeyCode::T){
        if *state.current() != AppState::Waiting{
            println!("Cannot tick while cars are moving");
            return;
        }
        println!("ticking");
        sim.tick();
        state.set(AppState::CalculatingCars).unwrap();
        kb.reset(KeyCode::T);
    }
    
}

fn car_movement_system(
    mut query: Query<(Entity, &mut Transform, &MovementComponent)>,
    mut commands : Commands,

){

    for (entity, mut transform, move_comp) in query.iter_mut(){
        let (x, y) = (transform.translation.x, transform.translation.y);
        let (goal_x, goal_y) = (move_comp.end_x_coord, move_comp.end_y_coord);
        let x_mul = {
            if goal_x - x == 0.{
                0f32
            }
            else if goal_x-x > 0.{
                1f32
            }
            else{
                -1f32
            }
        };
        let y_mul = {
            if goal_y - y == 0.{
                0f32
            }
            else if goal_y-y > 0.{
                1f32
            }
            else{
                -1f32
            }
        };
        let final_x = if (goal_x - (x + (x_mul * CAR_SPEED)) > 0. && x_mul != 1.)
            || (goal_x - (x + (x_mul * CAR_SPEED)) < 0. && x_mul != -1.)
        {
            goal_x
        }
        else{
            x + (x_mul * CAR_SPEED)
        };
        let final_y = if (goal_y - (y + (y_mul * CAR_SPEED)) > 0. && y_mul != 1.)
            || (goal_y - (y + (y_mul * CAR_SPEED)) < 0. && y_mul != -1.)
        {
            goal_y
        }
        else{
            y + (y_mul * CAR_SPEED)
        };

        
        transform.translation = Vec3::new(final_x, final_y, transform.translation.z);
        if final_x == goal_x && final_y == goal_y{
            commands.entity(entity)
                .remove::<MovementComponent>();
        }
    }
    
    // state.set(AppState::Waiting).unwrap();
}

fn movement_waiting_system(
    mut state : ResMut<State<AppState>>,
    mut query: Query<With<MovementComponent>>
){
    if query.is_empty(){
        state.set(AppState::Waiting);
    }
}

fn car_removal_system(
    mut commands: Commands,
    query : Query<(Entity, &CarComponent)>,
    sim: Res<Simulator>
){
    for (entity, car_comp) in query.iter(){
        let id = car_comp.0;
        if let None = sim
            .get_cars()
            .iter()
            .find(|car| car.get_id() == id)
        {
            commands.entity(entity).despawn();
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