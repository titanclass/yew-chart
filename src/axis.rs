/// Axis scaled value, expected to be between 0 and 1
/// except in the case where the value is outside of the axis range
pub struct NormalisedValue(pub f32);

/// A trait for creating an axis scale. It has two functions:
/// `ticks` returns a vector of axis ticks
/// `normalise` normalises the input `value`
pub trait AxisScale {
    fn ticks(&self) -> Vec<AxisTick>;
    fn normalise(&self, value: f32) -> NormalisedValue;
}

/// An axis tick
/// Each tick must have a location and a label.
pub struct AxisTick {
    pub location: NormalisedValue,
    pub label: String,
}
