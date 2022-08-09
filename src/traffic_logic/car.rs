use super::{intersection::Intersection, road::Direction};





///Represents a car's position in relation to intersections
/// # Members
/// * `between` : Option<Between> - information when the car is between 2 intersections
/// * `current` : Option<CurrentInt> - information when the car is at an intersection 
pub struct Position
{
    ///Information for when the car is between 2 intersections
    between: Option<Between>,
    ///Information for when the car is at an intersection
    current: Option<CurrentInt>
}


///defaults to no position
impl Default for Position{
    fn default() -> Self {
        Position{between: None, current: None}
    }
}


///Represents a car sitting at an intersection
/// # Members
/// * `id` - the id of the intersection the car is sitting at
/// * `direction` - the cardinal direction that the car is waiting at
struct CurrentInt{
    ///the ID of the intersection
    id: u32,
    ///the cardinal direction the car is waiting
    direction : Direction
}

///Represents a car driving between 2 intersections
struct Between{
    ///the id of the intersection the car is coming from
    intersection_1: u32, 
    ///the id of the intersection the car is going to
    intersection_2: u32,
    ///how far the car has gone from the source intersection
    progress: u32,
    ///the total distance from intersection 1 to intersection 2
    distance_to_next_intersection: u32,
    ///the cardinal direction the car will arrive at
    arrival_direction: Direction
}

///represents a car
pub struct Car
{
    /// the ID of the car
    id: u32,
    ///The current position of the car
    position : Position 
}

impl Car{
    ///creates a new car
    fn new(id : u32) -> Self{
        Car{id, position: Position::default()}
    }

    ///returns a reference to the car's position
    fn get_position(&self) -> &Position{
        &self.position
    }

    fn get_position_mut(&mut self) -> &mut Position{
        &mut self.position
    }


}