use std::{
    ops::{Add, Sub},
    rc::Rc,
};

use chrono::{Duration, Utc};
use yew::prelude::*;
use yew_chart::{
    axis::AxisScale,
    horizontal_axis::{self, HorizontalAxis},
    horizontal_series::{self, HorizontalSeries, SeriesData},
    linear_axis_scale::LinearAxisScale,
    time_axis_scale::TimeAxisScale,
    vertical_axis::{self, VerticalAxis},
};

const WIDTH: f32 = 533.0;
const HEIGHT: f32 = 300.0;
const MARGIN: f32 = 50.0;
const TICK_LENGTH: f32 = 10.0;

struct App {
    data_set: Rc<SeriesData>,
    vertical_axis_scale: Rc<dyn AxisScale>,
    horizontal_axis_scale: Rc<dyn AxisScale>,
}

impl Component for App {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let end_date = Utc::now();
        let start_date = end_date.sub(Duration::days(4));
        let time = start_date..end_date;

        let circle_text_labeller = Rc::from(horizontal_series::circle_text_label("Label"));

        App {
            data_set: Rc::new(vec![
                (start_date.timestamp() as f32, 1.0, None),
                (
                    start_date.add(Duration::days(1)).timestamp() as f32,
                    4.0,
                    None,
                ),
                (
                    start_date.add(Duration::days(2)).timestamp() as f32,
                    3.0,
                    None,
                ),
                (
                    start_date.add(Duration::days(3)).timestamp() as f32,
                    2.0,
                    None,
                ),
                (
                    start_date.add(Duration::days(4)).timestamp() as f32,
                    5.0,
                    Some(circle_text_labeller),
                ),
            ]),
            horizontal_axis_scale: Rc::new(TimeAxisScale::new(time, Duration::days(1))),
            vertical_axis_scale: Rc::new(LinearAxisScale::new(0.0..5.0, 1.0)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> yew::Html {
        html! {
            <svg class="chart" viewBox={format!("0 0 {} {}", WIDTH, HEIGHT)} preserveAspectRatio="none">
                <HorizontalSeries
                    series_type={horizontal_series::SeriesType::Line}
                    name="some-series"
                    data={Rc::clone(&self.data_set)}
                    horizontal_scale={Rc::clone(&self.horizontal_axis_scale)}
                    horizontal_scale_step={Duration::days(2).num_seconds() as f32}
                    vertical_scale={Rc::clone(&self.vertical_axis_scale)}
                    x={MARGIN} y={MARGIN} width={WIDTH - (MARGIN * 2.0)} height={HEIGHT - (MARGIN * 2.0)} />

                <VerticalAxis
                    name="some-y-axis"
                    orientation={vertical_axis::Orientation::Left}
                    scale={Rc::clone(&self.vertical_axis_scale)}
                    x1={MARGIN} y1={MARGIN} y2={HEIGHT - MARGIN}
                    tick_len={TICK_LENGTH}
                    title={"Some Y thing".to_string()} />

                <HorizontalAxis
                    name="some-x-axis"
                    orientation={horizontal_axis::Orientation::Bottom}
                    scale={Rc::clone(&self.horizontal_axis_scale)}
                    x1={MARGIN} y1={HEIGHT - MARGIN} x2={WIDTH - MARGIN}
                    tick_len={TICK_LENGTH} />

            </svg>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
