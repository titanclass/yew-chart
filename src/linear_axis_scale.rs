/// A LinearAxisScale represents a linear scale for floating point values within a fixed range.
/// A step is also expressed and indicates the interval to be used for each tick on the axis.
use std::ops::Range;

use crate::axis::{AxisScale, AxisTick, NormalisedValue};

#[derive(Clone)]
pub struct LinearAxisScale {
    range: Range<f32>,
    step: f32,
    scale: f32,
}

impl LinearAxisScale {
    pub fn for_range(range: Range<f32>, step: f32) -> LinearAxisScale {
        let scale = 1.0 / (range.end - range.start);

        LinearAxisScale { range, step, scale }
    }
}

impl AxisScale for LinearAxisScale {
    fn ticks(&self) -> Vec<AxisTick> {
        let scale = self.clone();
        let step_number = ((scale.range.end - scale.range.start) / scale.step).floor() as i32;
        let step_size = scale.scale * scale.step;
        (0..step_number + 1)
            .into_iter()
            .map(move |i| {
                let location = i as f32 * step_size;
                let value = scale.range.start + (i as f32 * scale.step);
                AxisTick {
                    location: NormalisedValue(location),
                    label: value.to_string(),
                }
            })
            .collect()
    }

    fn normalise(&self, value: f32) -> NormalisedValue {
        NormalisedValue((value - self.range.start) * self.scale)
    }
}
