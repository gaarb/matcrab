use matcrab::prelude::*;

fn main() {

    let mut document = Document::new("test.pdf");
    
    let mut fig: Figure = Figure::with_size(796., 612.);

    fig.xlim(0., 6.28);
    fig.ylim(-1., 1.);

    fig.xlabel("Time (ms)");
    fig.ylabel("Pressure, Voltage, Current");

    fig.title("Where's the freaking gabagool??");

    fig.ax_position(72., 72., 640.8, 558.);

    let x: Vec<f32> = (0..=628).map(|x| x as f32/100.).collect();
    let y: Vec<f32> = x.iter().map(|x| x.sin()).collect();
    let y2: Vec<f32> = x.iter().map(|x| (x + std::f32::consts::PI/3.).sin()).collect();
    let y3: Vec<f32> = x.iter().map(|x| (x + 2.*std::f32::consts::PI/3.).sin()).collect();

    plot!(fig, x, y, label="sin(x)");
    plot!(fig, x, y2, label="sin(x + pi/3)");
    plot!(fig, x, y3, label="sin(x + 2*pi/3)");

    fig.legend(Some((648., 72., 774., 198.)));

    document.add_figure(fig);

    document.publish();

}