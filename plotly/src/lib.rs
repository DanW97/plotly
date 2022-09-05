//! # Plotly.rs
//!
//! A plotting library for Rust powered by [Plotly.js](https://plot.ly/javascript/).
#![recursion_limit = "256"] // lets us use a large serde_json::json! macro for testing crate::layout::Axis
extern crate askama;
extern crate rand;
extern crate serde;

#[cfg(feature = "plotly_ndarray")]
pub mod ndarray;

#[cfg(feature = "wasm")]
pub mod bindings;

pub mod common;
pub mod scatter3d;
pub mod cone;

pub use crate::layout::Layout;
pub use crate::plot::ImageFormat;
pub use crate::plot::Plot;

pub use crate::bar::Bar;
pub use crate::box_plot::BoxPlot;
pub use crate::candlestick::Candlestick;
pub use crate::contour::Contour;
pub use crate::heat_map::HeatMap;
pub use crate::histogram::Histogram;
pub use crate::ohlc::Ohlc;
pub use crate::scatter::Scatter;
pub use crate::surface::Surface;
pub use crate::scatter3d::Scatter3D;
pub use crate::cone::Cone;
pub use crate::common::color::NamedColor;
pub use crate::common::color::Rgb;
pub use crate::common::color::Rgba;
pub mod configuration;
pub mod layout;
pub mod plot;
mod traces;

pub use common::color;
pub use configuration::Configuration;
pub use layout::Layout;
pub use plot::{ImageFormat, Plot, Trace};

// Bring the different trace types into the top-level scope
pub use traces::{
    Bar, BoxPlot, Candlestick, Contour, HeatMap, Histogram, Ohlc, Sankey, Scatter, Scatter3D,
    ScatterPolar, Surface,
};
// Also provide easy access to modules which contain additional trace-specific types
pub use traces::{box_plot, contour, histogram, sankey, surface};

#[cfg(feature = "plotly_ndarray")]
pub use crate::ndarray::ArrayTraces;

// Not public API.
#[doc(hidden)]
mod private;
