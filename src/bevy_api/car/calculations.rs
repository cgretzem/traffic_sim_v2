use bevy::prelude::*;

use crate::{traffic_logic::road::Direction, simulator::Simulator, bevy_api::{road::{RoadComponent, self, IntersectionComponent}, ROAD_SPRITE_SIZE, CAR_SPRITE_SIZE, CAR_SPRITE_SCALE}};

use super::{components::{CarComponent, CarRotation, CurrentComponent, BetweenComponent, MovementComponent}, AppState, CalcState};

pub fn car_at_intersection_system(
    mut commands : Commands,
    query : Query<(Entity, &CarComponent), With<CurrentComponent>>,
    query_road: Query<(&Transform, &RoadComponent, ), Without<CurrentComponent>>,
    sim : Res<Simulator>,
    mut state : ResMut<State<AppState>>,
    mut calc_state : ResMut<State<CalcState>>
){
    println!("Test1");
    if *calc_state.current() != CalcState::Waiting{
        return
    }
    for (entity, car_comp) in query.iter(){
        let car = sim.get_cars()
            .iter()
            .find(|car| car.get_id() == car_comp.0)
            .unwrap();

        let dir = car.get_position()
            .current
            .as_ref()
            .unwrap()
            .direction;

        let (road_transform, _road_comp)= query_road
            .iter()
            .find(|(_transform, road_comp)|{
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

            commands.entity(entity)
            .insert(MovementComponent{
                end_x_coord : x,
                end_y_coord: y,
                end_rotation: rotation,
                next_move : false
            })
            .remove::<CurrentComponent>();
            
    }
    calc_state.set(CalcState::Next).unwrap_or_else(|_|());
}


pub fn car_in_between_system(
    sim : Res<Simulator>,
    query : Query<(Entity, &CarComponent, &Transform), With<BetweenComponent>>,
    query_road: Query<(&Transform, &RoadComponent, ), Without<CurrentComponent>>,
    int_query : Query<(&Transform, &IntersectionComponent), (Without<CarComponent>, Without<RoadComponent>)>,
    mut commands : Commands,
    mut state : ResMut<State<AppState>>,
    mut calc_state : ResMut<State<CalcState>>
){
    if *calc_state.current() != CalcState::Next{
        return
    }
    
    println!("Test1");
    for (entity, car_comp, transform) in query.iter(){
        let car = sim.get_cars()
            .iter()
            .find(|car| car.get_id() == car_comp.0)
            .unwrap();
        
        let between = car
            .get_position()
            .between
            .as_ref()
            .unwrap();

        let dir = car
        .get_position()
        .between
        .as_ref()
        .unwrap()
        .arrival_direction;

        let (road_transform, road_comp)= query_road
            .iter()
            .find(|(_transform, road_comp)|{
                let int_1 = between.intersection_1;
                let int_2 = between.intersection_2;
                (road_comp.intersection1 == int_1 && road_comp.intersection2 == int_2)
                || (road_comp.intersection2 == int_1 && road_comp.intersection1 == int_2)
            })
            .unwrap();

        let center = road_transform.translation;
        let size = Vec3::new(ROAD_SPRITE_SIZE.0 * road_transform.scale.x, ROAD_SPRITE_SIZE.1 * road_transform.scale.y, road_transform.scale.z);
        let car_offset = 0.;
        let car_facing = CarRotation::new();

        

        let (x,y, rotation)  = match dir{
            Direction::North => {

                if transform.translation.x != center.x{
                    let target = if road_comp.intersection2 == between.intersection_1{
                        between.intersection_1
                    }
                    else{
                        between.intersection_2
                    };
                    let (int_transform, int_comp) = int_query
                    .iter()
                    .find(|(transform, int_comp)|{
                        int_comp.0 == between.intersection_1
                    })
                    .unwrap();

                    (int_transform.translation.x, int_transform.translation.y, transform.rotation)
                }
                else{
                    let begin = {
                        let (bot_x, bot_y) = (center.x, center.y - (size.y/2.));
                        (bot_x, bot_y + car_offset + (CAR_SPRITE_SIZE.1* CAR_SPRITE_SCALE/2.), car_facing.south)
                    };
                    let end = {
                        let top_y = center.y + (size.y/2.);
                        (center.x, top_y - car_offset - (CAR_SPRITE_SIZE.1* CAR_SPRITE_SCALE/2.), car_facing.north)
                    };
                    let new_y = begin.1 + ((between.progress/between.distance_to_next_intersection) as f32 *  (begin.1 - end.1).abs());
                    (center.x, new_y, car_facing.north)
                }
                //if x is not the same as road, we need to turn
            },
            Direction::East => {

                if transform.translation.y != center.y{
                    let target = if road_comp.intersection2 == between.intersection_1{
                        between.intersection_1
                    }
                    else{
                        between.intersection_2
                    };
                    let (int_transform, int_comp) = int_query
                    .iter()
                    .find(|(transform, int_comp)|{
                        int_comp.0 == between.intersection_1
                    })
                    .unwrap();

                    (int_transform.translation.x, int_transform.translation.y, transform.rotation)
                }
                else{
                    let begin = {
                        let left_x = center.x - (size.y/2.);
                        (left_x - car_offset - (CAR_SPRITE_SIZE.1 * CAR_SPRITE_SCALE/2.) + (CAR_SPRITE_SIZE.1* CAR_SPRITE_SCALE), center.y, car_facing.west)
                    };
                    let end = {
                        let right_x = center.x + (size.y/2.);
                        (right_x + car_offset + (CAR_SPRITE_SIZE.1* CAR_SPRITE_SCALE/2.) - (CAR_SPRITE_SIZE.1* CAR_SPRITE_SCALE), center.y, car_facing.east)
                    };
                    let new_x = begin.0 + ((between.progress/between.distance_to_next_intersection) as f32 *  (begin.0 - end.0).abs());
                    (new_x , center.y , car_facing.east)//what the fuck
                }

            },
            Direction::South => {

                if transform.translation.x != center.x{
                    let target = if road_comp.intersection2 == between.intersection_1{
                        between.intersection_1
                    }
                    else{
                        between.intersection_2
                    };
                    let (int_transform, int_comp) = int_query
                    .iter()
                    .find(|(transform, int_comp)|{
                        int_comp.0 == between.intersection_1
                    })
                    .unwrap();
                    (int_transform.translation.x, int_transform.translation.y, transform.rotation)
                }
                else{
                    let begin = {
                        let top_y = center.y + (size.y/2.);
                        (center.x, top_y - car_offset - (CAR_SPRITE_SIZE.1* CAR_SPRITE_SCALE/2.), car_facing.north)
                    };
                    let end = {
                        let (bot_x, bot_y) = (center.x, center.y - (size.y/2.));
                        (bot_x, bot_y + car_offset + (CAR_SPRITE_SIZE.1* CAR_SPRITE_SCALE/2.), car_facing.south)
                        
                    };
                    let new_y = begin.1 + ((between.progress/between.distance_to_next_intersection) as f32 *  (begin.1 - end.1).abs());
                    (center.x, new_y, car_facing.south)
                }
            }, 
            _ => {
                if transform.translation.y != center.y{
                    let target = if road_comp.intersection2 == between.intersection_1{
                        between.intersection_1
                    }
                    else{
                        between.intersection_2
                    };
                    let (int_transform, int_comp) = int_query
                    .iter()
                    .find(|(transform, int_comp)|{
                        int_comp.0 == between.intersection_1
                    })
                    .unwrap();

                    (int_transform.translation.x, int_transform.translation.y, transform.rotation)
                }
                else{
                    let begin = {
                        let right_x = center.x + (size.y/2.);
                        (right_x + car_offset + (CAR_SPRITE_SIZE.1* CAR_SPRITE_SCALE/2.) - (CAR_SPRITE_SIZE.1* CAR_SPRITE_SCALE), center.y, car_facing.east)
                    };
                    let end = {
                        let left_x = center.x - (size.y/2.);
                        (left_x - car_offset - (CAR_SPRITE_SIZE.1 * CAR_SPRITE_SCALE/2.) + (CAR_SPRITE_SIZE.1* CAR_SPRITE_SCALE), center.y, car_facing.west)
                        
                    };
                    let new_x = begin.0 + ((between.progress/between.distance_to_next_intersection) as f32 *  (begin.0 - end.0).abs());
                    (new_x , center.y , car_facing.west)//what the fuck
                }
            }, 
        };

        let mut move_comp = MovementComponent{
            end_x_coord : x,
            end_y_coord : y,
            end_rotation : rotation,
            next_move: false
        };
        match dir{
            Direction::North | Direction::South => {
                if transform.translation.x != center.x{
                    move_comp.next_move = true;
                }

            },
            _ => {
                if transform.translation.y != center.y{
                    move_comp.next_move = true;
                }
            }
        }
        commands.entity(entity)
            .insert(move_comp);
    }
    calc_state.set(CalcState::Waiting).unwrap();
    state.set(AppState::MovingCars).unwrap();

}





pub fn car_positioning_system(
    mut commands : Commands,
    sim : Res<Simulator>,
    query: Query<(Entity, &CarComponent)>,
    mut state : ResMut<State<AppState>>
){
    for car in sim.get_cars().iter(){
        let position = car.get_position();
        let entity = query.iter().find(|(_, car_comp)|{
            car_comp.0 == car.get_id()
        })
        .unwrap()
        .0;
        if let Some(_) = position.current{
            commands.entity(entity)
                .remove::<BetweenComponent>()
                .insert(CurrentComponent);
        }
        else{
            commands.entity(entity)
                .remove::<CurrentComponent>()
                .insert(BetweenComponent);
                
                
        }
    }
    state.set(AppState::CalculatingCars).unwrap();
}
