use bevy::utils::HashMap;

struct Connection
{
    direction: Direction,
    distance: u32,
    next_intersection: RoadNode
}

struct RoadNode
{
    int_id: u8,
    connnections: Vec<Connection>
}

#[derive(Clone, Copy, Default)]
pub enum Direction
{
    #[default]
    North,
    East,
    South,
    West
}


pub struct Road
{
    graph: HashMap<u8, RoadNode>
}