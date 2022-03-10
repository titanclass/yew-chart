/// A LinearScale represents a linear scale for floating point values within a fixed range.
/// A step is also expressed and indicates the interval to be used for each tick on the axis.
use std::{ops::Range, rc::Rc};

use crate::axis::{NormalisedValue, Scale, Tick};

/// An axis labeller is a closure that produces a string given a value within the axis scale
pub type Labeller = dyn Fn(f32) -> String;

fn labeller() -> Box<Labeller> {
    Box::new(|v| (v as u32).to_string())
}

#[derive(Clone)]
pub struct LinearScale {
    range: Range<f32>,
    step: f32,
    scale: f32,
    labeller: Option<Rc<Labeller>>,
}

impl LinearScale {
    /// Create a new scale with a range and step and labels as a integers
    pub fn new(range: Range<f32>, step: f32) -> LinearScale {
        Self::with_labeller(range, step, Some(Rc::from(labeller())))
    }

    /// Create a new scale with a range and step and a custom labeller
    pub fn with_labeller(
        range: Range<f32>,
        step: f32,
        labeller: Option<Rc<Labeller>>,
    ) -> LinearScale {
        let scale = 1.0 / (range.end - range.start);
        LinearScale {
            range,
            step,
            scale,
            labeller,
        }
    }
}

impl Scale for LinearScale {
    fn ticks(&self) -> Vec<Tick> {
        let scale = self.clone();
        let step_number = ((scale.range.end - scale.range.start) / scale.step).floor() as i32;
        let step_size = scale.scale * scale.step;
        (0..step_number + 1)
            .into_iter()
            .map(move |i| {
                let location = i as f32 * step_size;
                let value = scale.range.start + (i as f32 * scale.step);
                Tick {
                    location: NormalisedValue(location),
                    label: self.labeller.as_ref().map(|l| (l)(value)),
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
        let scale = LinearScale::new(0.0..100.0, 25.0);

        assert_eq!(
            scale.ticks(),
            vec![
                Tick {
                    location: NormalisedValue(0.0),
                    label: Some("0".to_string())
                },
                Tick {
                    location: NormalisedValue(0.25),
                    label: Some("25".to_string())
                },
                Tick {
                    location: NormalisedValue(0.5),
                    label: Some("50".to_string())
                },
                Tick {
                    location: NormalisedValue(0.75),
                    label: Some("75".to_string())
                },
                Tick {
                    location: NormalisedValue(1.0),
                    label: Some("100".to_string())
                }
            ]
        );

        assert_eq!(scale.normalise(50.0), NormalisedValue(0.5));
    }
}
