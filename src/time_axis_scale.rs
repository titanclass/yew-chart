use chrono::{DateTime, Duration, Local, NaiveDateTime, Utc};
/// A TimeAxisScale represents a linear scale for timestamps within a fixed range.
/// A step in seconds is also expressed and indicates the interval to be used for each tick on the axis.
///
/// Time is rendered in the browser's local time.
use std::ops::Range;

use crate::axis::{AxisScale, AxisTick, NormalisedValue};

const DEFAULT_LABEL_FORMAT: &str = "%d-%b";

#[derive(Clone)]
pub struct TimeAxisScale {
    time_from: i64,
    time_to: i64,
    step: i64,
    scale: f32,
    label_format: String,
}

impl TimeAxisScale {
    pub fn for_range(
        range: Range<DateTime<Utc>>,
        time_step: Duration,
        label_format: Option<String>,
    ) -> TimeAxisScale {
        let time_from = range.start.timestamp();
        let time_to = range.end.timestamp();
        let step = time_step.num_seconds();
        let scale = 1.0 / (time_to - time_from) as f32;

        TimeAxisScale {
            time_from,
            time_to,
            step,
            scale,
            label_format: label_format.unwrap_or_else(|| DEFAULT_LABEL_FORMAT.to_string()),
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
                let utc_date_time = NaiveDateTime::from_timestamp(i, 0);
                let local_date_time: DateTime<Local> =
                    DateTime::<Utc>::from_utc(utc_date_time, Utc).into();
                let date_str = local_date_time.format(&self.label_format);
                AxisTick {
                    location: NormalisedValue(location),
                    label: date_str.to_string(),
                }
            })
            .collect()
    }

    fn normalise(&self, value: f32) -> NormalisedValue {
        NormalisedValue((value - (self.time_from as f32)) * self.scale)
    }
}
