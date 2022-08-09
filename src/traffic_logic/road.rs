use bevy::utils::HashMap;

struct Connection
{
    direction: Direction,
    distance: u32,
    next_intersection: RoadNode
}

struct RoadNode
{
    int_id: u32,
    connnections: Vec<Connection>
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum Direction
{
    #[default]
    North,
    East,
    South,
    West
}

impl Direction{
    pub fn get_opposite(&self) -> Self{
        match self{
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            _ => Self::East
        }
    }
}


pub struct Road
{
    graph: HashMap<u32, RoadNode>
}

impl Road{
    pub fn new() -> Self{
        Road { graph: HashMap::new() }
    }

    pub fn add_connection(&mut self, int_1: u32, int_2: u32, distance: u32, direction:Direction){
        let rNode = RoadNode{int_id:int_1, connnections: Vec::new()};
        let found = self.graph.entry(int_1).or_insert(rNode).connnections.iter().find(|c| c.direction == direction);
        if let Some(_) = found{
            panic!("Cannot add a connection in a direction where a connection has already been made");
        }

        let rNode2 = RoadNode{int_id: int_2, connnections: Vec::new()};
        let found = self.graph.entry(int_2).or_insert(rNode2).connnections.iter().find(|c| c.direction == direction.get_opposite());
        if let Some(_) = found{
            panic!("Cannot add a connection in a direction where a connection has already been made");
        }
    }
}