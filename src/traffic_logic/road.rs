use bevy::utils::HashMap;
use rand::{Rng, seq::{SliceRandom, IteratorRandom}};
#[derive(Debug)]
struct Connection
{
    direction: Direction,
    distance: u32,
    next_intersection: u32
}
#[derive(Debug)]
struct RoadNode
{
    int_id: u32,
    connnections: Vec<Connection>
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Debug)]
pub enum Direction
{
    #[default]
    North,
    East,
    South,
    West
}

impl Direction{
    pub fn get_straight_dir(&self) -> Self{
        match self{
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            _ => Self::East
        }
    }

    pub fn get_left_dir(&self) -> Self
    {
        match self{
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            _ => Self::North
        }
    }


    pub fn get_right_dir(&self) -> Self
    {
        match self{
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            _ => Self::South
        }
    }

    pub fn get_random_dir() -> Self{
        let random_num = rand::thread_rng().gen_range(0..3);
        match random_num{
            0 => Self::North,
            1 => Self::East,
            2 => Self::South,
            _=> Self::West
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

    pub fn get_random_intersection(&self) -> u32{
        *self.graph.keys().choose(&mut rand::thread_rng()).unwrap()
    }

    pub fn add_connection(&mut self, int_1: u32, int_2: u32, distance: u32, direction:Direction){
        if int_1 == int_2{
            panic!("Cannot add connection from a road to itself");
        }
        let rNode = RoadNode{int_id:int_1, connnections: Vec::new()};
        let found = self.graph.entry(int_1)
                                                        .or_insert(rNode).connnections
                                                        .iter()
                                                        .find(|c|{c.direction == direction || c.next_intersection == int_2});
        if let Some(_) = found{
            panic!("Cannot add a connection in a direction where a connection has already been made");
        }

        let rNode2 = RoadNode{int_id: int_2, connnections: Vec::new()};
        let found = self.graph.entry(int_2)
                                                        .or_insert(rNode2).connnections
                                                        .iter()
                                                        .find(|c| c.direction == direction.get_straight_dir() || c.next_intersection == int_1);
        if let Some(_) = found{
            panic!("Cannot add a connection in a direction where a connection has already been made");
        }

        self.graph.get_mut(&int_1)
                                .unwrap()
                                .connnections
                                .push(Connection { direction, distance, next_intersection: int_2 });
        self.graph.get_mut(&int_2)
                                .unwrap()
                                .connnections
                                .push(Connection { direction:direction.get_straight_dir(), distance, next_intersection: int_1 });
    }

    pub fn get_next_intersection(&self, id: u32, dir: Direction) -> Option<(u32, u32)>{
        let inital_node = self.graph.get(&id).unwrap();
        if let Some(connection) = inital_node.connnections.iter().find(|conn| conn.direction == dir){
            return Some((connection.next_intersection, connection.distance));
        }
        None
    }

    pub fn get_all_intersections(&self) -> Vec<u32>{
        let mut new_vec: Vec<u32> = Vec::new();
        for key in self.graph.keys(){
            new_vec.push(*key);
        }
        new_vec
    }
}