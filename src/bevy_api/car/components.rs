use bevy::prelude::*;
#[derive(Component)]
pub struct MovementComponent{
    pub end_x_coord:f32,
    pub end_y_coord:f32,
    pub start_rotation: Quat,
    pub end_rotation: Quat,
}

#[derive(Component)]
pub struct CurrentComponent;

#[derive(Component)]
pub struct BetweenComponent;

#[derive(Component)]
pub struct CarComponent(pub u32);


pub struct CarRotation{
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
