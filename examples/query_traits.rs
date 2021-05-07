//
// TODO: This example showcases querying entities on trait implementations
// in bevy. Is this possible?

use bevy::prelude::*;
use std::fmt::Debug;

pub trait DebugComponent: Debug + Send + Sync {}

fn main() {
    let mut miniworld = World::new();
    let entity_id = miniworld.spawn().insert(432).id();

    fn my_system(query: Query<(Box<dyn DebugComponent + 'static>,)>) {
        query.for_each(|x| {
            dbg!(x);
        });
    }

    let mut some_stage = SystemStage::single(my_system.system());

    some_stage.run(miniworld);
}
