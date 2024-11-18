mod hello;

use crate::hello::HelloPlugin;
use bevy::prelude::App;
use bevy::DefaultPlugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}
