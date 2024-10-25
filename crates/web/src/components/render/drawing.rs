use leptos::html::Canvas;
use leptos::*;
use scamper_rs::modules::image::{Align, Drawing, Mode};
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;

fn render(x: f64, y: f64, drawing: &Drawing, canvas: &leptos::HtmlElement<Canvas>) {
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    match drawing {
        Drawing::Ellipse(ellipse) => {
            let color_str = ellipse.color.to_string();
            context.set_fill_style_str(&color_str);
            context.set_stroke_style_str(&color_str);

            // Calculate dimensions
            let radius_x = ellipse.width / 2.0;
            let radius_y = ellipse.height / 2.0;
            let center_x = x + radius_x;
            let center_y = y + radius_y;

            // Draw ellipse
            context.begin_path();
            context
                .ellipse(
                    center_x,
                    center_y,
                    radius_x,
                    radius_y,
                    0.0,
                    0.0,
                    2.0 * std::f64::consts::PI,
                )
                .unwrap();

            // Fill or stroke based on mode
            match ellipse.mode {
                Mode::Solid => context.fill(),
                Mode::Outline => context.stroke(),
            }
        }
        Drawing::Rectangle(rectangle) => {
            let color_str = rectangle.color.to_string();
            context.set_fill_style_str(&color_str);
            context.set_stroke_style_str(&color_str);

            // Fill or stroke based on mode
            match rectangle.mode {
                Mode::Solid => {
                    context.fill_rect(x, y, rectangle.width, rectangle.height);
                }
                Mode::Outline => {
                    context.stroke_rect(x, y, rectangle.width, rectangle.height);
                }
            }
        }
        Drawing::Triangle(triangle) => {
            let color_str = triangle.color.to_string();
            context.set_fill_style_str(&color_str);
            context.set_stroke_style_str(&color_str);

            context.begin_path();
            context.move_to(x, y + triangle.height);
            context.line_to(x + triangle.width / 2.0, y);
            context.line_to(x + triangle.width, y + triangle.height);
            context.line_to(x, y + triangle.height);

            // Fill or stroke based on mode
            match triangle.mode {
                Mode::Solid => context.fill(),
                Mode::Outline => context.stroke(),
            }
        }
        Drawing::Path(path) => {
            if path.points.is_empty() {
                return;
            }
            let color_str = path.color.to_string();
            context.set_fill_style_str(&color_str);
            context.set_stroke_style_str(&color_str);

            context.begin_path();
            context.move_to(x + path.points[0].0, y + path.points[0].1);
            for (px, py) in path.points.iter().skip(1) {
                context.line_to(x + px, y + py);
            }

            match path.mode {
                Mode::Solid => context.fill(),
                Mode::Outline => context.stroke(),
            }
        }
        Drawing::Beside(beside) => {
            let mut x_offset = 0.0;
            for drawing in &beside.drawings {
                render(
                    x + x_offset,
                    match beside.align {
                        Align::Top => y,
                        Align::Bottom => y + beside.height - drawing.height(),
                        _ => y + (beside.height - drawing.height()) / 2.0,
                    },
                    drawing,
                    canvas,
                );
                x_offset += drawing.width();
            }
        }
        Drawing::Above(above) => {
            let mut y_offset = 0.0;
            for drawing in &above.drawings {
                render(
                    match above.align {
                        Align::Left => x,
                        Align::Right => x + above.width - drawing.width(),
                        _ => x + (above.width - drawing.width()) / 2.0,
                    },
                    y + y_offset,
                    drawing,
                    canvas,
                );
                y_offset += drawing.height();
            }
        }
    }
}

#[component]
pub fn DrawingView(drawing: Drawing) -> impl IntoView {
    let canvas_ref = create_node_ref::<Canvas>();

    canvas_ref.on_load(move |canvas_ref| {
        let _ = canvas_ref.on_mount(move |canvas| {
            // Set canvas dimensions
            canvas.set_width(drawing.width() as u32);
            canvas.set_height(drawing.height() as u32);

            // Get context
            let context = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();

            context.set_fill_style_str("white");
            context.set_stroke_style_str("black");
            // context.clear_rect(0.0, 0.0, drawing.width(), drawing.height());

            render(0.0, 0.0, &drawing, &canvas);
        });
    });

    view! {
        <canvas _ref=canvas_ref></canvas>
    }
}
