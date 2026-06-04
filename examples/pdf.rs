use matcrab::prelude::*;

const LOGO: &[u8] = include_bytes!("../example_assets/logo.png");

fn main() {

    let mut document = Document::new("test.pdf");
    
    let mut fig: Figure = Figure::with_size(796., 612.);

    fig.xlim(0., 6.28);
    fig.ylim(-1., 1.);

    fig.xlabel("Time (ms)");
    fig.ylabel("Pressure, Voltage, Current");

    fig.title("Example Plot Title");

    fig.ax_position(72., 72., 640.8, 558.);

    let x: Vec<f32> = (0..=628).map(|x| x as f32/100.).collect();
    let y: Vec<f32> = x.iter().map(|x| x.sin()).collect();
    let y2: Vec<f32> = x.iter().map(|x| (x + std::f32::consts::PI / 2.).sin()).collect();
    let y3: Vec<f32> = x.iter().map(|x| (x + std::f32::consts::PI).sin()).collect();
    let y4: Vec<f32> = x.iter().map(|x| (x + std::f32::consts::PI * 3./2.).sin()).collect();

    plot!(fig, x, y, label="sin(x)", dash="-");
    plot!(fig, x, y2, label="sin(x + pi/2)", dash="--");
    plot!(fig, x, y3, label="sin(x + pi)", dash="-.");
    plot!(fig, x, y4, label="sin(x + 3*pi/2)", dash="..");

    fig.legend(Some((648., 72., 774., 198.)));

    let results_box = Rectangle {
        ltrb: (648., 205.2, 774., 349.2),
        ..Default::default()
    };

    let opening_response_label = Text {
        text: "Opening:".into(),
        font_size: 18.,
        x: 651.,
        y: 223.2,
        ..Default::default()
    };

    let opening_response = Text {
        text: "63.5 ms".into(),
        font_size: 24.,
        x: 651.,
        y: 252.,
        ..Default::default()
    };

    let logo = Image::from_png(LOGO.into(), 684.9825, 18., 774., 64.8);

    fig.annotate(&results_box);
    fig.annotate(&opening_response_label);
    fig.annotate(&opening_response);
    fig.annotate(&logo);

    document.add_figure(fig);

    document.publish();

}