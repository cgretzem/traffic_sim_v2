use bevy::prelude::*;

use crate::bevy_api::{CAR_SPEED};

use super::{components::MovementComponent, AppState};

pub fn car_movement_system(
    mut query: Query<(Entity, &mut Transform, &mut MovementComponent)>,
    mut commands : Commands,

){

    for (entity, mut transform, mut move_comp) in query.iter_mut(){
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
        //println!("Final X : {final_x}\nFinal Y : {final_y}\nGoal X : {goal_x}\nGoal Y : {goal_y}");
        if final_x == goal_x && final_y == goal_y{
            if move_comp.next_move == false{
                commands.entity(entity)
                .remove::<MovementComponent>();
            }
            else{
                println!("Removed Component");
                move_comp.next_move = false;
            }
            
        }
    }
    
    
    // state.set(AppState::Waiting).unwrap();
}


pub fn movement_waiting_system(
    mut state : ResMut<State<AppState>>,
    query: Query<With<MovementComponent>>
){
    if query.is_empty(){
        state.set(AppState::Waiting).unwrap();
    }

}


