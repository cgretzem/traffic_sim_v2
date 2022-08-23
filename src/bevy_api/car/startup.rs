use bevy::prelude::*;

use crate::{traffic_logic::road::Direction, simulator::Simulator, bevy_api::{GameTextures, road::RoadComponent, ROAD_SPRITE_SIZE, CAR_SPRITE_SIZE, CAR_SPRITE_SCALE, components::{Moveable, Scaleable}}};

use super::{components::{CarRotation, CarComponent, CurrentComponent, MovementComponent}, AppState};

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


pub fn car_movement_init_system(
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