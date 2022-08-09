use std::collections::HashMap;

use super::{car::Car, intersection::Intersection, road::Road};



pub struct Simulator
{
    cars : Vec<Car>,
    intersections: HashMap<u8, Intersection>,
    road: Road
    
}

impl Simulator{
    pub fn new(num_cars: u32, road:Road) -> Self{
        let mut sim = Simulator { cars:Vec::new(), intersections: HashMap::new(), road };
        for i in 0..num_cars{
            sim.cars.push(Car::new(i))
        }
        sim
    }


}