use std::{ops::Add, ops::Sub, rc::Rc};

use chrono::{Duration, Utc};
use yew::prelude::*;
use yew_chart::{
    horizontal_time_axis::HorizontalTimeAxis,
    horizontal_time_series::{self, HorizontalTimeSeries},
    vertical_axis::{self, VerticalAxis},
};

const ONE_DAY_SECS: i64 = 86_400;

const WIDTH: u32 = 533;
const HEIGHT: u32 = 300;
const MARGIN: u32 = 50;
const TICK_LENGTH: u32 = 10;

struct App {
    data_set: Rc<Vec<(i64, f32)>>,
    end_date: i64,
    start_date: i64,
}

impl Component for App {
    type Message = ();

    type Properties = ();

    fn create(_props: Self::Properties, _link: yew::ComponentLink<Self>) -> Self {
        let end_date = Utc::now();
        let start_date = end_date.sub(Duration::days(4));
        App {
            data_set: Rc::new(vec![
                (start_date.timestamp(), 1.0),
                (start_date.add(Duration::days(1)).timestamp(), 4.0),
                (start_date.add(Duration::days(2)).timestamp(), 3.0),
                (start_date.add(Duration::days(3)).timestamp(), 2.0),
                (start_date.add(Duration::days(4)).timestamp(), 5.0),
            ]),
            end_date: end_date.timestamp(),
            start_date: start_date.timestamp(),
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
            <svg class={"chart"} viewBox={format!("0 0 {} {}", WIDTH, HEIGHT)} preserveAspectRatio="none">
                <HorizontalTimeSeries
                    series_type={horizontal_time_series::SeriesType::Line}
                    name="some-series"
                    data={Rc::clone(&self.data_set)}
                    time_from={self.start_date} time_to={self.end_date} step={ONE_DAY_SECS}
                    range_from=1.0 range_to=5.0
                    x={MARGIN} y=MARGIN width={WIDTH - (MARGIN * 2)} height={HEIGHT - (MARGIN * 2)} />

                <VerticalAxis
                    name="some-y-axis"
                    orientation={vertical_axis::Orientation::Left}
                    range_from=1.0 range_to=5.0 range_step=0.5
                    x1={MARGIN} y1={MARGIN} y2={HEIGHT - MARGIN}
                    tick_len={TICK_LENGTH}
                    title={"Some Y thing".to_string()} />

                <HorizontalTimeAxis
                    time_from={self.start_date} time_to={self.end_date} step={ONE_DAY_SECS}
                    x1={MARGIN} y1={HEIGHT - MARGIN} x2={WIDTH - MARGIN}
                    tick_len={TICK_LENGTH} />

            </svg>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
