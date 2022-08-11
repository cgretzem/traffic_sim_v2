use std::{collections::HashMap};
use rand::{Rng, seq::SliceRandom};

use super::road::Direction;

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
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
    lights: LightConfig,
    turn_probabilities : HashMap<Direction, u8>,
    ///the number of cars that can go at once
    num_lanes : u8
}
#[derive(Clone, Copy, Debug)]
pub struct LightConfig{
     /// the north light of the intersection
     pub north : LightColor,
     /// the east light of the intersection
     pub east : LightColor,
     ///the south light of the intersection
     pub south : LightColor,
     ///the west light of the intersection
     pub west : LightColor,
}

impl LightConfig{
    pub fn get_direction(&self, dir : Direction) -> LightColor{
        match dir{
            Direction::North => self.north,
            Direction::South => self.south,
            Direction::East => self.east,
            Direction::West => self.west
        }
    }
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
        
        Intersection {id, lights: LightConfig{north: Red, east: Red, south: Red, west: Red}, turn_probabilities:map, num_lanes}
    }

    //sets the light in the direction specified
    pub fn set_light(&mut self, dir : Direction, new_color : LightColor){
        match dir{
            Direction::North => {self.lights.north = new_color},
            Direction::East => {self.lights.east = new_color},
            Direction::South => {self.lights.south = new_color},
            _ => {self.lights.west = new_color}
        };
    }

    ///sets all the lights in the order 
    /// * `North`
    /// * `East`
    /// * `South`
    /// * `West`
    pub fn set_lights(&mut self, lights: LightConfig ){
        self.lights.north = lights.north;
        self.lights.east = lights.east;
        self.lights.west = lights.west;
        self.lights.south = lights.south;
    }

    pub fn get_lights(&self) -> LightConfig
    {
        self.lights
    }


    pub fn get_all_light_configs() -> [LightConfig;6]{
        use LightColor::{Green, Red};
        let mut configs:[LightConfig;6] = [LightConfig{north:Green, east: Red, south: Green, west:Red};6];

        configs[0] = LightConfig{north:Green, east: Red, south: Green, west:Red};
        configs[1] = LightConfig{north:Red, east: Green, south: Red, west:Green};
        configs[2] = LightConfig{north:Green, east: Red, south: Red, west:Red};
        configs[3] = LightConfig{north:Red, east: Green, south: Red, west:Red};
        configs[4] = LightConfig{north:Red, east: Red, south: Green, west:Red};
        configs[5] = LightConfig{north:Red, east: Red, south: Red, west:Green};
        configs
    }

    pub fn get_random_config() -> LightConfig{
        *Self::get_all_light_configs().choose(&mut rand::thread_rng()).unwrap()
    }

    ///Sets the turn probability in a given direction
    fn set_probabilities(&mut self, dir: Direction, new_prob: u8){ 
       *self.turn_probabilities.entry(dir).or_insert(25) = new_prob;
    }

    pub fn get_id(&self) -> u32{
        self.id
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


    pub fn get_num_lanes(&self) -> u8{
        self.num_lanes
    }
}