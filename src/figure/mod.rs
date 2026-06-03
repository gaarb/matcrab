use crate::text::{text_height, text_width};
use crate::{Config, ToF32};
use crate::paint::{Color, Dash, Stroke, StrokePalette};
use crate::annotation::{self, Annotation, AnnotationElement};

mod series;

use series::Series;
use annotation::TextBox;


enum LegendLocation {
    UpperRight,
    UpperLeft,
    LowerLeft,
    LowerRight,
    These{left: f32, top: f32, right: f32, bottom: f32}
}


pub struct Figure {
    // Fig settings
    pub(crate) fig_size: (f32, f32),

    // Text Settings
    pub(crate) tick_label_font_size: f32,
    pub(crate) axis_label_font_size: f32,
    pub(crate) title_font_size: f32,
    pub(crate) legend_font_size: f32,
    pub(crate) xlabel: Option<String>,
    pub(crate) ylabel: Option<String>,
    pub(crate) title: Option<String>,
    pub(crate) num_decimals: (usize, usize),

    // Axes settings
    pub(crate) ax_size: (f32, f32),
    pub(crate) ax_position: (f32, f32),
    pub(crate) xlim: Option<(f32, f32)>,
    pub(crate) ylim: Option<(f32, f32)>,
    pub(crate) major_spacing: Config<(f32, f32)>,
    pub(crate) major_ticks: Config<(f32, f32)>,
    pub(crate) minor_spacing: Config<(f32, f32)>,
    pub(crate) legend_ltrb: Config<(f32, f32, f32, f32)>,

    // Strokes for the data
    stroke_palette: StrokePalette,

    // The data
    pub(crate) data: Vec<Series>,

    // Annotations
    pub(crate) annotations: Vec<AnnotationElement>
}
impl Figure {
    // 
    pub fn with_size(fig_width: f32, fig_height: f32) -> Self {
        Self {
            // Fig settings
            fig_size: (fig_width, fig_height),

            // Text items
            tick_label_font_size: 9.,
            axis_label_font_size: 9.,
            title_font_size: 16.,
            legend_font_size: 9.,
            xlabel: None,
            ylabel: None,
            title: None,
            num_decimals: (0, 0),

            // Axes Settings
            ax_size: (fig_width*0.8, fig_height*0.8),
            ax_position: (fig_width*0.1, fig_height*0.1),
            xlim: None,
            ylim: None,
            major_spacing: Config::Pending,
            major_ticks: Config::Pending,
            minor_spacing: Config::Pending,
            legend_ltrb: Config::Off,

            //
            stroke_palette: StrokePalette::default(),

            // The data
            data: Vec::new(),

            // Annotations
            annotations: Vec::new(),
        }
    }

    // Set the axis limits
    pub fn xlim(&mut self, min: f32, max: f32) { self.xlim = Some( (min, max) ); }
    pub fn ylim(&mut self, min: f32, max: f32) { self.ylim = Some( (min, max) ); }

    // Set the axis labels
    pub fn xlabel<T: Into<String>>(&mut self, label: T) { self.xlabel = Some(label.into()); }
    pub fn ylabel<T: Into<String>>(&mut self, label: T) { self.ylabel = Some(label.into()); }

    // Set the title
    pub fn title<T: Into<String>>(&mut self, title: T) { self.title = Some(title.into()); }

    // Add a text box annotation

    // Set the axes position and size
    // (x, y) is position from the bottom left corner of the figure
    // (w, h) is the width and height of the axes
    pub fn ax_position(&mut self, l: f32, t: f32, r: f32, b: f32) {
        self.ax_size = (r - l, b - t);
        self.ax_position = (l, t);
    }

    pub fn legend(&mut self, position_ltrb: Option<(f32, f32, f32, f32)>) {
        //
        self.legend_ltrb = match position_ltrb {
            None => Config::Pending,
            Some((l, t, r, b)) => Config::On((l, t, r, b))
        };
    }

