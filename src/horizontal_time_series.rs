/// A HorizontalTimeSeries represents the data of a chat. Data is expressed as a vector of
/// timestamp/float value tuples. As per the HorizontalTimeAxis, a range of timestamps is
/// expressed along with a step. If there is a break in data where one or more steps are 
/// missed then any line being drawn will be stopped and then resumed accordingly.
/// A line can be labelled at various places in time represented with a timestamp, a string
/// label and a float value at that point.
///
/// A name is associated with the series to facilitate styling.

use std::{cmp, rc::Rc};

use wasm_bindgen::JsCast;
use yew::{
    prelude::*,
    services::{resize::ResizeTask, ResizeService},
    web_sys::{Element, SvgElement},
};

const DATA_LABEL_OFFSET: u32 = 3;

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
    pub series_type: SeriesType,
    pub name: String,
    pub data: Rc<Vec<(i64, f32)>>,
    pub data_labels: Option<Rc<Vec<(i64, String, f32)>>>,
    pub time_from: i64,
    pub time_to: i64,
    pub step: i64,
    pub range_from: f32,
    pub range_to: f32,
    pub x: u32,
    pub y: u32,
    pub height: u32,
    pub width: u32,
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
            && self.time_from == other.time_from
            && self.time_to == other.time_to
            && self.step == other.step
            && self.range_from == other.range_from
            && self.range_to == other.range_to
            && self.x == other.x
            && self.y == other.y
            && self.height == other.height
            && self.width == other.width
    }
}

struct DerivedProps {
    svg_elements: Vec<Html>,
}

pub struct HorizontalTimeSeries {
    props: Props,
    derived_props: DerivedProps,
    _resize_task: ResizeTask,
    svg: NodeRef,
}

impl HorizontalTimeSeries {
    fn derive_props(props: &Props) -> DerivedProps {
        let classes = classes!("horizontal-series", props.name.to_owned());

        let x_range = props.time_to - props.time_from;
        let x_scale = props.width as f32 / x_range as f32;

        let y_range = props.range_to - props.range_from;
        let y_scale = props.height as f32 / y_range as f32;

        let mut svg_elements = Vec::<Html>::with_capacity(
            props.data.len() + props.data_labels.as_ref().map(|d| d.len()).unwrap_or(0),
        );
        let mut element_points = Vec::<(u32, u32)>::with_capacity(props.data.len());

        let mut top_y = props.height;

        let mut last_step_time = -props.step;
        for (time, datum) in props.data.iter() {
            let step_time = (time / props.step) * props.step;
            if step_time - last_step_time > props.step {
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
                ((*time - props.time_from) as f32 * x_scale) as u32,
                props.width,
            ) + props.x;
            let y = (props.height
                - cmp::min(((*datum - props.range_from) * y_scale) as u32, props.height))
                + props.y;
            top_y = cmp::min(top_y, y);
            element_points.push((x, y));

            last_step_time = step_time;
        }
        if props.series_type == SeriesType::Line {
            let points = element_points
                .iter()
                .map(|(x, y)| format!("{},{} ", x, y))
                .collect::<String>();
            svg_elements
                .push(html!(<polyline points={points} class={classes.to_owned()} fill="none"/>));
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

        if let Some(data_labels) = props.data_labels.as_ref() {
            let circle_radius = DATA_LABEL_OFFSET >> 1;
            for (time, label, datum) in data_labels.iter() {
                let x = cmp::min(
                    ((*time - props.time_from) as f32 * x_scale) as u32,
                    props.width,
                ) + props.x;
                let y = props.height - ((*datum - props.range_from) * y_scale) as u32 + props.y;
                svg_elements.push(
                    html! {
                        <>
                        <circle cx={x.to_string()} cy={y.to_string()} r={circle_radius.to_string()} />
                        <text x={x.to_string()} y={(y  - DATA_LABEL_OFFSET).to_string()} class={classes.to_owned()}>{label}</text>
                        </>
                    })
            }
        }

        DerivedProps { svg_elements }
    }
}

impl Component for HorizontalTimeSeries {
    type Message = Msg;

    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        HorizontalTimeSeries {
            derived_props: Self::derive_props(&props),
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
            self.derived_props = Self::derive_props(&props);
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let p = &self.props;

        html! {
            <svg ref=self.svg.clone()>
                <line x1={p.x.to_string()} x2={(p.x + p.width).to_string()} y1=0 y2=0 />
                { self.derived_props.svg_elements.to_owned() }
            </svg>
        }
    }

    fn rendered(&mut self, _first_render: bool) {
        let p = &self.props;

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
        }
    }
}
