/// A Series represents the data of a chart. Data is expressed as a vector of
/// data x/y float value tuples. A range of x is expressed along with a step. A range
/// of y datum is also expressed. If there is a break in data where one or more steps are
/// missed then any line being drawn will be stopped and then resumed accordingly.
/// A line can be labelled at various places in time represented at x, y, and a string
/// label for that point.
///
/// A name is associated with the series to facilitate styling.
use std::rc::Rc;

use gloo_events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::{Element, SvgElement};
use yew::{prelude::*, virtual_dom::VNode};

use crate::axis::Scale;

/// Describes a closure that takes data values (x, y) and produces Html as the label
pub type Labeller = dyn Fn(f32, f32) -> Html;

/// Describes a closure that takes data values (x, y) and produces tooltip strings for
/// each datapoint.
pub type Tooltipper = dyn Fn(f32, f32) -> String;

/// A callback for displaying tooltip data given a mouseover event.
#[cfg(feature = "custom-tooltip")]
pub type TooltipCallback = Callback<(MouseEvent, String)>;

/// Describes a data series with each point optionally receiving a labeller
pub type Data = Vec<(f32, f32, Option<Rc<Labeller>>)>;

const DATA_LABEL_OFFSET: f32 = 3.0;
const CIRCLE_RADIUS: f32 = DATA_LABEL_OFFSET * 0.5;

// A convenience for using an optional string as a label along with a circle dot.
fn label(text: Option<String>) -> Box<Labeller> {
    Box::new(move |x, y| {
        html! {
            <>
            <circle cx={x.to_string()} cy={y.to_string()} r={CIRCLE_RADIUS.to_string()} />
            if let Some(t) = &text {
                <text x={x.to_string()} y={(y - DATA_LABEL_OFFSET).to_string()}>{t.to_owned()}</text>
            }
            </>
        }
    })
}

/// A circle dot label.
pub fn circle_label() -> Box<Labeller> {
    label(None)
}

/// A circle dot label with associated text.
pub fn circle_text_label(text: &str) -> Box<Labeller> {
    label(Some(text.to_string()))
}

/// Basic tooltip that just outputs a y value
pub fn y_tooltip() -> Box<Tooltipper> {
    Box::new(|_, y| (y as i32).to_string())
}

pub enum Msg {
    Resize,
}

/// Describes how to process each item of series data
#[derive(Clone, PartialEq)]
pub enum Type {
    /// Plots the data points as bars
    Bar(BarType),
    /// Plots the data points as lines
    Line,
    /// Does not join the data points - relies on a labeller
    Scatter,
}

///Describes the direction that the bars in a Bar Chart point
#[derive(PartialEq, Clone, Copy)]
pub enum BarType {
    ///Each bar begins at the bottom of the graph and rises to the given datapoint.
    Rise,
    ///Each bar begins at the top of the graph and drops to the given datapoint.
    Drop,
}

#[derive(Properties, Clone)]
pub struct Props {
    /// A vector of data points that represents the series, along with optional labels at each point
    pub data: Rc<Data>,
    /// The SVG height of the series
    pub height: f32,
    /// The scaling factor for data along the x axis
    pub horizontal_scale: Rc<dyn Scale>,
    /// The horizontal scale step is used to determine when there is a gap in data, such that
    /// if a line chart was drawn, then if two data items are separated by more than this can,
    /// the line will end and start again. For scatter plots, this property does not get used.
    /// If None then this functionality is disabled.
    pub horizontal_scale_step: Option<f32>,
    /// A name to be used for CSS selection
    pub name: String,
    #[cfg(feature = "custom-tooltip")]
    /// A callback to receive mouseover events along with tooltipper function text results. Requires
    /// the custom-tooltip feature.
    pub onmouseover: Rc<TooltipCallback>,
    /// The type of series to be rendered
    pub series_type: Type,
    /// An optional function that renders a string to be used for tooltips
    pub tooltipper: Option<Rc<Tooltipper>>,
    /// The scaling factor for data along the y axis
    pub vertical_scale: Rc<dyn Scale>,
    /// The SVG width of the series
    pub width: f32,
    /// The start position
    pub x: f32,
    /// The start position
    pub y: f32,
}

impl Props {
    #[cfg(feature = "custom-tooltip")]
    fn is_onmouseover_eq(&self, other: &Self) -> bool {
        self.onmouseover == other.onmouseover
    }
    #[cfg(not(feature = "custom-tooltip"))]
    fn is_onmouseover_eq(&self, _other: &Self) -> bool {
        true
    }
}

impl PartialEq for Props {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.data, &other.data)
            && self.height == other.height
            && self.horizontal_scale_step == other.horizontal_scale_step
            && self.name == other.name
            && self.is_onmouseover_eq(other)
            && self.series_type == other.series_type
            && match (self.tooltipper.as_ref(), other.tooltipper.as_ref()) {
                (Some(left), Some(right)) => std::ptr::eq(&*left as *const _ as *const u8, &*right as *const _ as *const u8),
                _=> false
            }
            && self.width == other.width
            && self.x == other.x
            && self.y == other.y
            // test reference equality, avoiding issues with vtables discussed in
            // https://github.com/rust-lang/rust/issues/46139
            && std::ptr::eq(
                &*self.horizontal_scale as *const _ as *const u8,
                &*other.horizontal_scale as *const _ as *const u8,
            )
            && std::ptr::eq(
                &*self.vertical_scale as *const _ as *const u8,
                &*other.vertical_scale as *const _ as *const u8,
            )
    }
}

struct DerivedProps {
    svg_elements: Vec<Html>,
}

