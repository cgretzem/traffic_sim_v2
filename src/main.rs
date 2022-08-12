// use bevy::prelude::*;
// use std::env;

// fn hello_world(){
//     println!("Hello world")
// }

// struct Parser
// {
//     //-r
//     render : bool,
//     //-v
//     verbose: bool,
//     //-noAI
//     noAI : bool
// }

// impl Default for Parser{
//     fn default() -> Parser
//     {
//         Parser{render: false, verbose : true, noAI: true}
//     }
// }

// impl Parser{
//     fn new() -> Self {
//         if env::args().len() > 1
//         {
//             let render = if let Some(_) = env::args().find(|arg|&String::from("-r") == arg){true} else {false};
//             let verbose = if let Some(_) = env::args().find(|arg|&String::from("-v") == arg){true} else {false};
//             let noAI = if let Some(_) = env::args().find(|arg|&String::from("-noAI") == arg){true} else {false};
//             return Parser{render, verbose, noAI}
//         }
//         Parser::default()
//     }

//     fn run(&self) {
//         let mut app = App::new()
//                          .add_system(bevy::window::close_on_esc);
//         if self.verbose {
            
//         }
//         if !self.noAI {

//         }
//         if self.render {
//             app.add_plugins(DefaultPlugins);
//         }
//         app.run()
//     }
// }


use traffic_sim_v2::{simulator::Simulator, traffic_logic::road::{Road, Direction}};
fn main() {
    //Parser::new().run()

    let mut road = Road::new();
    road.add_connection(0, 1, 5, Direction::North);
    road.add_connection(1, 2, 5, Direction::North);
    let mut sim = Simulator::new(40,road);
    sim.verbose = false;
    use std::time::Instant;
    let now = Instant::now();

    for tick in 0..1000{
        //println!("----------------------\nTICK {}\n----------------------\n", tick);
        sim.tick();
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
