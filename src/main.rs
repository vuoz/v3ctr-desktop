use bevy::prelude::*;

use std::any::TypeId;

#[cfg(not(feature = "reload"))]
use v3ctr::*;
#[cfg(feature = "reload")]
use v3ctr_hot::*;

#[cfg(feature = "reload")]
#[hot_lib_reloader::hot_module(dylib = "v3ctr")]
mod v3ctr_hot {
    use bevy::input::mouse::MouseMotion;
    use bevy::input::mouse::MouseWheel;
    use bevy::prelude::*;
    use components::*;
    use v3ctr::*;
    hot_functions_from_file!("v3ctr/src/lib.rs");
}

fn main() {
    println!("from main: {:?}", TypeId::of::<MouseButton>());
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, v3ctr::setup)
        .add_systems(Update, camera_zoom_system)
        .add_systems(Update, orbit_camera_system)
        .add_systems(Update, rotate_cube)
        .run();
}
