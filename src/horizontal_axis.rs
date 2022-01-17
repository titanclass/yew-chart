/// A HorizontalAxis represents a range of i32 values. The tick interval of that range is expressed
/// as a step. The axis also has an orientation describing which side of the axis should be used
/// to convey its optional title.
///
/// The component takes a "name" property field so that it may be easily referenced when styled.
///
/// The following styling properties are available:
///
/// * axis-x - the axis as a whole
/// *   line - the axis line
/// *   tick - the axis tick line
/// *   text - the axis text
use std::rc::Rc;

use gloo_events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::{Element, SvgElement};
use yew::prelude::*;

use crate::axis::{AxisScale, AxisTick, NormalisedValue};

pub enum Msg {
    Resize,
}

#[derive(Clone, PartialEq)]
pub enum Orientation {
    Bottom,
    Top,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub name: String,
    pub orientation: Orientation,
    pub x1: u32,
    pub x2: u32,
    pub y1: u32,
    pub tick_len: u32,
    pub title: Option<String>,
    pub scale: Rc<dyn AxisScale>,
}

impl PartialEq for Props {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.orientation == other.orientation
            && self.x1 == other.x1
            && self.x2 == other.x2
            && self.y1 == other.y1
            && self.tick_len == other.tick_len
            && self.title == other.title
            && std::ptr::eq(
                &*self.scale as *const dyn AxisScale as *const u8,
                &*other.scale as *const dyn AxisScale as *const u8,
            )
    }
}

pub struct HorizontalAxis {
    _resize_listener: Option<EventListener>,
    svg: NodeRef,
}

impl Component for HorizontalAxis {
    type Message = Msg;

    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        HorizontalAxis {
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

        let scale = (p.x2 - p.x1) as f32;
        let y = p.y1;
        let to_y = if p.orientation == Orientation::Top {
            y - p.tick_len
        } else {
            y + p.tick_len
        };

        html! {
            <svg ref={self.svg.clone()} class={classes!("axis-x", p.name.to_owned())}>
                <line x1={p.x1.to_string()} y1={p.y1.to_string()} x2={p.x2.to_string()} y2={p.y1.to_string()} class="line" />
                { for(p.scale.ticks().iter()).map(|AxisTick { location: NormalisedValue(normalised_location), label }| {
                    let x = p.x1 as f32 + normalised_location * scale;
                    html! {
                    <>
                        <line x1={x.to_string()} y1={y.to_string()} x2={x.to_string()} y2={to_y.to_string()} class="tick" />
                        <text x={(x + 1.0).to_string()} y={to_y.to_string()} text-anchor="start" transform-origin={format!("{} {}", x, to_y + 1)} class="text">{label.to_string()}</text>
                    </>
                    }
                }) }
                { for p.title.as_ref().map(|t| {
                    let title_distance = p.tick_len << 1;
                    let y = if p.orientation == Orientation::Top {
                        p.y1 - title_distance
                    } else {
                        p.y1 + title_distance
                    };
                    let x = p.x1 + ((p.x2 - p.x1) >> 1);
                    html! {
                        <text
                            x={x.to_string()} y={y.to_string()}
                            text-anchor={"middle"}
                            class="title" >
                            <tspan>{"\u{25ac}\u{25ac} "}</tspan>{t}
                        </text>
                    }
                })}
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
