use itertools_num::linspace;
use plotly::common::{
    ColorScale, ColorScalePalette, DashType, Fill, Font, Line, LineShape, Marker, Mode, Title,
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

fn main(){
    simple_cone_plot();
}