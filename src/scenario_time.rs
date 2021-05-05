//! Scenario time
//!
//! Use when simulating a scenario to encapsulate all time readings, and timed events.
//!

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

/// Scenario time ticks in days
pub type Time = u64;

/// Total number of days in a year.
pub const DAYS_IN_A_YEAR: Time = 364;
/// Total number of weeks in a year.
pub const WEEKS_IN_A_YEAR: Time = 52;
/// Total number of days within a week.
pub const DAYS_IN_A_WEEK: Time = 7;
// month is harder as 30 x 12 = 360

/// Scenario time object
///
/// This object keeps track of elapsed timesteps.
///
/// A necessary installation as there are dynamics that operate within specific
/// timepoints.
///
#[readonly::make]
// #[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct ScenarioTime {
    start_time: Time,
    end_time: Option<Time>,
    elapsed_time: Time,
}

//TODO: inline everything here..

impl ScenarioTime {
    /// Create a `ScenarioTime`.
    ///
    /// Note: it is okay to initialize with 0, but don't use before incrementing time.
    ///
    /// `end_time` is optional
    pub fn new(start_time: Time, end_time: impl Into<Option<Time>> + Copy) -> Self {
        // assert!(start_time >= 0); //FIXME: is this necessary?
        if let Some(actual_end_time) = end_time.into() {
            debug_assert!(start_time <= actual_end_time);
        }

        Self {
            start_time,
            end_time: end_time.into(),
            elapsed_time: 0,
        }
    }

    /// Returns true if it is the first day of the year.
    #[must_use]
    pub fn first_day_of_the_year(&self) -> bool {
        self.day_in_the_year() == 1
    }

    /// Returns true if it is the last day of the year.
    #[must_use]
    pub fn last_day_of_the_year(&self) -> bool {
        self.day_in_the_year() == DAYS_IN_A_YEAR
    }

    /// Increment scenario time.
    ///
    /// Should only be invoked from one single, central location
    pub fn update_time(&mut self, increment_time: Time) {
        self.elapsed_time += increment_time;
    }

    /// Return current time in days
    #[must_use]
    pub fn current_time(&self) -> Time {
        //TODO: handle the case when `current_time` is greater than `end_time`
        assert!(
            self.elapsed_time + self.start_time > 0,
            "module assumes that day time is always greater than 0, instead: {}",
            self.elapsed_time + self.start_time
        );
        self.elapsed_time + self.start_time
    }

    /// Return the first date of the scenario
    #[must_use]
    pub const fn start_time(&self) -> Time {
        self.start_time
    }

    /// Return the specified end time
    #[must_use]
    pub const fn end_time(&self) -> Option<Time> {
        self.end_time
    }

    /// Return what year the scenario is in
    ///
    /// Note: Starting with year 0
    #[must_use]
    pub fn year(&self) -> Time {
        let year = self.current_time();
        if year > DAYS_IN_A_YEAR {
            year / DAYS_IN_A_YEAR + if year % DAYS_IN_A_YEAR == 0 { 0 } else { 1 }
        } else {
            1
        }
    }

    /// Return elapsed time
    #[must_use]
    pub fn elapsed_duration(&self) -> Time {
        self.current_time() - self.start_time
    }

    /// Return the set scenario duration, unless `end_time` isn't given, in which default to
    /// elapsed duration;
    #[must_use]
    pub fn scenario_duration(&self) -> Time {
        if let Some(end_time) = self.end_time {
            end_time - self.start_time
        } else {
            self.elapsed_duration()
        }
    }

    /// Returns the day in the year within the range `1..=364`.
    #[must_use]
    pub fn day_in_the_year(&self) -> Time {
        let days_count = self.current_time();
        if days_count > DAYS_IN_A_YEAR {
            if days_count % DAYS_IN_A_YEAR == 0 {
                DAYS_IN_A_YEAR
            } else {
                days_count % (DAYS_IN_A_YEAR)
                    + if days_count % DAYS_IN_A_YEAR == 0 {
                        1
                    } else {
                        0
                    }
            }
        } else {
            days_count
        }
    }

    /// Returns the current week no. within range `1..=52`.
    #[must_use]
    pub fn current_week_in_the_year(&self) -> Time {
        let day_in_a_year = self.day_in_the_year();
        if day_in_a_year > DAYS_IN_A_WEEK {
            day_in_a_year / DAYS_IN_A_WEEK
                + if day_in_a_year % DAYS_IN_A_WEEK == 0 {
                    0
                } else {
                    1
                }
        } else {
            1
        }
    }

    /// Return 2000-01-01 and use it as the first actual date in the scenario simulation.
    ///
    /// This is useful when plotting the scenario data.
    #[must_use]
    pub fn first_day_date(&self) -> chrono::NaiveDate {
        chrono::NaiveDate::from_ymd(2000, 1, 1)
    }

