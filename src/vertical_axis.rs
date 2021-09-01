use std::ops::Range;

/// A VerticalAxis represents a range of i32 values. The tick interval of that range is expressed
/// as a step. The axis also has an orientation describing which side of the axis should be used
/// to convey its optional title.
///
/// The component takes a "name" property field so that it may be easily referenced when styled.
///
/// The following styling properties are available:
///
/// * axis-y - the axis as a whole
/// *   line - the axis line
/// *   tick - the axis tick line
/// *   text - the axis text
use yew::{
    prelude::*,
    services::{resize::ResizeTask, ResizeService},
    web_sys::Element,
};

pub enum Msg {
    Resize,
}

#[derive(Clone, PartialEq)]
pub enum Orientation {
    Left,
    Right,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub name: String,
    pub orientation: Orientation,
    pub scale: Range<f32>,
    pub scale_step: f32,
    pub x1: u32,
    pub y1: u32,
    pub y2: u32,
    pub tick_len: u32,
    pub title: Option<String>,
}

pub struct VerticalAxis {
    props: Props,
    _resize_task: ResizeTask,
    svg: NodeRef,
}

impl Component for VerticalAxis {
    type Message = Msg;

    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        VerticalAxis {
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

        let range_from = &p.scale.start;
        let range_to = &p.scale.end;
        let range_step = &p.scale_step;

        let range = range_to - range_from;
        let scale = (p.y2 - p.y1) as f32 / range;

        let range_from = (range_from * 100.0) as u32;
        let range_to = (range_to * 100.0) as u32;
        let range_step = (range_step * 100.0) as u32;

        html! {
            <svg ref=self.svg.clone() class={classes!("axis-y", p.name.to_owned())}>
                <line x1={p.x1.to_string()} y1={p.y1.to_string()} x2={p.x1.to_string()} y2={p.y2.to_string()} class="line" />
                { for ((range_from + range_step)..=range_to).step_by(range_step as usize).map(|i| {
                    let i = i as f32 / 100.0;
                    let x = p.x1;
                    let to_x = if p.orientation == Orientation::Left {
                        x - p.tick_len
                    } else {
                        x + p.tick_len
                    };
                    let y = (p.y1 as f32 + ((range - i) as f32 + p.scale.start) * scale) as u32;
                    html! {
                    <>
                        <line x1={x.to_string()} y1={y.to_string()} x2={to_x.to_string()} y2={y.to_string()} class="tick" />
                        <text x={to_x.to_string()} y={y.to_string()} text-anchor={if p.orientation == Orientation::Left {"end"} else {"start"}} class="text">{i}</text>
                    </>
                    }
                }) }
                { for p.title.as_ref().map(|t| {
                    let title_distance = p.tick_len << 1;
                    let (x, rotation) = if p.orientation == Orientation::Left {
                        (p.x1 - title_distance, 270)
                    } else {
                        (p.x1 + title_distance, 90)
                    };
                    let y = p.y1 + ((p.y2 - p.y1) >> 1);
                    html! {
                        <text
                            x={x.to_string()} y={y.to_string()}
                            text-anchor={"middle"}
                            transform={format!("rotate({}, {}, {})", rotation, x, y)}
                            class="title" >
                            <tspan>{"\u{25ac}\u{25ac} "}</tspan>{t}
                        </text>
                    }
                })}
            </svg>
        }
    }

    fn rendered(&mut self, _first_render: bool) {
        let p = &self.props;

        let element = self.svg.cast::<Element>().unwrap();
        let height = element.get_bounding_client_rect().height() as f32;
        let scale = (p.y2 - p.y1) as f32 / height;
        let font_size = scale * 100f32;
        let _ = element.set_attribute("font-size", &format!("{}%", &font_size));
        let _ = element.set_attribute("style", &format!("stroke-width: {}", scale));
    }
}
