use std::{collections::HashMap};
use rand::Rng;

use super::road::Direction;

#[derive(Clone, Copy, Default)]
///represents the color of a light
pub enum LightColor{
    #[default]
    Red,
    Green
}


///Represents an Intersection. An intersection has 4 lights in the 4 cardinal directions
pub struct Intersection
{
    ///the ID of the intersection
    id: u32,
    /// the north light of the intersection
    north : LightColor,
    /// the east light of the intersection
    east : LightColor,
    ///the south light of the intersection
    south : LightColor,
    ///the west light of the intersection
    west : LightColor,
    ///the probability of a car turning a certain direction at a given light
    turn_probabilities : HashMap<Direction, u8>,
    ///the number of cars that can go at once
    num_lanes : u8
}

impl Intersection{
    ///creates a new intersection
    /// # Parameters
    /// * `id` : the ID of the intersection
    /// * `num_lanes` : the number of lanes the intersection has, determines the number of cars that can go in one tick
    pub fn new(id: u32, num_lanes : u8) -> Self{
        use LightColor::Red;
        let mut map: HashMap<Direction, u8> = HashMap::new();
        map.insert(Direction::North, 25);
        map.insert(Direction::East, 25);
        map.insert(Direction::South, 25);
        map.insert(Direction::West, 25);
        Intersection {id, north: Red, east: Red, south: Red, west: Red, turn_probabilities: HashMap::new(), num_lanes}
    }

    //sets the light in the direction specified
    pub fn set_light(&mut self, dir : Direction, new_color : LightColor){
        match dir{
            Direction::North => {self.north = new_color},
            Direction::East => {self.east = new_color},
            Direction::South => {self.south = new_color},
            _ => {self.west = new_color}
        };
    }

    ///sets all the lights in the order 
    /// * `North`
    /// * `East`
    /// * `South`
    /// * `West`
    pub fn set_lights(&mut self, north : LightColor, east : LightColor, south : LightColor, west : LightColor, ){
        self.north = north;
        self.east = east;
        self.west = west;
        self.south = south;
    }

    ///Sets the turn probability in a given direction
    fn set_probabilities(&mut self, dir: Direction, new_prob: u8){ 
       *self.turn_probabilities.entry(dir).or_insert(25) = new_prob;
    }

    ///Sets the turn probabilities in the order
    /// * `North`
    /// * `East`
    /// * `South`
    /// * `West`
    pub fn set_turn_probabilities(&mut self, north: u8, east: u8, south: u8, west: u8){
        let total = north + east + south + west;
        let normalized = (100*north/total, 100*east/total, 100*south/total, 100*west/total);
        self.set_probabilities(Direction::North, normalized.0);
        self.set_probabilities(Direction::East, normalized.1);
        self.set_probabilities(Direction::South, normalized.2);
        self.set_probabilities(Direction::West, normalized.3);
    }

    ///Gets a random next direction based on the probability to go a certain way
    pub fn get_turn(&self) -> Direction{
        let num:u8 = rand::thread_rng().gen_range(0..=100);
        let north = self.turn_probabilities.get(&Direction::North).unwrap();
        let east = self.turn_probabilities.get(&Direction::East).unwrap();
        let south = self.turn_probabilities.get(&Direction::South).unwrap();
        if &num <= north {
            Direction::North
        }
        else if num <= north + east {
            Direction::East
        }
        else if num <= north + east + south{
            Direction::South
        }
        else{
            Direction::West
        }

    }
}