    /// Returns true if it is the first day of the week, optionally provide which week to consider.
    #[must_use]
    pub fn first_day_of_week(&self, week_no: Option<Time>) -> bool {
        let day_counter = self.current_time();
        day_counter % 7 == 1 && week_no.map_or(true, |x| x == self.current_week_in_the_year())
    }

    /// Returns true if the scenario has ended according to the provided `end_time`.
    #[must_use]
    pub fn ended(&self) -> bool {
        let end_time = self.end_time.expect("there is no end time given");
        self.current_time() >= end_time
    }
}

impl std::fmt::Display for ScenarioTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Time: {}; Week no. {}",
            self.current_time(),
            self.current_week_in_the_year()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use std::collections::{BTreeMap, HashSet};

    #[test]
    fn test_first_day_of_week() {
        let duration = 6 * 364;
        let mut days = Vec::with_capacity(duration);
        let mut scenario_time = ScenarioTime::new(0, duration as Time);
        // println!("{:>7?}", ("Tick", "Day", "Week", "Year", "FDIW", "FDIW in 28",));
        loop {
            scenario_time.update_time(1);
            if scenario_time.first_day_of_the_year() {
                println!("First day of the year");
            }
            let day_tuple = (
                scenario_time.current_time(),
                scenario_time.day_in_the_year(),
                scenario_time.current_week_in_the_year(),
                scenario_time.year(),
                scenario_time.first_day_of_week(None),
                scenario_time.first_day_of_week(Some(28)),
            );

            println!("{:>7?}", day_tuple);
            days.push(day_tuple);

            if scenario_time.ended() {
                break;
            }
        }
    }

    #[test]
    fn test_day_in_a_year() {
        let example = ScenarioTime::new(1, None);
        assert_eq!(example.day_in_the_year(), 1);
        let example = ScenarioTime::new(2, None);
        assert_eq!(example.day_in_the_year(), 2);
        let example = ScenarioTime::new(363, None);
        assert_eq!(example.day_in_the_year(), 363);
        let example = ScenarioTime::new(364, None);
        assert_eq!(example.day_in_the_year(), 364);
        let example = ScenarioTime::new(365, None);
        assert_eq!(example.day_in_the_year(), 1);
        let example = ScenarioTime::new(366, None);
        assert_eq!(example.day_in_the_year(), 2);

        let example = ScenarioTime::new(363 + 364, None);
        assert_eq!(example.day_in_the_year(), 363);
        let example = ScenarioTime::new(364 + 364, None);
        assert_eq!(example.day_in_the_year(), 364);
        let example = ScenarioTime::new(365 + 364, None);
        assert_eq!(example.day_in_the_year(), 1);
        let example = ScenarioTime::new(366 + 364, None);
        assert_eq!(example.day_in_the_year(), 2);
    }

    #[test]
    fn test_count_weeks_in_multiple_years() {
        let duration = (20 * 364) as Time;
        let mut days = Vec::with_capacity(duration as _);
        let mut scenario_time = ScenarioTime::new(0, duration);
        loop {
            scenario_time.update_time(1);
            if scenario_time.first_day_of_the_year() {
                println!("First day of the year");
            }
            let day_tuple = (
                scenario_time.current_time(),
                scenario_time.day_in_the_year(),
                scenario_time.current_week_in_the_year(),
                scenario_time.year(),
            );

            println!(
                "{} {} {} {}",
                day_tuple.0, day_tuple.1, day_tuple.2, day_tuple.3
            );
            days.push(day_tuple);

            if scenario_time.ended() {
                break;
            }
        }
        assert_eq!(
            scenario_time.elapsed_duration(),
            duration,
            "Duration wasn't reached exactly."
        );

        for x in &days {
            assert_ne!(
                x.0, 0,
                "all time representations are non-zero; instead: {:?}",
                x.0
            );
            assert_ne!(
                x.1, 0,
                "all time representations are non-zero; instead: {:?}",
                x.1
            );
            assert_ne!(
                x.2, 0,
                "all time representations are non-zero; instead: {:?}",
                x.2
            );
            assert_ne!(
                x.3, 0,
                "all time representations are non-zero; instead: {:?}",
                x.3
            );
        }

        let mut day_in_a_year = BTreeMap::new();
        let mut days_in_a_week = BTreeMap::new();
        let mut weeks_year = BTreeMap::new();
        for (_day_count, day, week, year) in days.into_iter() {
            day_in_a_year
                .entry((day, year))
                .and_modify(|x| *x += 1)
                .or_insert(1);
            days_in_a_week
                .entry((week, year))
                .and_modify(|x| *x += 1)
                .or_insert(1);
            weeks_year
                .entry(year)
                .and_modify(|x: &mut HashSet<_>| {
                    x.insert(week);
                })
                .or_insert(maplit::hashset! { week });
        }

        for &day_count in day_in_a_year.values() {
            assert_eq!(day_count, 1, "Counts must all be equal to 1.");
        }

        for &days_in_a_week_count in days_in_a_week.values() {
            assert_eq!(days_in_a_week_count, 7, "Counts must all be equal to 7.");
        }

        for x in weeks_year {
            assert_eq!(
                x.1.len() as Time,
                WEEKS_IN_A_YEAR,
                "There must be {} weeks in a year.",
                WEEKS_IN_A_YEAR
            );
        }
    }

    #[test]
    fn test_current_week_in_the_year() {
        // assert_eq!(ScenarioTime::new(0, None).current_week_in_the_year(), 0 + 1);
        assert_eq!(ScenarioTime::new(1 + 1, None).current_week_in_the_year(), 1);
        assert_eq!(ScenarioTime::new(1 + 2, None).current_week_in_the_year(), 1);
        assert_eq!(ScenarioTime::new(1 + 3, None).current_week_in_the_year(), 1);
        assert_eq!(ScenarioTime::new(1 + 4, None).current_week_in_the_year(), 1);
        assert_eq!(ScenarioTime::new(1 + 5, None).current_week_in_the_year(), 1);
        assert_eq!(ScenarioTime::new(1 + 6, None).current_week_in_the_year(), 1);
        assert_eq!(
            ScenarioTime::new(1 + 7, None).current_week_in_the_year(),
            1 + 1
        );
        assert_eq!(
            ScenarioTime::new(1 + 8, None).current_week_in_the_year(),
            1 + 1
        );
        assert_eq!(
            ScenarioTime::new(1 + 9, None).current_week_in_the_year(),
            1 + 1
        );
        assert_eq!(
            ScenarioTime::new(1 + 10, None).current_week_in_the_year(),
            1 + 1
        );
        assert_eq!(
            ScenarioTime::new(1 + 11, None).current_week_in_the_year(),
            1 + 1
        );
        assert_eq!(
            ScenarioTime::new(1 + 12, None).current_week_in_the_year(),
            1 + 1
        );
        assert_eq!(
            ScenarioTime::new(1 + 13, None).current_week_in_the_year(),
            1 + 1
        );
        assert_eq!(
            ScenarioTime::new(1 + 14, None).current_week_in_the_year(),
            2 + 1
        );
        assert_eq!(
            ScenarioTime::new(1 + 364 - 7, None).current_week_in_the_year(),
            51 + 1
        );
        assert_eq!(
            ScenarioTime::new(1 + 364 - 6, None).current_week_in_the_year(),
            51 + 1
        );
        assert_eq!(
            ScenarioTime::new(1 + 364 - 5, None).current_week_in_the_year(),
            51 + 1
        );
        assert_eq!(
            ScenarioTime::new(1 + 364 - 4, None).current_week_in_the_year(),
            51 + 1
        );
        assert_eq!(
            ScenarioTime::new(1 + 364 - 3, None).current_week_in_the_year(),
            51 + 1
        );
        assert_eq!(
            ScenarioTime::new(1 + 364 - 2, None).current_week_in_the_year(),
            51 + 1
        );
        assert_eq!(
            ScenarioTime::new(364, None).current_week_in_the_year(),
            51 + 1
        );
        assert_eq!(
            ScenarioTime::new(1 + 365, None).current_week_in_the_year(),
            1
        );
        assert_eq!(
            ScenarioTime::new(1 + 364, None).current_week_in_the_year(),
            1
        );
        assert_eq!(
            ScenarioTime::new(1 + 365 + 7, None).current_week_in_the_year(),
            1 + 1
        );
        assert_eq!(
            ScenarioTime::new(1 + 365 + 14, None).current_week_in_the_year(),
            2 + 1
        );
    }

    #[test]
    fn tests() {
        let ex_current_time = vec![1, 45, 365, 365 + 1, 365 + 2];
        assert_eq!(
            ex_current_time.iter().map(|x| x % 365 == 1).collect_vec(),
            vec!(true, false, false, true, false)
        );

        let mut some_time = ScenarioTime::new(1, 365 + 10);
        assert!(some_time.first_day_of_the_year());

        some_time.update_time(120);
        some_time.update_time(1);
        some_time.update_time(1);
        some_time.update_time(1);

        assert!(!some_time.first_day_of_the_year());
        // expect_equal(some_time$first_day_of_year(), false)
        some_time.update_time(361);
        // # five days have elapsed, lets go to a year
        // # five days have elapsed, lets go to a year

        // dbg!(some_time.current_time());
        // assert!(some_time.first_day_of_the_year());
    }

    #[test]
    fn test_elapsed_duration() {
        for &start_time_offset in &[1, 21, 423, 12] {
            let mut scenario_time = ScenarioTime::new(start_time_offset, None);
            for _ in 0..25 {
                assert_eq!(
                    scenario_time.elapsed_duration(),
                    scenario_time.current_time() - start_time_offset
                );
                scenario_time.update_time(14);
            }
        }
    }
}
