//
// TODO: This example showcases querying entities on trait implementations
// in bevy. Is this possible?

use epi_bevy::prelude::*;

fn main() {
    let mut miniworld = World::new();
    let entity_id = miniworld.spawn().insert(432).id();

    // Query<Trait> doesn't work
    // Query<dyn Trait> doesn't work
    // Query<Box<Trait>> doesn't work
    fn my_system(query: Query<Box<std::fmt::Debug>>) {
        query.for_each(|x| {
            dbg!(x);
        });
    }

    let mut some_stage = SystemStage::single(my_system.system());

    some_stage.run(miniworld);

}
