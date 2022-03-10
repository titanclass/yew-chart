use std::rc::Rc;

use gloo_events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::{Element, SvgElement};
use yew::prelude::*;

/// Axis scaled value, expected to be between 0 and 1
/// except in the case where the value is outside of the axis range
#[derive(Debug, PartialEq)]
pub struct NormalisedValue(pub f32);

/// Specifies a generic scale on which axes and data can be rendered
pub trait Scale {
    /// Provides the list of [ticks](AxisTick) that should be rendered along the axis
    fn ticks(&self) -> Vec<Tick>;

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
pub struct Tick {
    /// normalised location between zero and one along the axis specifying
    /// the position at which the tick should be rendered
    pub location: NormalisedValue,

    /// An optional label that should be rendered alongside the tick
    pub label: Option<String>,
}

pub enum Msg {
    Resize,
}

#[derive(Clone, PartialEq)]
pub enum Orientation {
    Left,
    Right,
    Bottom,
    Top,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub name: String,
    pub orientation: Orientation,
    pub x1: f32,
    pub y1: f32,
    pub xy2: f32,
    pub tick_len: f32,
    pub title: Option<String>,
    pub scale: Rc<dyn Scale>,
}

impl PartialEq for Props {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.orientation == other.orientation
            && self.x1 == other.x1
            && self.y1 == other.y1
            && self.xy2 == other.xy2
            && self.tick_len == other.tick_len
            && self.title == other.title
            && std::ptr::eq(
                // test reference equality, avoiding issues with vtables discussed in
                // https://github.com/rust-lang/rust/issues/46139
                &*self.scale as *const _ as *const u8,
                &*other.scale as *const _ as *const u8,
            )
    }
}

/// A Axis represents a range of f32 values. The tick interval of that range is expressed
/// as a step. The axis also has an orientation describing which side of the axis should be used
/// to convey its optional title.
///
/// The component takes a "name" property field so that it may be easily referenced when styled.
///
/// The following styling properties are available:
///
/// * axis - the axis as a whole
/// * line - the axis line
/// * tick - the axis tick line
/// * text - the axis text
pub struct Axis {
    _resize_listener: EventListener,
    svg: NodeRef,
}

impl Component for Axis {
    type Message = Msg;

    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let on_resize = ctx.link().callback(|_: Event| Msg::Resize);
        Axis {
            _resize_listener: EventListener::new(&gloo_utils::window(), "resize", move |e| {
                on_resize.emit(e.clone())
            }),
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

        fn title(x: f32, y: f32, baseline: &str, title: &str) -> Html {
            html! {
                <text
                    x={x.to_string()} y={y.to_string()}
                    dominant-baseline={baseline.to_string()}
                    text-anchor={"middle"}
                    transform-origin={format!("{} {}", x, y)}
                    class="title" >
                    {title}
                </text>
            }
        }

        let class = match p.orientation {
            Orientation::Left => "left",
            Orientation::Right => "right",
            Orientation::Bottom => "bottom",
            Orientation::Top => "top",
        };

        if p.orientation == Orientation::Left || p.orientation == Orientation::Right {
            let scale = (p.xy2 - p.y1) as f32;
            let x = p.x1;
            let to_x = if p.orientation == Orientation::Left {
                x - p.tick_len
            } else {
                x + p.tick_len
            };

            html! {
                <svg ref={self.svg.clone()} class={classes!("axis", class, p.name.to_owned())}>
                    <line x1={p.x1.to_string()} y1={p.y1.to_string()} x2={p.x1.to_string()} y2={p.xy2.to_string()} class="line" />
                    { for (p.scale.ticks().iter()).map(|Tick { location: NormalisedValue(normalised_location), label }| {
                        let y = (p.xy2 as f32 - (normalised_location * scale)) as u32;
                        html! {
                        <>
                            <line x1={x.to_string()} y1={y.to_string()} x2={to_x.to_string()} y2={y.to_string()} class="tick" />
                            if let Some(l) = label {
                                <text x={to_x.to_string()} y={y.to_string()} text-anchor={if p.orientation == Orientation::Left {"end"} else {"start"}} class="text">{l.to_string()}</text>
                            }
                        </>
                        }
                    }) }
                    { for p.title.as_ref().map(|t| {
                        let title_distance = p.tick_len * 2.0;
                        let x = if p.orientation == Orientation::Left {
                            p.x1 - title_distance
                        } else {
                            p.x1 + title_distance
                        };
                        let y = p.y1 + ((p.xy2 - p.y1) * 0.5);
                        title(x, y, "auto",t)
                    })}
                </svg>
            }
        } else {
            let scale = (p.xy2 - p.x1) as f32;
            let y = p.y1;
            let (to_y, baseline) = if p.orientation == Orientation::Top {
                (y - p.tick_len, "auto")
            } else {
                (y + p.tick_len, "hanging")
            };

            html! {
                <svg ref={self.svg.clone()} class={classes!("axis", class, p.name.to_owned())}>
                    <line x1={p.x1.to_string()} y1={p.y1.to_string()} x2={p.xy2.to_string()} y2={p.y1.to_string()} class="line" />
                    { for(p.scale.ticks().iter()).map(|Tick { location: NormalisedValue(normalised_location), label }| {
                        let x = p.x1 as f32 + normalised_location * scale;
                        html! {
                        <>
                            <line x1={x.to_string()} y1={y.to_string()} x2={x.to_string()} y2={to_y.to_string()} class="tick" />
                            if let Some(l) = label {
                                <text x={x.to_string()} y={to_y.to_string()} text-anchor="middle" transform-origin={format!("{} {}", x, to_y)} dominant-baseline={baseline.to_string()} class="text">{l.to_string()}</text>
                            }
                        </>
                        }
                    }) }
                    { for p.title.as_ref().map(|t| {
                        let title_distance = p.tick_len * 2.0;
                        let y = if p.orientation == Orientation::Top {
                            p.y1 - title_distance
                        } else {
                            p.y1 + title_distance
                        };
                        let x = p.x1 + ((p.xy2 - p.x1) * 0.5);
                        title(x, y, baseline, t)
                    })}
                </svg>
            }
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        let p = ctx.props();

        let element = self.svg.cast::<Element>().unwrap();
        if let Some(svg_element) = element
            .first_child()
            .and_then(|n| n.dyn_into::<SvgElement>().ok())
        {
            let bounding_rect = svg_element.get_bounding_client_rect();
            let scale = if p.orientation == Orientation::Left || p.orientation == Orientation::Right
            {
                let height = bounding_rect.height() as f32;
                (p.xy2 - p.y1) / height
            } else {
                let width = bounding_rect.width() as f32;
                (p.xy2 - p.x1) / width
            };
            let font_size = scale * 100.0;
            let _ = element.set_attribute("font-size", &format!("{}%", &font_size));
            let _ = element.set_attribute("style", &format!("stroke-width: {}", scale));
        }
    }
}
