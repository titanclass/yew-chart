use std::rc::Rc;

use yew::prelude::*;
use yew_chart::{
    axis::{Axis, Orientation, Scale},
    linear_axis_scale::LinearScale,
    series::{self, Labeller, Series, Type},
};

const WIDTH: f32 = 300.0;
const HEIGHT: f32 = 300.0;
const MARGIN: f32 = 50.0;
const TICK_LENGTH: f32 = 2.0;

#[function_component(App)]
fn app() -> Html {
    let labels = vec![
        "Sales",
        "Marketing",
        "Development",
        "Customer Support",
        "Information Technology",
        "Adminstration",
    ];
    let data_labels_60 = Rc::new(
        series::to_radial(vec![60.0; 6])
            .into_iter()
            .zip(labels)
            .map(|((x, y, _), label)| {
                (
                    x,
                    y,
                    Some(Rc::from(series::circle_text_label(label)) as Rc<dyn Labeller>),
                )
            })
            .flat_map(|d| [d.clone(), (0.0, 0.0, None), d])
            .collect(),
    );

    let data_labels_50 = Rc::new(series::to_radial(vec![50.0; 6]));
    let data_labels_40 = Rc::new(series::to_radial(vec![40.0; 6]));
    let data_labels_30 = Rc::new(series::to_radial(vec![30.0; 6]));
    let data_labels_20 = Rc::new(series::to_radial(vec![20.0; 6]));
    let data_labels_10 = Rc::new(series::to_radial(vec![10.0; 6]));

    let actuals = Rc::new(series::to_radial(vec![50.0, 45.0, 10.0, 10.0, 15.0, 14.0]));
    let budgets = Rc::new(series::to_radial(vec![42.0, 20.0, 60.0, 19.0, 23.0, 10.0]));

    let h_scale = Rc::new(LinearScale::new(-60.0..60.0, 10.0)) as Rc<dyn Scale<Scalar = _>>;
    let v_scale = Rc::new(LinearScale::new(-60.0..60.0, 10.0)) as Rc<dyn Scale<Scalar = _>>;

    let axis_scale = Rc::new(LinearScale::new(0.0..60.0, 10.0)) as Rc<dyn Scale<Scalar = _>>;

    html! {
            <svg class="chart" viewBox={format!("0 0 {} {}", WIDTH, HEIGHT)} preserveAspectRatio="none">
                <Series<f32, f32>
                    series_type={Type::Area}
                    name="line"
                    data={data_labels_60}
                    horizontal_scale={h_scale.clone()}
                    vertical_scale={v_scale.clone()}
                    x={MARGIN} y={MARGIN} width={WIDTH - (MARGIN * 2.0)} height={HEIGHT - (MARGIN * 2.0)} />

                <Series<f32, f32>
                    series_type={Type::Area}
                    name="line"
                    data={data_labels_50}
                    horizontal_scale={h_scale.clone()}
                    vertical_scale={v_scale.clone()}
                    x={MARGIN} y={MARGIN} width={WIDTH - (MARGIN * 2.0)} height={HEIGHT - (MARGIN * 2.0)} />

                <Series<f32, f32>
                    series_type={Type::Area}
                    name="line"
                    data={data_labels_40}
                    horizontal_scale={h_scale.clone()}
                    vertical_scale={v_scale.clone()}
                    x={MARGIN} y={MARGIN} width={WIDTH - (MARGIN * 2.0)} height={HEIGHT - (MARGIN * 2.0)} />

                <Series<f32, f32>
                    series_type={Type::Area}
                    name="line"
                    data={data_labels_30}
                    horizontal_scale={h_scale.clone()}
                    vertical_scale={v_scale.clone()}
                    x={MARGIN} y={MARGIN} width={WIDTH - (MARGIN * 2.0)} height={HEIGHT - (MARGIN * 2.0)} />

                <Series<f32, f32>
                    series_type={Type::Area}
                    name="line"
                    data={data_labels_20}
                    horizontal_scale={h_scale.clone()}
                    vertical_scale={v_scale.clone()}
                    x={MARGIN} y={MARGIN} width={WIDTH - (MARGIN * 2.0)} height={HEIGHT - (MARGIN * 2.0)} />

                <Series<f32, f32>
                    series_type={Type::Area}
                    name="line"
                    data={data_labels_10}
                    horizontal_scale={h_scale.clone()}
                    vertical_scale={v_scale.clone()}
                    x={MARGIN} y={MARGIN} width={WIDTH - (MARGIN * 2.0)} height={HEIGHT - (MARGIN * 2.0)} />

                <Series<f32, f32>
                    series_type={Type::Area}
                    name="budgets-series"
                    data={budgets}
                    horizontal_scale={h_scale.clone()}
                    vertical_scale={v_scale.clone()}
                    x={MARGIN} y={MARGIN} width={WIDTH - (MARGIN * 2.0)} height={HEIGHT - (MARGIN * 2.0)} />

                <Series<f32, f32>
                    series_type={Type::Area}
                    name="actuals-series"
                    data={actuals}
                    horizontal_scale={h_scale.clone()}
                    vertical_scale={v_scale.clone()}
                    x={MARGIN} y={MARGIN} width={WIDTH - (MARGIN * 2.0)} height={HEIGHT - (MARGIN * 2.0)} />

                <Axis<f32>
                    name="some-y-axis"
                    orientation={Orientation::Left}
                    scale={axis_scale}
                    x1={MARGIN + (WIDTH - (MARGIN * 2.0)) / 2.0} y1={MARGIN} xy2={(HEIGHT - (MARGIN * 2.0)) / 2.0 + MARGIN}
                    tick_len={TICK_LENGTH} />

            </svg>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
