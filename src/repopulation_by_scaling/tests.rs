use super::*;

#[test]
fn testing_the_scaling() {
    let mut miniworld = World::new();
    let some_entity = miniworld.spawn().insert_bundle((
        HerdSize::new(100),
        Susceptible(22),
        Infected(2),
        Recovered(14),
    )).id();

    let mut scaling_stage = SystemStage::single(repopulate_rescale_disease_compartments.system());

    // dbg!(miniworld.get::<HerdSize>(some_entity));
    // dbg!(miniworld.get::<Susceptible>(some_entity));
    // dbg!(miniworld.get::<Infected>(some_entity));
    // dbg!(miniworld.get::<Recovered>(some_entity));

    scaling_stage.run(&mut miniworld);

    // dbg!(miniworld.components());
    dbg!(miniworld.get::<HerdSize>(some_entity).map(|x|x.0).unwrap());
    dbg!(miniworld.get::<Susceptible>(some_entity).map(|x|x.0).unwrap());
    dbg!(miniworld.get::<Infected>(some_entity).map(|x|x.0).unwrap());
    dbg!(miniworld.get::<Recovered>(some_entity).map(|x|x.0).unwrap());

}
