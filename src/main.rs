mod plugins {
    pub mod camera;
    pub mod player;
    pub mod world;
}

use bevy::prelude::App;
use bevy::prelude::DefaultPlugins;
use plugins::camera::CameraPlugin;
use plugins::player::PlayerPlugin;
use plugins::world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CameraPlugin,
            PlayerPlugin,
            WorldPlugin,
        ))
        .run();
}
