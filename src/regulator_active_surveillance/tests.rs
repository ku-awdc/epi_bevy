use rand::SeedableRng;
use super::*;

#[test]
fn test_active_surveillance() {
    //spell-checker:words
    let mut mini_world = World::new();

    let farm_ids: Vec<Entity> = mini_world
        .spawn_batch(vec![
            (Infected::new(0),),
            (Infected::new(1),),
            (Infected::new(10),),
            (Infected::new(100),),
            (Infected::new(1000),),
            (Infected::new(10000),),
            (Infected::new(100000),),
        ])
        .collect();

    mini_world.insert_resource(StdRng::seed_from_u64(20210507 - 10));
    mini_world.insert_resource(DetectionRate(
        Rate::try_from(Probability::new(0.01).unwrap()).unwrap(),
    ));
    mini_world.insert_resource(RemainingProportion(Probability::new(0.01).unwrap()));

    let mut stage = SystemStage::single(update_active_surveillance.system());
    stage.run(&mut mini_world);

    dbg!(farm_ids
        .into_iter()
        .map(|x| mini_world.get::<Infected>(x))
        .collect_vec());
}
