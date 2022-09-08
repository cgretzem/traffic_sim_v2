
mod calculations;
mod movement;
mod startup;
mod components;
use bevy::prelude::*;

use crate::simulator::Simulator;

use self::{components::CarComponent, startup::{car_startup_system}, movement::{car_movement_system, movement_waiting_system}, calculations::{car_positioning_system, car_at_intersection_system, car_in_between_system}};


pub struct CarPlugin;

impl Plugin for CarPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, car_startup_system);
        app.add_state(AppState::Waiting);
        app.add_state(CalcState::Waiting);
        app.add_system_set(
            SystemSet::on_update(AppState::RemovingCars)
            .with_system(car_removal_system)
        );
        app.add_system_set(
            SystemSet::on_update(AppState::InitializingComponents)
            .with_system(car_positioning_system)
        );
        app.add_system_set(
            SystemSet::on_update(AppState::CalculatingCars)
            .with_system(car_at_intersection_system)
            .with_system(car_in_between_system)
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
        state.set(AppState::RemovingCars).unwrap();
        kb.reset(KeyCode::T);
    }
    
}


fn car_removal_system(
    mut commands: Commands,
    query : Query<(Entity, &CarComponent)>,
    sim: Res<Simulator>,
    mut state : ResMut<State<AppState>>
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
    state.set(AppState::InitializingComponents).unwrap();
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum AppState{
    InitializingComponents,
    RemovingCars,
    CalculatingCars,
    MovingCars,
    Waiting
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum CalcState{
    Waiting,
    Next,
}
