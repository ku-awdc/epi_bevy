use std::sync::Mutex;

//
// Here we investigate if systems can wholeheartedly take hold of [World] and
// ammend entities with some properties, circumventing the command-framework.
//
//
//
use epi_bevy::{populations::HerdSize, prelude::*};

fn main() {
    let mut world = World::new();
    let mut rng = StdRng::seed_from_u64(20210608);

    world.insert_resource(HerdSize::new_single_population(100));

    let entities: Vec<Entity> = (0..14)
        .map(|_| rng.gen_range(90..145))
        .map(|number| world.spawn().insert_bundle(((), number)).id())
        .collect_vec();
    // world.insert_resource(Mutex::new(rng));
    world.insert_resource(rng);
    world.insert_resource(entities);

    let mut main_loop = SystemStage::single_threaded();

    main_loop.add_system(add_rate_to_everyone.exclusive_system());
    main_loop.run(&mut world);

    let entities = world.get_resource::<Vec<Entity>>().cloned().unwrap();
    let random_flag = entities
        .into_iter()
        .filter_map(|x| world.get::<bool>(x))
        .copied()
        .collect_vec();
    dbg!(random_flag);
}

/// This has to be an `.exclusive_system` and these can only have `&mut World`
/// as argument. But locally we can inquiry the world about these things.
fn add_rate_to_everyone(world: &mut World) {
    let entities = world.get_resource::<Vec<Entity>>().cloned().unwrap();

    world.resource_scope(|world, mut rng: Mut<StdRng>| {
        entities.into_iter().for_each(|entity| {
            let random_flag = rng.gen_bool(0.2);
            world.entity_mut(entity).insert(random_flag);
        });
    });
}
