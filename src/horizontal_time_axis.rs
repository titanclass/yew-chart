use chrono::{DateTime, Duration, Local, NaiveDateTime, Utc};
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
use std::ops::Range;

use gloo_events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::{Element, SvgElement};
use yew::prelude::*;

pub enum Msg {
    Resize,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub time: Range<DateTime<Utc>>,
    pub time_step: Duration,
    pub x1: u32,
    pub x2: u32,
    pub y1: u32,
    pub tick_len: u32,
}

pub struct HorizontalTimeAxis {
    _resize_listener: Option<EventListener>,
    svg: NodeRef,
}

impl Component for HorizontalTimeAxis {
    type Message = Msg;

    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        HorizontalTimeAxis {
            _resize_listener: None,
            svg: NodeRef::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Resize => true,
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();

        let time_from = p.time.start.timestamp();
        let time_to = p.time.end.timestamp();
        let step = p.time_step.num_seconds();

        let range = time_to - time_from;
        let scale = (p.x2 - p.x1) as f32 / range as f32;

        html! {
            <svg ref={self.svg.clone()} class="time-axis-x">
                <line x1={p.x1.to_string()} y1={p.y1.to_string()} x2={p.x2.to_string()} y2={p.y1.to_string()} class="line" />
                { for ((time_from + step)..time_to).step_by(step as usize).map(|i| {
                    let x = (p.x1 as f32 + ((i - time_from) as f32 * scale)) as u32;
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

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        let p = ctx.props();

        let element = self.svg.cast::<Element>().unwrap();
        if let Some(svg_element) = element
            .first_child()
            .map(|n| n.dyn_into::<SvgElement>().ok())
            .flatten()
        {
            let width = svg_element.get_bounding_client_rect().width() as f32;
            let scale = (p.x2 - p.x1) as f32 / width;
            let font_size = scale * 100f32;
            let _ = element.set_attribute("font-size", &format!("{}%", &font_size));
            let _ = element.set_attribute("style", &format!("stroke-width: {}", scale));
            let on_resize = ctx.link().callback(|_: Event| Msg::Resize);
            self._resize_listener = Some(EventListener::new(&svg_element, "resize", move |e| {
                on_resize.emit(e.clone())
            }));
        }
    }
}
