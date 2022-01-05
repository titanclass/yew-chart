/// A Series represents the data of a chart. Data is expressed as a vector of
/// data x/y float value tuples. A range of x is expressed along with a step. A range
/// of y datum is also expressed. If there is a break in data where one or more steps are
/// missed then any line being drawn will be stopped and then resumed accordingly.
/// A line can be labelled at various places in time represented at x, y, and a string
/// label for that point.
///
/// A name is associated with the series to facilitate styling.
use std::{cmp, ops::Range, rc::Rc};

use gloo_events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::{Element, SvgElement};
use yew::prelude::*;

pub type SeriesData = Vec<(f32, f32)>;
pub type SeriesDataLabelled = Vec<(f32, f32, Box<SeriesDataLabeller>)>;
pub type SeriesDataLabeller = dyn Fn(u32, u32) -> Html;

const DATA_LABEL_OFFSET: u32 = 3;
const CIRCLE_RADIUS: u32 = DATA_LABEL_OFFSET >> 1;

// A convenience for using a string as a label along with a circle dot.
pub fn label(text: &str) -> Box<SeriesDataLabeller> {
    let t = text.to_string();
    Box::new(move |x, y| {
        html! {
            <>
            <circle cx={x.to_string()} cy={y.to_string()} r={CIRCLE_RADIUS.to_string()} />
            <text x={x.to_string()} y={(y - DATA_LABEL_OFFSET).to_string()}>{t.to_owned()}</text>
            </>
        }
    })
}

pub enum Msg {
    Resize,
}

#[derive(Clone, PartialEq)]
pub enum SeriesType {
    Line,
    Bar,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub data: Rc<SeriesData>,
    pub data_labels: Option<Rc<SeriesDataLabelled>>,
    pub height: u32,
    pub horizontal_scale: Range<f32>,
    pub horizontal_scale_step: f32,
    pub name: String,
    pub series_type: SeriesType,
    pub vertical_scale: Range<f32>,
    pub width: u32,
    pub x: u32,
    pub y: u32,
}

impl PartialEq for Props {
    fn eq(&self, other: &Self) -> bool {
        self.series_type == other.series_type
            && self.name == other.name
            && Rc::ptr_eq(&self.data, &other.data)
            && match (&self.data_labels, &other.data_labels) {
                (Some(labels), Some(other_labels)) => Rc::ptr_eq(labels, other_labels),
                _ => false,
            }
            && self.horizontal_scale == other.horizontal_scale
            && self.horizontal_scale_step == other.horizontal_scale_step
            && self.vertical_scale == other.vertical_scale
            && self.x == other.x
            && self.y == other.y
            && self.height == other.height
            && self.width == other.width
    }
}

struct DerivedProps {
    svg_elements: Vec<Html>,
}

pub struct HorizontalSeries {
    derived_props: DerivedProps,
    _resize_listener: Option<EventListener>,
    svg: NodeRef,
}

impl HorizontalSeries {
    fn derive_props(props: &Props) -> DerivedProps {
        let classes = classes!("horizontal-series", props.name.to_owned());

        let x_range = props.horizontal_scale.end - props.horizontal_scale.start;
        let x_scale = props.width as f32 / x_range;

        let y_range = props.vertical_scale.end - props.vertical_scale.start;
        let y_scale = props.height as f32 / y_range;

        let mut svg_elements = Vec::<Html>::with_capacity(
            props.data.len() + props.data_labels.as_ref().map(|d| d.len()).unwrap_or(0),
        );

        if props.data.len() > 0 {
            let mut element_points = Vec::<(u32, u32)>::with_capacity(props.data.len());

            let mut top_y = props.height;

            let data_step = props.horizontal_scale_step;
            let mut last_data_step = -data_step;
            for (data_x, data_y) in props.data.iter() {
                let step = (data_x / data_step) * data_step;
                if step - last_data_step > data_step {
                    if props.series_type == SeriesType::Line {
                        let points = element_points
                            .iter()
                            .map(|(x, y)| format!("{},{} ", x, y))
                            .collect::<String>();
                        svg_elements.push(
                            html!(<polyline points={points} class={classes.to_owned()} fill="none"/>),
                        );
                    } else {
                        for point in element_points.iter() {
                            let x1 = point.0;
                            let y1 = point.1;
                            let x2 = x1;
                            let y2 = props.height + props.y;
                            if y1 != y2 {
                                svg_elements.push(
                                html!(<line x1={x1.to_string()} y1={y1.to_string()} x2={x2.to_string()} y2={y2.to_string()} class={classes.to_owned()}/>));
                            }
                        }
                    }
                    element_points.clear();
                }
                let x = cmp::min(
                    ((*data_x - props.horizontal_scale.start) * x_scale) as u32,
                    props.width,
                ) + props.x;
                let y = (props.height
                    - cmp::min(
                        ((*data_y - props.vertical_scale.start) * y_scale) as u32,
                        props.height,
                    ))
                    + props.y;
                top_y = cmp::min(top_y, y);
                element_points.push((x, y));

                last_data_step = step;
            }
            if props.series_type == SeriesType::Line {
                let points = element_points
                    .iter()
                    .map(|(x, y)| format!("{},{} ", x, y))
                    .collect::<String>();
                svg_elements.push(
                    html!(<polyline points={points} class={classes.to_owned()} fill="none"/>),
                );
            } else {
                for point in element_points.iter() {
                    let x1 = point.0;
                    let y1 = point.1;
                    let x2 = x1;
                    let y2 = props.height + props.y;
                    if y1 != y2 {
                        svg_elements.push(
                        html!(<line x1={x1.to_string()} y1={y1.to_string()} x2={x2.to_string()} y2={y2.to_string()} class={classes.to_owned()}/>));
                    }
                }
            }
        }

        if let Some(data_labels) = props.data_labels.as_ref() {
            for (data_x, data_y, label) in data_labels.iter() {
                let x = cmp::min(
                    ((*data_x - props.horizontal_scale.start) * x_scale) as u32,
                    props.width,
                ) + props.x;
                let y = props.height - ((*data_y - props.vertical_scale.start) * y_scale) as u32
                    + props.y;
                svg_elements.push(html! {
                    <g class={classes.to_owned()}>
                        {label(x, y)}
                    </g>
                })
            }
        }

        DerivedProps { svg_elements }
    }
}

impl Component for HorizontalSeries {
    type Message = Msg;

    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        HorizontalSeries {
            derived_props: Self::derive_props(ctx.props()),
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

        html! {
            <svg ref={self.svg.clone()}>
                <line x1={p.x.to_string()} x2={(p.x + p.width).to_string()} y1=0 y2=0 />
                { self.derived_props.svg_elements.to_owned() }
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
            let scale = p.width as f32 / width;
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