    // Set all the automatically calculated settings (grid spacing, limits, num display decimals)
    pub(crate) fn resolve_settings(&mut self) {
        // If xlim not decided on or off then do automatic selection
        if let None = self.xlim {
            // Go through the data and find the min and max x values
            let (mut min, mut max): (f32, f32) = (std::f32::MAX, std::f32::MIN);
            for series in &self.data {
                for x in &series.x {
                    if *x < min { min = *x; }
                    if *x > max { max = *x; }
                }
            }
            // Set the limits
            self.xlim = Some((min, max));
        }
        // If ylim not decided on or off then do automatic selection
        let nudge_y: bool; // We'll use this later to fine tune the y limits after the major grid is resolved
        if let None = self.ylim {
            // Going to readjust the y limits later based on the grid spacing
            nudge_y = true;
            // Go through the data and find the min and max y values
            let (mut min, mut max): (f32, f32) = (std::f32::MAX, std::f32::MIN);
            for series in &self.data {
                for y in &series.y {
                    if *y < min { min = *y; }
                    if *y > max { max = *y; }
                }
            }
            // Set the limits
            self.ylim = Some((min, max));
        } else { nudge_y = false; }

        // If major grid spacing not set to on or off then do automatic selection
        // We also do the 
        match (&self.major_spacing, &self.major_ticks) {
            // If both are off then we don't need to calculate anything, default value for ticks will be "true"
            (Config::Off, Config::Off) => {
                // If minor grid is waiting for automatic selection we will turn it off
                if let Config::Pending = self.minor_spacing { self.minor_spacing = Config::Off; }
            },
            // Otherwise we do need to make the calculation
            _ => {
                // x spacing
                let xrange = {
                    let (xmin, xmax) = self.xlim.unwrap();
                    xmax - xmin
                };
                // Calculate the step size
                // First the raw step size to have 10 divisions on the plot
                let raw_step = xrange/10.;
                // Magnitude of this raw step size
                let mag = 10_f32.powf(raw_step.log10().floor());
                // Residual to figure out which "nice" interval we are close to
                let residual = raw_step / mag;
                // Calculate the step size
                let x_step = {
                    if residual <= 1. { mag }
                    else if residual <= 2. { 2.*mag }
                    else if residual <= 5. { 5.*mag }
                    else if residual <= 10. {10.*mag }
                    else { panic!("Something has gone horribly wrong! (Figure::resolve_major, x step calc)") }
                };

                // y spacing
                let yrange = {
                    let (ymin, ymax) = self.ylim.unwrap();
                    ymax - ymin
                };
                // Calculate the step size
                // First the raw step size to have 10 divisions on the plot
                let raw_step = yrange/10.;
                // Magnitude of this raw step size
                let mag = 10_f32.powf(raw_step.log10().floor());
                // Residual to figure out which "nice" interval we are close to
                let residual = raw_step / mag;
                // Calculate the step size
                let y_step = {
                    if residual <= 1. { mag }
                    else if residual <= 2. { 2.*mag }
                    else if residual <= 5. { 5.*mag }
                    else if residual <= 10. {10.*mag }
                    else { panic!("Something has gone horribly wrong! (Figure::resolve_major, y step calc)") }
                };
                
                // Check each major_spacing and major_ticks and set whichever is not Config::Off
                if let Config::Pending = self.major_spacing { self.major_spacing = Config::On((x_step, y_step)); }
                if let Config::Pending = self.major_ticks   { self.major_ticks   = Config::On((x_step, y_step)); }
                // If minor grid is auto selection then set the spacing
                if let Config::Pending = self.minor_spacing { self.minor_spacing = Config::On((x_step/5., y_step/5.)); }
            }
        }

        // Figuring out how many decimals to display for the x and y labels
        // Max allowable decimals and max allowable error in the final display
        let threshold: f32 = 1e-5;
        let max_decimals: usize = 10;
        // Loop through the range of possible decimals
        let (xmin, xmax) = self.xlim.unwrap();
        let (xmajor, ymajor) = self.major_spacing.unwrap_clone();
        for n in 0..=max_decimals {
            let mut max_error: f32 = 0.;
            // Loop through the keys
            for i in ((xmin/xmajor).ceil() as i32)..=((xmax/xmajor).floor() as i32) {
                let value: f32 = (i as f32) * xmajor;
                // Scale by 10^n and check how far apart the rounded and unrounded values are
                let scaled = value * 10_f32.powi(n as i32);
                let err = (scaled - scaled.round()).abs();
                // If this is the max error this loop then overwrite
                if err > max_error { max_error = err; }
            }
            // If our max error less than threshold then this number of decimals is good
            if max_error < threshold {
                self.num_decimals.0 = n;
                break;
            } else {
                if n == max_decimals { self.num_decimals.0 = n; break; } else { continue; }
            }
        }
        // Loop through the range of possible decimals
        let (ymin, ymax) = self.ylim.unwrap();
        for n in 0..=max_decimals {
            let mut max_error: f32 = 0.;
            // Loop through the keys
            for i in ((ymin/ymajor).floor() as i32)..=((ymax/ymajor).ceil() as i32) {
                let value: f32 = (i as f32) * ymajor;
                // Scale by 10^n and check how far apart the rounded and unrounded values are
                let scaled = value * 10_f32.powi(n as i32);
                let err = (scaled - scaled.round()).abs();
                // If this is the max error this loop then overwrite
                if err > max_error { max_error = err; }
            }
            // If our max error less than threshold then this number of decimals is good
            if max_error < threshold {
                self.num_decimals.1 = n;
                break;
            } else {
                if n == max_decimals { self.num_decimals.1 = n; break; } else { continue; }
            }
        }
        
        // If we did auto y selection we're going to adjust it to have some padding based on the automatic grid
        // Do this by setting min and max to the lower and upper bounds from the grid spacing
        if nudge_y { self.ylim = Some(((ymin/ymajor).floor() * ymajor, (ymax/ymajor).ceil() * ymajor)) }

        // Legend settings
        if let Config::Pending = self.legend_ltrb {
            // Number of entries in the legend
            let mut num_legend_entries: usize = 0;
            // Check the widths of the labels and find the max
            let mut max_label_width: f32 = 0.;
            for series in &self.data {
                if let Some(label) = &series.label {
                    let width = text_width(label, self.legend_font_size);
                    max_label_width = max_label_width.max(width);
                    num_legend_entries += 1;
                }
            }
            // Width of the legend to fit the labels
            // Fixed width of 40.0 for the line, 5.0 for left margin, 5.0 for right margin, 2.5 for space between line and text
            let legend_width = 52.5 + max_label_width;
            // Height of the legend
            // 1.5*legend_font_size*num_legend_entries + gap between top of legend and first line (for symmetric padding on bottom)
            let legend_height = 1.5*self.legend_font_size*(num_legend_entries as f32 + 1.) - text_height(self.legend_font_size);

            // Check for the best location to place the legend
            // Upper-right
            // Upper-left
            // Lower-left
            // Lower-right
            // Set each bound, and check the total length of curve that lies below the legend
            // Select the location that obscures the least length of curve. If at any point one of them has 0 length obscured
            // we pick it and don't look for alternatives

            // UpperRight
            let mut upper_right_length_obscured: f32 = 0.;
            // The coordinates of the legend if UpperRight
            let right = self.ax_position.0 + self.ax_size.0 - 5.;
            let left = right - legend_width;
            let top = self.ax_position.1 + 5.;
            let bottom = top + legend_height;

            // 
            for series in &self.data {
                // Loop through each of the data points
                for (x, y) in series.x.iter().zip(series.y.iter()) {

                }
            }

            self.legend_ltrb = Config::On((left, top, right, bottom));

        }
    }

