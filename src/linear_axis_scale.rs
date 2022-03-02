/// A LinearAxisScale represents a linear scale for floating point values within a fixed range.
/// A step is also expressed and indicates the interval to be used for each tick on the axis.
use std::{ops::Range, rc::Rc};

use crate::axis::{AxisScale, AxisTick, NormalisedValue};

/// An axis labeller is a closure that produces a string given a value within the axis scale
pub type Labeller = dyn Fn(f32) -> String;

fn labeller() -> Box<Labeller> {
    Box::new(move |v| (v as u32).to_string())
}

#[derive(Clone)]
pub struct LinearAxisScale {
    range: Range<f32>,
    step: f32,
    scale: f32,
    labeller: Rc<Labeller>,
}

impl LinearAxisScale {
    /// Create a new scale with a range and step and labels as a integers
    pub fn new(range: Range<f32>, step: f32) -> LinearAxisScale {
        Self::with_labeller(range, step, Rc::new(labeller()))
    }

    /// Create a new scale with a range and step and a custom labeller
    pub fn with_labeller(
        range: Range<f32>,
        step: f32,
        labeller: Rc<Box<Labeller>>,
    ) -> LinearAxisScale {
        let scale = 1.0 / (range.end - range.start);
        LinearAxisScale {
            range,
            step,
            scale,
            labeller,
        }
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
                    label: (self.labeller)(value),
                }
            })
            .collect()
    }

    fn normalise(&self, value: f32) -> NormalisedValue {
        NormalisedValue((value - self.range.start) * self.scale)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale() {
        let scale = LinearAxisScale::new(0.0..100.0, 25.0);

        assert_eq!(
            scale.ticks(),
            vec![
                AxisTick {
                    location: NormalisedValue(0.0),
                    label: "0".to_string()
                },
                AxisTick {
                    location: NormalisedValue(0.25),
                    label: "25".to_string()
                },
                AxisTick {
                    location: NormalisedValue(0.5),
                    label: "50".to_string()
                },
                AxisTick {
                    location: NormalisedValue(0.75),
                    label: "75".to_string()
                },
                AxisTick {
                    location: NormalisedValue(1.0),
                    label: "100".to_string()
                }
            ]
        );

        assert_eq!(scale.normalise(50.0), NormalisedValue(0.5));
    }
}
