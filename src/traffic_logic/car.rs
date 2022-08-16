use super::{road::Direction};





///Represents a car's position in relation to intersections
/// # Members
/// * `between` : Option<Between> - information when the car is between 2 intersections
/// * `current` : Option<CurrentInt> - information when the car is at an intersection 
#[derive(Debug)]
pub struct Position
{
    ///Information for when the car is between 2 intersections
    pub between: Option<Between>,
    ///Information for when the car is at an intersection
    pub current: Option<CurrentInt>
}


///defaults to no position
impl Default for Position{
    fn default() -> Self {
        Position{between: None, current: None}
    }
}

impl Position{
    ///sets a new position with a filled between struct and an empty current struct
    pub fn new_between(int_1: u32, int_2: u32, progress:u32, distance_to_next_intersection: u32, arrival_direction: Direction) -> Position{
        Position{between: Some(Between{intersection_1: int_1, intersection_2: int_2, progress, distance_to_next_intersection, arrival_direction}), current:None}
    }
    ///sets a new position with a filled current struct and an empty between struct
    pub fn new_current(id: u32, direction: Direction) -> Position{
        Position { between: None, current: Some(CurrentInt{id, direction}) }
    }
    
}


///Represents a car sitting at an intersection
/// # Members
/// * `id` - the id of the intersection the car is sitting at
/// * `direction` - the cardinal direction that the car is waiting at
#[derive(Debug)]
pub struct CurrentInt{
    ///the ID of the intersection
    pub id: u32,
    ///the cardinal direction the car is waiting
    pub direction : Direction
}

///Represents a car driving between 2 intersections
#[derive(Debug)]
pub struct Between{
    ///the id of the intersection the car is coming from
    pub intersection_1: u32, 
    ///the id of the intersection the car is going to
    pub intersection_2: u32,
    ///how far the car has gone from the source intersection
    pub progress: u32,
    ///the total distance from intersection 1 to intersection 2
    pub distance_to_next_intersection: u32,
    ///the cardinal direction the car will arrive at
    pub arrival_direction: Direction
}

///represents a car
/// # Members
/// * `id` - the id of the car
/// * `position` - the position of the car relative to intersections
pub struct Car
{
    /// the ID of the car
    id: u32,
    ///The current position of the car
    position : Position,
    ///the direction the car wants to turn, initalized when a car arrives at an intersection
    intent: Direction
}

impl Car{
    ///creates a new car
    pub fn new(id : u32) -> Self{
        Car{id, position: Position::default(), intent:Direction::default()}
    }

    ///returns a reference to the car's position
    pub fn get_position(&self) -> &Position{
        &self.position
    }

    ///returns a mutable reference to the car's position
    pub fn get_position_mut(&mut self) -> &mut Position{
        &mut self.position
    }

    ///returns the car's id
    pub fn get_id(&self) -> u32{
        self.id
    }

    //sets the car's position to a new position
    pub fn set_position(&mut self, pos: Position){
        self.position = pos;
    }
    ///gets the car's intention
    pub fn get_intent(&self) -> Direction{
        self.intent
    }
    ///sets the car's intention to a new Direction
    pub fn set_intent(&mut self, new_intent : Direction){
        self.intent = new_intent
    }


}