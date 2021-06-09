# SIR implemented in ECS

## Developer environment setup

Use `nightly` or change [config](.cargo/config.toml) file.

Use `sccache`.

Currently, the default toolchain should be `nightly-gnu`. There is
an error with `msvc`.


## TODO

- [ ] Replications (reps) of scenario configuration is not implemented. These could be implemented in several ways.

    1. Take the world out once it is fully formed, and then pass it to
        clones of `App` and run these. Then find a way to aggregate the
        results through something that summarises the reps.
    2. After a world is done, `next_repetition`-method and then run the scenario
        once again.
        This `next_repetition` can do the following:

        `rng`: Increase the set seed by one, to get the next seed.
        **recorders**: Obviously these should be emptied, but they better retain
        the aggregated capacity.
        Disease compartments all set to zero.

        Re-run all startup systems, since these need to be generated again in order
        to ensure consistency throughout.
