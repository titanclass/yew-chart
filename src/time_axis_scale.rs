/// A TimeAxisScale represents a linear scale for timestamps within a fixed range.
/// A step duration is also expressed and indicates the interval to be used for each tick on the axis.
use chrono::TimeZone;
use chrono::{DateTime, Duration, Local, Utc};
use std::{ops::Range, rc::Rc};

use crate::axis::{NormalisedValue, Scale, Tick};

/// An axis labeller is a closure that produces a string given a value within the axis scale
pub type Labeller = dyn Fn(i64) -> String;

fn local_time_labeller(format: &'static str) -> Box<Labeller> {
    Box::new(move |ts| {
        let utc_date_time = Utc.timestamp_millis(ts);
        let local_date_time: DateTime<Local> = utc_date_time.into();
        local_date_time.format(format).to_string()
    })
}

#[derive(Clone)]
pub struct TimeScale {
    time: Range<i64>,
    step: i64,
    scale: f32,
    labeller: Option<Rc<Labeller>>,
}

impl TimeScale {
    /// Create a new scale with a range and step representing labels as a day and month in local time.
    pub fn new(range: Range<DateTime<Utc>>, step: Duration) -> TimeScale {
        Self::with_local_time_labeller(range, step, "%d-%b")
    }

    /// Create a new scale with a range and step and local time labeller with a supplied format.
    pub fn with_local_time_labeller(
        range: Range<DateTime<Utc>>,
        step: Duration,
        format: &'static str,
    ) -> TimeScale {
        Self::with_labeller(range, step, Some(Rc::from(local_time_labeller(format))))
    }

    /// Create a new scale with a range and step and custom labeller.
    pub fn with_labeller(
        range: Range<DateTime<Utc>>,
        step: Duration,
        labeller: Option<Rc<Labeller>>,
    ) -> TimeScale {
        let time_from = range.start.timestamp_millis();
        let time_to = range.end.timestamp_millis();
        let delta = time_to - time_from;
        let scale = if delta != 0 { 1.0 / delta as f32 } else { 1.0 };
        let step = step.num_milliseconds();

        TimeScale {
            time: time_from..time_to,
            step,
            scale,
            labeller,
        }
    }
}

impl Scale for TimeScale {
    fn ticks(&self) -> Vec<Tick> {
        TimeScaleInclusiveIter {
            time_from: self.time.start,
            time_to: self.time.end,
            step: self.step,
            first_time: true,
        }
        .map(move |i| {
            let location = (i - self.time.start) as f32 * self.scale;
            Tick {
                location: NormalisedValue(location),
                label: self.labeller.as_ref().map(|l| (l)(i)),
            }
        })
        .collect()
    }

    fn normalise(&self, value: f32) -> NormalisedValue {
        NormalisedValue((value - (self.time.start as f32)) * self.scale)
    }
}

struct TimeScaleInclusiveIter {
    pub time_from: i64,
    pub time_to: i64,
    pub step: i64,
    pub first_time: bool,
}

impl Iterator for TimeScaleInclusiveIter {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let time = if !self.first_time {
            if let Some(time) = self.time_from.checked_add(self.step) {
                self.time_from = time;
                Some(time)
            } else {
                None
            }
        } else {
            self.first_time = false;
            Some(self.time_from)
        };
        if let Some(t) = time {
            if t <= self.time_to {
                time
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::ops::Sub;

    #[test]
    fn test_scale() {
        let end_date = Local.ymd(2022, 3, 2).and_hms(16, 56, 0);
        let start_date = end_date.sub(Duration::days(4));
        let range = start_date.into()..end_date.into();
        let scale = TimeScale::new(range, Duration::days(1));

        assert_eq!(
            scale.ticks(),
            vec![
                Tick {
                    location: NormalisedValue(0.0),
                    label: Some("26-Feb".to_string())
                },
                Tick {
                    location: NormalisedValue(0.25),
                    label: Some("27-Feb".to_string())
                },
                Tick {
                    location: NormalisedValue(0.5),
                    label: Some("28-Feb".to_string())
                },
                Tick {
                    location: NormalisedValue(0.75),
                    label: Some("01-Mar".to_string())
                },
                Tick {
                    location: NormalisedValue(1.0),
                    label: Some("02-Mar".to_string())
                }
            ]
        );

        assert_eq!(
            scale.normalise(end_date.sub(Duration::days(2)).timestamp_millis() as f32),
            NormalisedValue(0.4998637)
        );
    }

    #[test]
    fn test_zero_range() {
        let end_date = Local.ymd(2022, 3, 2).and_hms(16, 56, 0);
        let start_date = end_date.clone();
        let range = start_date.into()..end_date.into();
        let scale = TimeScale::new(range, Duration::days(1));

        assert_eq!(
            scale.ticks(),
            vec![Tick {
                location: NormalisedValue(0.0),
                label: Some("02-Mar".to_string())
            },]
        );

        assert_eq!(
            scale.normalise(end_date.timestamp_millis() as f32),
            NormalisedValue(0.0)
        );
    }
}
