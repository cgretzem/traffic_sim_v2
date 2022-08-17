use bevy::prelude::*;

use crate::{simulator::Simulator, traffic_logic::road::Direction};

use super::{road::RoadComponent, ROAD_SPRITE_SIZE};

pub struct CarPlugin;

impl Plugin for CarPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, car_startup_system);
    }
}


pub struct CarComponent(pub u32);


fn car_startup_system(
    mut commands: Commands,
    sim: Res<Simulator>,
    query : Query<(&Transform, &RoadComponent), With<RoadComponent>>
){

    for car in sim.get_cars().iter(){
        let (road_transform, road_comp)= query
            .iter()
            .find(|(transform, road_comp)|{
                road_comp.intersection == car
                    .get_position()
                    .current
                    .as_ref()
                    .unwrap()
                    .id
            })
            .unwrap();
        let center = road_transform.translation;
        let size = road_transform.scale.dot(Vec3::new(ROAD_SPRITE_SIZE.0, ROAD_SPRITE_SIZE.1, 1.));
        let horizontal = road_transform.rotation == Quat::default();
        let num_cars = 
        let (x,y) = match car.get_position()
            .current
            .as_ref()
            .unwrap()
            .direction{
                Direction::North => {
                    
                },
                Direction::East =>{},
                Direction::South => {}
                _ => {}
            };
        

    }

}