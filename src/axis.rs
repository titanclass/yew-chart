/// Axis scaled value, expected to be between 0 and 1
/// except in the case where the value is outside of the axis range
#[derive(Debug, PartialEq)]
pub struct NormalisedValue(pub f32);

/// Specifies a generic scale on which axes and data can be rendered
pub trait AxisScale {
    /// Provides the list of [ticks](AxisTick) that should be rendered along the axis
    fn ticks(&self) -> Vec<AxisTick>;

    /// Normalises a value within the axis scale to a number between 0 and 1,
    /// where 0 represents the minimum value of the scale, and 1 the maximum
    ///
    /// For example, for a linear scale between 50 and 100:
    /// - normalise(50)  -> 0
    /// - normalise(60)  -> 0.2
    /// - normalise(75)  -> 0.5
    /// - normalise(100) -> 1
    fn normalise(&self, value: f32) -> NormalisedValue;
}

/// An axis tick, specifying a label to be displayed at some normalised
/// position along the axis
#[derive(Debug, PartialEq)]
pub struct AxisTick {
    /// normalised location between zero and one along the axis specifying
    /// the position at which the tick should be rendered
    pub location: NormalisedValue,

    /// The label that should be rendered alongside the tick
    pub label: String,
}