    pub fn add_series<'a, IX, IY, X, Y>(&mut self, x: IX, y: IY, color: Option<Color>, dash: Option<Dash>, width: Option<f32>, label: Config<String>)
    where
        IX: IntoIterator<Item = &'a X>,
        IY: IntoIterator<Item = &'a Y>,
        X: ToF32 + 'a,
        Y: ToF32 + 'a,
    {
        // Create vectors for x and y so we can verify the lengths match
        let x: Vec<f32> = x.into_iter().map(|x| x.to_f32()).collect();
        let y: Vec<f32> = y.into_iter().map(|y| y.to_f32()).collect();
        // Check if they match, panic if they don't
        if x.len() != y.len() { panic!("Unable to create Series in Figure. X and Y vectors are of different lengths."); }

        //
        // Interpret the inputs for color and dash
        let width: f32 = if let Some(width) = width { width } else { 1. };
        let stroke: Stroke = match (color, dash) {
            // Specified nothing, get the next in the list
            (None, None) => {
                let (color, dash) = self.stroke_palette.next();
                Stroke { color, dash, width }
            },
            // Specified both, get both
            (Some(color), Some(dash)) => Stroke { color, dash, width },
            // No dash specified, pick Dash::Solid
            (Some(color), None) => Stroke { color, dash: Dash::Solid, width },
            // Specified dash and no color (why?), take the next color in the list anyway
            (None, Some(dash)) => Stroke { color: self.stroke_palette.next().0, dash, width }
        };

        // Determine the label
        let label: Option<String> = match label {
            // No label return 'None'
            Config::Off => None,
            // Something specified, return Some(label)
            Config::On(label) => Some(label),
            // Nothing specified, do automatic
            Config::Pending => Some(format!("Series {}", self.data.len() + 1))
        };

        // Add the series to self.data
        self.data.push(Series { x, y, stroke, label });

    }

