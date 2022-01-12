use std::rc::Rc;

// Axis scaled value, expected to be between 0 and 1
// except in the case where the value is outside of the axis range
pub struct NormalisedValue(pub f32);

pub trait AxisScale {
    fn ticks(&self) -> Rc<Vec<AxisTick>>;
    fn normalise(&self, value: f32) -> NormalisedValue;
}

pub struct AxisTick {
    pub location: NormalisedValue,
    pub label: String,
}
