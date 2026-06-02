use matcrab::prelude::*;

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

    let results_box = TextBox {
        text: "Results can go in here\n\nCan also do comments and they can spill into multiple lines".into(),
        font_size: 12.,
        ltrb: (648., 205.2, 774., 349.2),
        vertical_alignment: VerticalAlignment::Middle,
        horizontal_alignment: HorizontalAlignment::Center,
        ..Default::default()
    };

    fig.add_textbox(results_box);

    document.add_figure(fig);

    document.publish();

}