    // Add an annotation to the plot
    pub fn annotate(&mut self, annotation: &dyn Annotation) { for element in annotation.to_buffer() {self.annotations.push(element)}; }

}




#[macro_export]
macro_rules! plot {
    ($fig:expr, $x:expr, $y:expr $(, $($kwargs:tt)*)?) => {
        // Default input values for the series
        let mut color: Option<Color> = None;
        let mut dash: Option<Dash> = None;
        let mut width: Option<f32> = None;
        let mut series_label: Config<String> = Config::Pending;
        // Recursive calls for the kwargs
        plot!(@parse color, dash, width, series_label $(, $($kwargs)*)?);
        // Add the series to the figure
        $fig.add_series(&$x, &$y, color, dash, width, series_label);
    };

    // Final call once exhausted kwargs
    // Does nothing, stops recursion
    (@parse $color:ident, $dash:ident, $width:ident, $series_label:ident) => {};


    // Recursive calls to interpret kwargs

    // Label specified
    (@parse $color:ident, $dash:ident, $width:ident, $series_label:ident, label=$label:expr $(, $($kwargs:tt)*)?) => {
        // Set the label
        $series_label = Config::On($label.into());
        // Next call
        plot!(@parse $color, $dash, $width, $series_label $(, $($kwargs)*)?);
    };
    // Label off
    (@parse $color:ident, $dash:ident, $width:ident, $series_label:ident, label=none $(, $($kwargs:tt)*)?) => {
        // Set the label
        $series_label = Config::Off;
        // Next call
        plot!(@parse $color, $dash, $width, $series_label $(, $($kwargs)*)?);
    };

    // Color specified
    (@parse $color:ident, $dash:ident, $width:ident, $series_label:ident, color=($r:expr, $g:expr, $b:expr) $(, $($kwargs:tt)*)?) => {
        // Set the color
        $color = Some(Color::from_rgb($r, $g, $b));
        // Next call
        plot!(@parse $color, $dash, $width, $series_label $(, $($kwargs)*)?);
    };

    // Dash specified
    // Solid
    (@parse $color:ident, $dash:ident, $width:ident, $series_label:ident, dash="-" $(, $($kwargs:tt)*)?) => {
        // Set to solid
        $dash = Some(Dash::Solid);
        // Next call
        plot!(@parse $color, $dash, $width, $series_label $(, $($kwargs)*)?);
    };
    // Dashed
    (@parse $color:ident, $dash:ident, $width:ident, $series_label:ident, dash="--" $(, $($kwargs:tt)*)?) => {
        // Set to Dashed
        $dash = Some(Dash::Dashed);
        // Next call
        plot!(@parse $color, $dash, $width, $series_label $(, $($kwargs)*)?);
    };
    // DashDot
    (@parse $color:ident, $dash:ident, $width:ident, $series_label:ident, dash="-." $(, $($kwargs:tt)*)?) => {
        // Set to DashDot
        $dash = Some(Dash::DashDot);
        // Next call
        plot!(@parse $color, $dash, $width, $series_label $(, $($kwargs)*)?);
    };
    // Dotted
    (@parse $color:ident, $dash:ident, $width:ident, $series_label:ident, dash=".." $(, $($kwargs:tt)*)?) => {
        // Set to Dotted
        $dash = Some(Dash::Dotted);
        // Next call
        plot!(@parse $color, $dash, $width, $series_label $(, $($kwargs)*)?);
    };
    // Invalid input
    (@parse $color:ident, $dash:ident, $width:ident, $series_label:ident, dash=$invalid_input:tt $(, $($kwargs:tt)*)?) => {
        // Print compile error message
        compile_error!(concat!("Invalid dash type: ", stringify!($invalid_input), " in plot!() call. Valid dash inputs are:
            \"-\"  -> Solid
            \"--\" -> Dashed
            \"-.\" -> DashDot
            \"..\" -> Dotted"));
        // Next call
        plot!(@parse $color, $dash, $width, $series_label $(, $($kwargs)*)?);
    };
}