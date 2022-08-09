pub mod car;
pub mod road;
pub mod intersection;

use bevy::{app::PluginGroupBuilder, prelude::{PluginGroup, Plugin, App}};
pub struct TrafficPlugin;

impl PluginGroup for TrafficPlugin{
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(RenderPlugin);
    }

    
}

pub struct RenderPlugin;

impl Plugin for RenderPlugin{
    fn build(&self, app: &mut App) {

    }
}