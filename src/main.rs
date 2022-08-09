use bevy::prelude::*;
use std::env;

fn hello_world(){
    println!("Hello world")
}

struct Parser
{
    //-r
    render : bool,
    //-v
    verbose: bool,
    //-noAI
    noAI : bool
}

impl Default for Parser{
    fn default() -> Parser
    {
        Parser{render: false, verbose : true, noAI: true}
    }
}

impl Parser{
    fn new() -> Self {
        if env::args().len() > 1
        {
            let render = if let Some(_) = env::args().find(|arg|&String::from("-r") == arg){true} else {false};
            let verbose = if let Some(_) = env::args().find(|arg|&String::from("-v") == arg){true} else {false};
            let noAI = if let Some(_) = env::args().find(|arg|&String::from("-noAI") == arg){true} else {false};
            return Parser{render, verbose, noAI}
        }
        Parser::default()
    }

    fn run(&self) {
        let mut app = App::new();
        if self.verbose {
            
        }
        if !self.noAI {

        }
        if self.render {
            app.add_plugins(DefaultPlugins);
        }
        app.run()
    }
}

fn main() {
    Parser::new().run()
    
}
