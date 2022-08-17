
use std::{collections::{HashMap, HashSet}};


use rand::seq::SliceRandom;

use crate::traffic_logic::{car::{Car, Position}, intersection::{Intersection, LightColor, LightConfig}, road::{Road, Direction}};



pub struct Simulator
{
    cars : Vec<Car>,
    intersections: HashMap<u32, Intersection>,
    pub road: Road,
    pub verbose: bool
    
}

impl Simulator{
    pub fn new(num_cars: u32, road:Road) -> Self{
        let mut sim = Simulator { cars:Vec::new(), intersections: HashMap::new(), road, verbose:true};

        for int in sim.road.get_all_intersections(){
            sim.intersections.insert(int, Intersection::new(int, 3));
        }

        for i in 0..num_cars{
            let mut car = Car::new(i);
            let int_id = &sim.road.get_random_intersection();
            sim.set_random_start(&mut car, int_id);
            sim.cars.push(car);
        }
        
        sim
    }

    pub fn get_cars(&self) -> &Vec<Car>{
        &self.cars
    }

    fn set_random_start(&mut self, car: &mut Car, int_id: &u32){
        
        
        let intersection_dirs: Vec<Direction> = self.road
            .get_all_connections(&int_id)
            .iter()
            .map(|conn| conn.direction)
            .collect();

        let dir = intersection_dirs.choose(&mut rand::thread_rng())
            .unwrap_or_else(|| panic!("Intersection {int_id} has no connections!"));

        car.set_position(Position::new_current(*int_id, *dir))
    }

    pub fn tick(&mut self) {
        self.tick_lights_random();
        self.tick_cars();
    }

    fn tick_lights_random(&mut self){
        for (_,intersection) in &mut self.intersections{
            let config = Intersection::get_random_config();
            intersection.set_lights(config);
        }
    }

    pub fn tick_ai(&mut self, map: &HashMap<u32, LightConfig>){
        for (id, lights) in map{
            self.intersections.get_mut(id).unwrap_or_else(|| panic!("Cannot find intersection {}", id)).set_lights(lights.clone());
        }
    }

    fn tick_cars(&mut self){
        let mut cars_to_check: HashSet<u32> = HashSet::new();
        self.cars.iter_mut().for_each(|car|{
            let c_id = car.get_id();
            let pos = car.get_position_mut();
            if let Some(between) = &mut pos.between{
                //car is not at final stretch
                if between.progress != between.distance_to_next_intersection-1{
                    between.progress += 1;
                    if self.verbose{
                        println!("Car  {} is {} / {} to intersection {}", c_id, between.progress, between.distance_to_next_intersection, between.intersection_2);
                    }
                    
                    
                }
                else {
                    if self.verbose{
                        println!("Car {} has made it from intersection {} to intersection {}", c_id, between.intersection_1, between.intersection_2);
                    }
                    let new_pos = Position::new_current(between.intersection_2, between.arrival_direction);
                    
                    let new_dir = self.intersections.get(&between.intersection_2).unwrap_or_else(|| panic!("Cannot find intersection {}", between.intersection_2)).get_turn();
                    car.set_position(new_pos);
                    car.set_intent(new_dir);
                    cars_to_check.insert(car.get_id());
                    
                }
            }
            let id = car.get_id();
            let pos = car.get_position_mut();
            if let Some(curr) = &pos.current{
                cars_to_check.insert(id);
            }
            
        });

        let mut lane_map:HashMap<u32, u8> = HashMap::new();

        for car_id in cars_to_check{
            if let Some(pos) = self.car_can_go(car_id){
                let key = self.cars.iter().find(|c| c.get_id() == car_id).unwrap().get_position().current.as_ref().unwrap().id;
                let entry = lane_map.entry(key).or_insert(0);
                if *entry < 3{
                    *entry += 1;
                    self.cars.iter_mut().find(|car| car.get_id() == car_id).unwrap().set_position(pos);
                }
                else if self.verbose{
                    
                    println!("Car {} cannot advance because there is not enough lane space", car_id);
                }
                
            }
        }
    }


    fn car_can_go(&mut self, car_id: u32) -> Option<Position>{
        use LightColor::{Green, Red};
        // let intersection = self.intersections.get(&int_id).unwrap_or_else(|| panic!("Cannot find intersection {}", int_id));
        let car = self.cars.iter().find(|car| car.get_id() == car_id);
        if let None = car{
            return None
        }
        let car = car.unwrap();
        let intersection = self.intersections
            .get(&car.get_position()
                .current
                .as_ref()
                .unwrap_or_else(||
                    panic!("Car: {}\nPosition {:#?}",car.get_id(), &car.get_position())
                )
                .id
            )
            .unwrap();
        let config = intersection.get_lights();
        let intent = car.get_intent();
        let curr = car.get_position().current.as_ref().unwrap();
        let int_id = intersection.get_id();
        let next = self.road.get_next_intersection(int_id, intent);
        // if let None = next{
        let mut can_go = false;
        if intent == curr.direction.get_right_dir(){
            if config.get_direction(curr.direction.get_straight_dir()) == Green && config.get_direction(curr.direction.get_right_dir()) == Red{
                can_go = true
            }
        }
        //straight
        else if intent == curr.direction{
            if config.get_direction(curr.direction.get_straight_dir()) == Green{
                can_go = true
            }
        }
        //left and uturn
        else{
            if config.get_direction(curr.direction.get_straight_dir()) == Green && config.get_direction(curr.direction) == Red{
               can_go = true
            } 
        }
        if can_go{
            if let Some((next_int_id, dist)) = next{
                if self.verbose{println!("Car {} can advance {:?} from intersection {} to intersection {} because the lights are {:#?}",car_id, intent, int_id, next_int_id, config);
                println!("Car  {} is {} / {} to intersection {}", car_id, 1, dist, next_int_id);}
                return Some(Position::new_between(int_id,
                    next_int_id,
                    1,
                    dist,
                    intent.get_straight_dir()))
            }
            else{
                if self.verbose{println!("Car {} Leaves the road from Intersection{} going {:?}", car_id, int_id, intent);}
                self.cars.swap_remove(self.cars.iter().position(|c| c.get_id() == car_id).expect("Car not found"));
                return None
            }
        }
        if self.verbose{println!("Car {} cannot advance in direction {:?} because the lights are {:#?}", car_id, intent, config);}
            None
    }
    
    pub fn get_random_intersection(&self) -> u32{
        self.road.get_random_intersection()
    }

}