use itertools_num::linspace;
use plotly::common::{
    ColorScale, ColorScalePalette, DashType, Fill, Font, Line, LineShape, Marker, Mode, Title, ColorBar,
};
use plotly::cone::Anchor;
use plotly::layout::{Axis, BarMode, Layout, Legend, TicksDirection};
use plotly::{Bar, NamedColor, Plot, Rgb, Rgba, Scatter, Cone};
use rand_distr::{Distribution, Normal, Uniform};

fn simple_cone_plot() {
    let x = vec![1.];
    let y = vec![1.];
    let z = vec![1.];
    let u = vec![1.];
    let v = vec![1.];
    let w = vec![0.];
    let trace = Cone::new(x,y,z,u,v,w).anchor(Anchor::Tip);
    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.use_local_plotly();
    plot.show();
}

fn simple_color_scatter() {
    let x = vec![0.];
    let y = vec![1.];
    let arr = vec![0.1];
    let trace = Scatter::new(x,y)
                        .mode(Mode::Markers)
                        .marker(Marker::new()
                                    .cauto(true)
                                    .color_scale(ColorScale::Palette(ColorScalePalette::Viridis))
                                    .color_array(arr)
                                    .color_bar(ColorBar::new()));
    let mut plot = Plot::new();
    plot.add_trace(trace);
    let x = vec![0.];
    let y = vec![1.3];
    let arr = vec![1.];
    let layout = Layout::new();
    let trace = Scatter::new(x,y)
                        .mode(Mode::Markers)
                        .marker(Marker::new()
                                    .cauto(true)
                                    .color_scale(ColorScale::Palette(ColorScalePalette::Viridis))
                                    .color_array(arr)
                                    .color_bar(ColorBar::new()));
    plot.add_trace(trace);
    plot.set_layout(layout);
    plot.use_local_plotly();
    plot.show();
}

fn main(){
    simple_color_scatter();
}