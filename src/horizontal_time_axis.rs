/// A HorizontalTimeAxis represents a from and to time expressed as a timestamp
/// as represented by Chrono. A step in seconds is also expressed and indicates 
/// the interval to be used for each tick on the axis.
///
/// Time is rendered in the browser's local time.
///
/// The following styling properties are available:
///
/// * time-axis-x - the axis as a whole
/// *   line - the axis line
/// *   tick - the axis tick line
/// *   text - the axis text

use chrono::{DateTime, Local, NaiveDateTime, Utc};
use yew::{prelude::*, services::{ResizeService, resize::ResizeTask}, web_sys::Element};

pub enum Msg {
    Resize,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub time_from: i64,
    pub time_to: i64,
    pub step: i64,
    pub x1: u32,
    pub x2: u32,
    pub y1: u32,
    pub tick_len: u32,
}

pub struct HorizontalTimeAxis {
    props: Props,
    _resize_task: ResizeTask,
    svg: NodeRef,
}

impl Component for HorizontalTimeAxis {
    type Message = Msg;

    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        HorizontalTimeAxis {
            props,
            _resize_task: ResizeService::register(link.callback(|_| Msg::Resize)),
            svg: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Resize => true,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props != self.props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let p = &self.props;

        let range = p.time_to - p.time_from;
        let scale = (p.x2 - p.x1) as f32 / range as f32;

        html! {
            <svg ref=self.svg.clone() class={"time-axis-x"}>
                <line x1={p.x1.to_string()} y1={p.y1.to_string()} x2={p.x2.to_string()} y2={p.y1.to_string()} class="line" />
                { for ((p.time_from + p.step)..p.time_to).step_by(p.step as usize).map(|i| {
                    let x = (p.x1 as f32 + ((i - p.time_from) as f32 * scale)) as u32;
                    let y = p.y1;
                    let to_y = y + p.tick_len;
                    let utc_date_time = NaiveDateTime::from_timestamp(i, 0);
                    let local_date_time: DateTime<Local> = DateTime::<Utc>::from_utc(utc_date_time, Utc).into();
                    let date_str = local_date_time.format("%d-%b");
                    html! {
                    <>
                        <line x1={x.to_string()} y1={y.to_string()} x2={x.to_string()} y2={to_y.to_string()} class="tick" />
                        <text x={x.to_string()} y={to_y.to_string()} transform={format!("rotate(45, {}, {})", x, to_y + 1)}>{date_str}</text>
                    </>
                    }
                }) }
        </svg>
        }
    }

    fn rendered(&mut self, _first_render: bool) {
        let p = &self.props;

        let element = self.svg.cast::<Element>().unwrap();
        let width = element.get_bounding_client_rect().width() as f32;
        let scale = (p.x2 - p.x1) as f32 / width;
        let font_size = scale * 100f32;
        let _ = element.set_attribute("font-size", &format!("{}%", &font_size));
        let _ = element.set_attribute("style", &format!("stroke-width: {}", scale));
    }
}
