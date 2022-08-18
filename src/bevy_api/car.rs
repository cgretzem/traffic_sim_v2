use bevy::prelude::*;

use crate::{simulator::Simulator, traffic_logic::road::Direction};

use super::{road::RoadComponent, ROAD_SPRITE_SIZE};

pub struct CarPlugin;

impl Plugin for CarPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, car_startup_system);
    }
}


// region:    --- Helper Structs
struct CarRotation{
    pub north : Quat,
    pub east : Quat,
    pub south : Quat,
    pub west : Quat
}

const car_facing: CarRotation = CarRotation{
    north : Quat::from_rotation_z(180.),
    south: Quat::from_rotation_z(0.),
    east : Quat::from_rotation_z(270.),
    west: Quat::from_rotation_z(90.)
};
// endregion: --- Helper Structs


pub struct CarComponent(pub u32);


fn car_startup_system(
    mut commands: Commands,
    sim: Res<Simulator>,
    query : Query<(&mut Transform, &mut RoadComponent), With<RoadComponent>>
){

    for car in sim.get_cars().iter(){
        let (road_transform, road_comp)= query
            .iter_mut()
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
        let horizontal = road_transform.rotation == Quat::from_rotation_z(0.);
        let num_cars = road_comp.num_cars
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