pub struct Series {
    derived_props: DerivedProps,
    _resize_listener: EventListener,
    svg: NodeRef,
}

impl Series {
    fn derive_props(props: &Props) -> DerivedProps {
        let classes = classes!("series", props.name.to_owned());

        let x_scale = props.width;
        let y_scale = props.height;

        let mut svg_elements = Vec::<Html>::with_capacity(props.data.len() * 2);

        if props.data.len() > 0 {
            let mut element_points = Vec::<(f32, f32, f32, f32)>::with_capacity(props.data.len());

            let mut top_y = props.height;

            let data_step = props.horizontal_scale_step.unwrap_or(f32::MAX);
            let mut last_data_step = -data_step;
            for (data_x, data_y, labeller) in props.data.iter() {
                let (data_x, data_y) = (*data_x, *data_y);
                let step = (data_x / data_step) * data_step;
                if step - last_data_step > data_step {
                    draw_chart(&element_points, props, &mut svg_elements, &classes);
                    element_points.clear();
                }
                let x = (props.horizontal_scale.normalise(data_x).0 * x_scale.min(props.width))
                    + props.x;
                let y = props.height
                    - (props.vertical_scale.normalise(data_y).0 * y_scale).min(props.height)
                    + props.y;

                if let Some(l) = labeller {
                    svg_elements.push(html! {
                        <g class={classes.to_owned()}>
                            {l(x, y)}
                        </g>
                    });
                }

                top_y = top_y.min(y);
                element_points.push((data_x, data_y, x, y));

                last_data_step = step;
            }
            draw_chart(&element_points, props, &mut svg_elements, &classes);
        }

        DerivedProps { svg_elements }
    }
}

fn draw_chart(
    element_points: &[(f32, f32, f32, f32)],
    props: &Props,
    svg_elements: &mut Vec<VNode>,
    classes: &Classes,
) {
    #[cfg(feature = "custom-tooltip")]
    fn onmouseover(props: &Props, title: String) -> impl Fn(MouseEvent) {
        let cb = Rc::clone(&props.onmouseover);
        move |e| {
            (*cb).emit((e, title.clone()));
        }
    }

    match props.series_type {
        Type::Bar(bar_type) => {
            for point in element_points.iter() {
                let (data_x, data_y1, x, y1) = *point;

                let (y1, y2) = match bar_type {
                    BarType::Rise => (y1, props.height + props.y),
                    BarType::Drop => (props.y, y1),
                };

                if y1 != y2 {
                    #[cfg(feature = "custom-tooltip")]
                    let html = {
                        let title = if let Some(tt) = &props.tooltipper {
                            tt(data_x, data_y1)
                        } else {
                            String::default()
                        };
                        html! {
                            <line x1={x.to_string()} y1={y1.to_string()} x2={x.to_string()} y2={y2.to_string()}
                                class={classes!(classes.to_owned(), "bar-chart")}
                                onmouseover={onmouseover(props, title)}/>
                        }
                    };
                    #[cfg(not(feature = "custom-tooltip"))]
                    let html = html! {
                        <line x1={x.to_string()} y1={y1.to_string()} x2={x.to_string()} y2={y2.to_string()}
                            class={classes!(classes.to_owned(), "bar-chart")}>
                        {
                            if let Some(tt) = &props.tooltipper {
                                html! {
                                    <title>{tt(data_x, data_y1)}</title>
                                }
                            } else {
                                html!()
                            }
                        }
                        </line>
                    };

                    svg_elements.push(html);
                }
            }
        }
        Type::Line => {
            let mut last_point: Option<(f32, f32, f32, f32)> = None;
            for point in element_points.iter() {
                let (data_x2, data_y2, x2, y2) = *point;

                if let Some((data_x1, data_y1, x1, y1)) = last_point {
                    #[cfg(feature = "custom-tooltip")]
                    let html = {
                        let title = if let Some(tt) = &props.tooltipper {
                            format!("{}-{}", tt(data_x1, data_y1), tt(data_x2, data_y2))
                        } else {
                            String::default()
                        };
                        html! {
                            <line x1={x1.to_string()} y1={y1.to_string()} x2={x2.to_string()} y2={y2.to_string()} class={classes.to_owned()} fill="none"
                            onmouseover={onmouseover(props, title)} />
                        }
                    };
                    #[cfg(not(feature = "custom-tooltip"))]
                    let html = html! {
                        <line x1={x1.to_string()} y1={y1.to_string()} x2={x2.to_string()} y2={y2.to_string()} class={classes.to_owned()} fill="none">
                        {
                            if let Some(tt) = props.tooltipper.as_ref() {
                                html! {
                                    <title>{tt(data_x1, data_y1)}{"-"}{tt(data_x2, data_y2)}</title>
                                }
                            } else {
                                html!()
                            }
                        }
                        </line>
                    };

                    svg_elements.push(html);
                }

                last_point = Some(*point);
            }
        }
        Type::Scatter => (),
    }
}

impl Component for Series {
    type Message = Msg;

    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let on_resize = ctx.link().callback(|_: Event| Msg::Resize);
        Series {
            derived_props: Self::derive_props(ctx.props()),
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

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.derived_props = Self::derive_props(ctx.props());
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
            .and_then(|n| n.dyn_into::<SvgElement>().ok())
        {
            let width = svg_element.get_bounding_client_rect().width() as f32;
            let scale = p.width / width;
            let font_size = scale * 100f32;
            let _ = element.set_attribute("font-size", &format!("{}%", &font_size));
            let _ = element.set_attribute("style", &format!("stroke-width: {}", scale));
        }
    }
}
