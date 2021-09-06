use std::{
    ops::Add,
    ops::{Range, Sub},
    rc::Rc,
};

use chrono::{DateTime, Duration, Utc};
use yew::prelude::*;
use yew_chart::{
    horizontal_series::{self, HorizontalSeries},
    horizontal_time_axis::HorizontalTimeAxis,
    vertical_axis::{self, VerticalAxis},
};

const WIDTH: u32 = 533;
const HEIGHT: u32 = 300;
const MARGIN: u32 = 50;
const TICK_LENGTH: u32 = 10;

struct App {
    data_set: Rc<Vec<(f32, f32)>>,
    time: Range<DateTime<Utc>>,
}

impl Component for App {
    type Message = ();

    type Properties = ();

    fn create(_props: Self::Properties, _link: yew::ComponentLink<Self>) -> Self {
        let end_date = Utc::now();
        let start_date = end_date.sub(Duration::days(4));
        App {
            data_set: Rc::new(vec![
                (start_date.timestamp() as f32, 1.0),
                (start_date.add(Duration::days(1)).timestamp() as f32, 4.0),
                (start_date.add(Duration::days(2)).timestamp() as f32, 3.0),
                (start_date.add(Duration::days(3)).timestamp() as f32, 2.0),
                (start_date.add(Duration::days(4)).timestamp() as f32, 5.0),
            ]),
            time: start_date..end_date,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> yew::ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> yew::Html {
        html! {
            <svg class="chart" viewBox=format!("0 0 {} {}", WIDTH, HEIGHT) preserveAspectRatio="none">
                <HorizontalSeries
                    series_type=horizontal_series::SeriesType::Line
                    name="some-series"
                    data=Rc::clone(&self.data_set)
                    horizontal_scale=self.time.start.timestamp() as f32..self.time.end.timestamp() as f32
                    horizontal_scale_step=Duration::days(1).num_seconds() as f32
                    vertical_scale=0.0..5.0
                    x=MARGIN y=MARGIN width={WIDTH - (MARGIN * 2)} height={HEIGHT - (MARGIN * 2)} />

                <VerticalAxis
                    name="some-y-axis"
                    orientation=vertical_axis::Orientation::Left
                    scale=0.0..5.0 scale_step=0.5
                    x1=MARGIN y1=MARGIN y2={HEIGHT - MARGIN}
                    tick_len=TICK_LENGTH
                    title="Some Y thing".to_string() />

                <HorizontalTimeAxis
                    time=self.time.to_owned() time_step=Duration::days(1)
                    x1=MARGIN y1={HEIGHT - MARGIN} x2={WIDTH - MARGIN}
                    tick_len=TICK_LENGTH />

            </svg>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
