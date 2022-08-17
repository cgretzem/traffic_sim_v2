use std::collections::HashMap;

use bevy::{prelude::*};
use crate::{simulator::Simulator, traffic_logic::road::Direction, bevy_api::components::Moveable};

use super::{GameTextures, ROAD_SPRITE_SIZE};

// region:    --- Road Components
#[derive(Component)]
struct IntersectionComponent;

#[derive(Component)]
pub struct RoadComponent{
    pub intersection : u32,
    pub direction : Direction,
    pub num_cars : u32
}

// endregion: --- Road Components


pub struct RoadPlugin;

impl Plugin for RoadPlugin{
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, road_startup_system);
    }
}


// region:    --- Road Constants
const ROAD_UNIT_DISTANCE: f32 = 50.;
const INTERSECTION_SIZE: f32 = 50.;
// endregion: --- Road Constants



fn road_startup_system(
    mut commands: Commands,
    sim : Res<Simulator>,
    gt : Res<GameTextures>
){
    //calculate first intersection
    let mut fringe:Vec<u32> = Vec::new();
    let root = sim.get_random_intersection();

    //spawn inital intersection at 0,0, center of screen
    commands.spawn_bundle(SpriteBundle{
        sprite: Sprite {
            color: Color::rgba(1., 1., 1., 1.),
            custom_size: Some(Vec2::new(INTERSECTION_SIZE, INTERSECTION_SIZE)),
            ..Default::default()
        },
        transform: Transform{
            translation: Vec3::new(0.,0., 5.),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Moveable)
    .insert(IntersectionComponent);


    fringe.push(root); // adding inital intersection to fringe
    //breadth first search on all intersections
    let mut int_map:HashMap<u32, (f32, f32)> = HashMap::new();
    let mut explored:Vec<u32> = Vec::new();
    let mut used_roads:Vec<(u32, u32)> = Vec::new();
    let mut new_fringe:Vec<u32> = Vec::new();
    //adding all new intersections to fringe
    while !fringe.is_empty(){
        fringe.retain(|id|{
            let int_coords = int_map.entry(*id).or_insert((0.,0.)).clone();
            let connections = sim.road.get_all_connections(id);
            for conn in &connections{

                
                let (x_mul, y_mul) = match conn.direction{
                    Direction::North => (0f32, 1f32),
                    Direction::East => (1f32, 0f32),
                    Direction::South => (0f32, -1f32),
                    _ => (-1., 0.)
                };

                //compute scale
                let (x_scale, y_scale) = (INTERSECTION_SIZE/ROAD_SPRITE_SIZE.0, ((ROAD_UNIT_DISTANCE* conn.distance as f32) - INTERSECTION_SIZE/2.) /ROAD_SPRITE_SIZE.1);

                // let (x_scale, y_scale) = if x_mul != 0.{
                //     (((ROAD_UNIT_DISTANCE* conn.distance as f32) - INTERSECTION_SIZE/2.)/ROAD_SPRITE_SIZE.0 , INTERSECTION_SIZE/ROAD_SPRITE_SIZE.1)
                // }
                // else{
                //     (INTERSECTION_SIZE/ROAD_SPRITE_SIZE.0, ((ROAD_UNIT_DISTANCE* conn.distance as f32) - INTERSECTION_SIZE/2.) /ROAD_SPRITE_SIZE.1)
                // };
                
                //creating intersections
                let x_offset = (conn.distance as f32 * ROAD_UNIT_DISTANCE + INTERSECTION_SIZE/2.)* x_mul;
                let y_offset = (conn.distance as f32 * ROAD_UNIT_DISTANCE + INTERSECTION_SIZE/2.)* y_mul;
                let (x,y) = (
                    x_offset + int_coords.0,
                    y_offset + int_coords.1
                );
                
                commands.spawn_bundle(SpriteBundle{
                    sprite: Sprite {
                        color: Color::rgba(1., 1., 1., 1.),
                        custom_size: Some(Vec2::new(INTERSECTION_SIZE, INTERSECTION_SIZE)),
                        ..Default::default()
                    },
                    transform: Transform{
                        translation: Vec3::new(x,y, 5.),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Moveable)
                .insert(IntersectionComponent);

                int_map.insert(conn.next_intersection, (x,y));

                if let None = used_roads
                .iter()
                .find(|tup|{
                    **tup == (*id, conn.next_intersection) || **tup == (conn.next_intersection, *id)
                }){
                    let rotation:f32 = if x_mul != 0.{
                        1.5708
                    }
                    else{
                        0.
                    };
                    commands.spawn_bundle(SpriteBundle{
                        texture : gt.road.clone(),
                        transform: Transform{
                            scale: Vec3::new(x_scale, y_scale, 1.),
                            translation : Vec3::new( 
                                (x + int_coords.0)/2.,
                                (y + int_coords.1)/2., 5.),
                            rotation:Quat::from_rotation_z(rotation),
                            ..Default::default()
                        },
                        ..Default::default()
                        
                    })
                    .insert(RoadComponent{
                        intersection : *id,
                        direction : conn.direction,
                        num_cars : 0
                    })
                    .insert(RoadComponent{
                        intersection : conn.next_intersection,
                        direction : conn.direction.get_straight_dir(),
                        num_cars: 0
                    })
                    .insert(Moveable);
                    used_roads.push((*id, conn.next_intersection));
                }
                

                new_fringe.push(conn.next_intersection);
                
            }
            explored.push(*id);
            false
        });
        for new_id in &new_fringe{
            if !explored.contains(new_id){
                fringe.push(*new_id)
            }
            
        }
        new_fringe.clear();
    }

}