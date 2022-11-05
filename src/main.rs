#![allow(non_snake_case)]

mod utils;

use utils::data;

// use plotters::prelude::*;

fn main() {
    let (X, y) = data::spiral(100_000, 100);

    // plotting
    // let root_area = BitMapBackend::new("images/scatter1.png", (600, 400))
    //     .into_drawing_area();
    // root_area.fill(&WHITE).unwrap();

    // let mut ctx = ChartBuilder::on(&root_area)
    //     .set_label_area_size(LabelAreaPosition::Left, 40)
    //     .set_label_area_size(LabelAreaPosition::Bottom, 40)
    //     .caption("Spiral Data sample 1", ("sans-serif", 40))
    //     .build_cartesian_2d(-1.0..1.0, -1.0..1.0)
    //     .unwrap();

    // ctx.configure_mesh().draw().unwrap();
    // let colors = [RED, GREEN, BLUE];
    // for (data, color) in X.chunks(100).zip(colors.iter()) {
    //     ctx.draw_series(
    //         data.iter().map(
    //             |point| Circle::new(
    //                 *point,
    //                 2,
    //                 ShapeStyle {
    //                     color: color.to_rgba(),
    //                     filled: true,
    //                     stroke_width: 1,
    //                 }
    //             )
    //         )
    //     ).unwrap();
    // }
}
