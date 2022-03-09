/// A TimeAxisScale represents a linear scale for timestamps within a fixed range.
/// A step duration is also expressed and indicates the interval to be used for each tick on the axis.
use chrono::TimeZone;
use chrono::{DateTime, Duration, Local, Utc};
use std::{ops::Range, rc::Rc};

use crate::axis::{AxisScale, AxisTick, NormalisedValue};

/// An axis labeller is a closure that produces a string given a value within the axis scale
pub type Labeller = dyn Fn(i64) -> String;

fn labeller() -> Box<Labeller> {
    Box::new(move |ts| {
        let utc_date_time = Utc.timestamp(ts, 0);
        let local_date_time: DateTime<Local> = utc_date_time.into();
        local_date_time.format("%d-%b").to_string()
    })
}

#[derive(Clone)]
pub struct TimeAxisScale {
    time_from: i64,
    time_to: i64,
    step: i64,
    scale: f32,
    labeller: Option<Rc<Labeller>>,
}

impl TimeAxisScale {
    /// Create a new scale with a range and step representing labels as a day and month in local time.
    pub fn new(range: Range<DateTime<Utc>>, step: Duration) -> TimeAxisScale {
        Self::with_labeller(range, step, Some(Rc::from(labeller())))
    }

    /// Create a new scale with a range and step and custom labeller.
    pub fn with_labeller(
        range: Range<DateTime<Utc>>,
        step: Duration,
        labeller: Option<Rc<Labeller>>,
    ) -> TimeAxisScale {
        let time_from = range.start.timestamp();
        let time_to = range.end.timestamp();
        let step = step.num_seconds();
        let scale = 1.0 / (time_to - time_from) as f32;

        TimeAxisScale {
            time_from,
            time_to,
            step,
            scale,
            labeller,
        }
    }
}

impl AxisScale for TimeAxisScale {
    fn ticks(&self) -> Vec<AxisTick> {
        let scale = self.clone();
        ((self.time_from)..self.time_to + 1)
            .into_iter()
            .step_by(scale.step as usize)
            .map(move |i| {
                let location = (i - scale.time_from) as f32 * scale.scale;
                AxisTick {
                    location: NormalisedValue(location),
                    label: self.labeller.as_ref().map(|l| (l)(i)),
                }
            })
            .collect()
    }

    fn normalise(&self, value: f32) -> NormalisedValue {
        NormalisedValue((value - (self.time_from as f32)) * self.scale)
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
        let scale = TimeAxisScale::new(range, Duration::days(1));

        assert_eq!(
            scale.ticks(),
            vec![
                AxisTick {
                    location: NormalisedValue(0.0),
                    label: Some("26-Feb".to_string())
                },
                AxisTick {
                    location: NormalisedValue(0.25),
                    label: Some("27-Feb".to_string())
                },
                AxisTick {
                    location: NormalisedValue(0.5),
                    label: Some("28-Feb".to_string())
                },
                AxisTick {
                    location: NormalisedValue(0.75),
                    label: Some("01-Mar".to_string())
                },
                AxisTick {
                    location: NormalisedValue(1.0),
                    label: Some("02-Mar".to_string())
                }
            ]
        );

        assert_eq!(
            scale.normalise(end_date.sub(Duration::days(2)).timestamp() as f32),
            NormalisedValue(0.5)
        );
    }
}
