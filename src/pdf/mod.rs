use core::num;
use std::cmp::max;

use krilla;
use tiny_skia;
use crate::figure::Figure;
use crate::figure::annotation::{HorizontalAlignment, VerticalAlignment};
use crate::paint::{Color, Dash, Stroke};
use crate::Config;
use crate::text::{default_font, string_to_lines, text_height, text_width};

pub struct Document {
    document: krilla::Document,
    //
    file_name: String,
}
impl Document {
    //
    pub fn new<T: Into<String>>(file_name: T) -> Self {
        Self {
            document: krilla::Document::new(),
            //
            file_name: file_name.into(),
        }
    }
    // Add a figure to the pdf (creates a new page with size determined by "fig")
    pub fn add_figure(&mut self, mut fig: Figure) {
        // Make sure all the settings are good (see "figure/mod.rs")
        fig.resolve_settings();

        // Add new page to the document
        let mut page = self.document.start_page_with(krilla::page::PageSettings::from_wh(fig.fig_size.0, fig.fig_size.1).unwrap());
        // Get Surface object to draw on
        let mut surface = page.surface();

        // Upscaling parameters for the rasterized axes. Doing this because pdf is vector based but file size is huge if there are
        // a lot of data points. Using tiny_skia and rasterizing the plot area basically makes the file size constant and much smaller
        // if there are a large number of data points. Upscaling the rasterized portion maintains much better output image quality tho
        // when zooming into the pdf
        let upscaling: f32 = 5.;
        // Rasterize the plot area
        let plot_area = fig.rasterize_plot(upscaling);
        // Move to the area to draw the image
        surface.push_transform(&krilla::geom::Transform::from_translate(fig.ax_position.0 - 0.5, fig.ax_position.1 - 0.5));
        surface.draw_image(plot_area.into(), krilla::geom::Size::from_wh(fig.ax_size.0+1., fig.ax_size.1+1.).unwrap());
        surface.pop();
        
        // Draw the labels at the major axis ticks
        draw_tick_labels(&mut surface, &fig);
        // Draw the axis labels
        draw_axis_labels(&mut surface, &fig);
        // Draw the title
        draw_title(&mut surface, &fig);
        // Draw the legend
        draw_legend(&mut surface, &fig);
        // Draw the text boxes
        draw_text_boxes(&mut surface, &fig);
        

        surface.finish();
        page.finish();

    }
    // Create the pdf
    pub fn publish(self) {
        let _ = std::fs::write(self.file_name, self.document.finish().unwrap());
    }
}



impl Figure {
    //
    pub(self) fn rasterize_plot(&mut self, upscaling: f32) -> krilla::image::Image {
        // Sizes for the pixmap
        let ax_width: f32 = self.ax_size.0 * upscaling;
        let ax_height: f32 = self.ax_size.1 * upscaling;
        let pixmap_width: u32 = ax_width as u32 + upscaling as u32;
        let pixmap_height: u32 = ax_height as u32 + upscaling as u32;
        // Start a pixmap to draw the axes with
        let mut pixmap = tiny_skia::Pixmap::new(pixmap_width, pixmap_height).unwrap();

        // Range of x and y values
        let (xmin, xmax) = self.xlim.unwrap();
        let (ymin, ymax) = self.ylim.unwrap();
        // Half of upscaling. used a lot here so might as well just calculate once
        let half_upscaling = 0.5*upscaling;
        // Scaling for axis coords to pixmap coords, also used a lot
        let x_scale = ax_width/(xmax-xmin);
        let y_scale = ax_height/(ymax-ymin);
        // Closures for inline conversion to pixmap coordinates
        let pixmap_x = |x: f32| (x - xmin)*x_scale + half_upscaling;
        let pixmap_y = |y: f32| (ymax - y)*y_scale + half_upscaling;

        // Layer-by-layer going to draw plot:
        //     Minor grid
        //     Major grid
        //     Major ticks + bounding box
        //     Plot data

        // tiny_skia variables. Allocate once here and changes settings as necessary in each section
        let mut paint = tiny_skia::Paint::default();
        paint.anti_alias = false;
        let mut stroke = tiny_skia::Stroke {
            width: upscaling,
            miter_limit: 1.,
            line_cap: tiny_skia::LineCap::Round,
            line_join: tiny_skia::LineJoin::Round,
            dash: None
        };

        // Minor grid
        if let Config::On((xminor, yminor)) = self.minor_spacing {
            // Set the color for the minor grid
            paint.set_color(Color::DEFAULT_MINOR.into());
            // PathBuilder for the gridlines
            let mut minor_gridlines = tiny_skia::PathBuilder::new();
            // Loop through the x values
            for i in ((xmin/xminor).floor() as i32)..=((xmax/xminor).ceil() as i32) {
                let x = pixmap_x(xminor * i as f32);
                minor_gridlines.move_to(x, 0.);
                minor_gridlines.line_to(x, pixmap_height as f32);
            }
            // Loop through the y values
            for i in ((ymin/yminor).floor() as i32)..=((ymax/yminor).ceil() as i32) {
                let y = pixmap_y(yminor * i as f32);
                minor_gridlines.move_to(0., y);
                minor_gridlines.line_to(pixmap_width as f32, y);
            }
            // Draw the minor grid
            pixmap.stroke_path(&minor_gridlines.finish().unwrap(), &paint, &stroke, tiny_skia::Transform::identity(), None);
        }

        // Major grid
        if let Config::On((xmajor, ymajor)) = self.major_spacing {
            // Set the color for the major grid
            paint.set_color(Color::DEFAULT_MAJOR.into());
            // PathBuilder for the gridlines
            let mut major_gridlines = tiny_skia::PathBuilder::new();
            // Loop through the x values
            for i in ((xmin/xmajor).floor() as i32)..=((xmax/xmajor).ceil() as i32) {
                let x = pixmap_x(xmajor * i as f32);
                major_gridlines.move_to(x, 0.);
                major_gridlines.line_to(x, pixmap_height as f32);
            }
            // Loop through the y values
            for i in ((ymin/ymajor).floor() as i32)..=((ymax/ymajor).ceil() as i32) {
                let y = pixmap_y(ymajor * i as f32);
                major_gridlines.move_to(0., y);
                major_gridlines.line_to(pixmap_width as f32, y);
            }
            // Draw the major grid
            pixmap.stroke_path(&major_gridlines.finish().unwrap(), &paint, &stroke, tiny_skia::Transform::identity(), None);
        }

        // Bounding box
        // Set the color for the axis bounds and ticks
        paint.set_color(Color::BLACK.into());
        // PathBuilder for the axes bounding box
        let mut bounding_box = tiny_skia::PathBuilder::new();
        // Axis bounding box
        bounding_box.push_rect(tiny_skia::Rect::from_xywh(half_upscaling, half_upscaling, ax_width, ax_height).unwrap());
        // Draw the bounding box
        pixmap.stroke_path(&bounding_box.finish().unwrap(), &paint, &stroke, tiny_skia::Transform::identity(), None);
        
        // Major ticks
        if let Config::On((xmajor, ymajor)) = self.major_ticks {
            // PathBuilder for the major grid ticks
            let mut ticks = tiny_skia::PathBuilder::new();
            // Loop through the x values
            for i in ((xmin/xmajor).floor() as i32)..=((xmax/xmajor).ceil() as i32) {
                let x = pixmap_x(xmajor * i as f32);
                ticks.move_to(x, ax_height + 0.5 * upscaling);
                ticks.line_to(x, ax_height - 4.5 * upscaling);
            }
            // Loop through the y values
            for i in ((ymin/ymajor).floor() as i32)..=((ymax/ymajor).ceil() as i32) {
                let y = pixmap_y(ymajor * i as f32);
                ticks.move_to(0.5 * upscaling, y);
                ticks.line_to(5.5 * upscaling, y);
            }
            // Draw the bounding box and major grid ticks
            pixmap.stroke_path(&ticks.finish().unwrap(), &paint, &stroke, tiny_skia::Transform::identity(), None);
        }

        // PLot the data
        paint.anti_alias = true;
        for series in &self.data {
            // Set the color for this series
            paint.set_color(series.stroke.color.clone().into());
            // Set the width and dash for this series
            stroke.width = series.stroke.width * upscaling;
            stroke.dash = match series.stroke.dash {
                Dash::Solid => None,
                Dash::Dashed => tiny_skia::StrokeDash::new(vec![10.*upscaling, 3.*upscaling], 0.),
                Dash::DashDot => tiny_skia::StrokeDash::new(vec![10.*upscaling, 3.*upscaling, 0., 3.*upscaling], 0.),
                Dash::Dotted => tiny_skia::StrokeDash::new(vec![0., 3.*upscaling], 0.),
            };
            // PathBuilder for this series
            let mut series_path = tiny_skia::PathBuilder::new();
            // Go to the first point
            series_path.move_to(pixmap_x(series.x[0]), pixmap_y(series.y[0]));
            // Loop through the rest of the points and line_to each
            for i in 1..series.x.len() { series_path.line_to(pixmap_x(series.x[i]), pixmap_y(series.y[i])); }
            // Draw the series
            pixmap.stroke_path(&series_path.finish().unwrap(), &paint, &stroke, tiny_skia::Transform::identity(), None);
        }

        // Encode the pixmap to png buffer (png avoids needing to demultiply the rgb values),
        // then read into a krilla::image::Image and return
        return krilla::image::Image::from_png(pixmap.encode_png().unwrap().into(), false).unwrap();
    } 

}


fn draw_tick_labels(surface: &mut krilla::surface::Surface, fig: &Figure) {
    // Get the axis limits
    let (xmin, xmax) = fig.xlim.unwrap();
    let (ymin, ymax) = fig.ylim.unwrap();
    // Drawing the labels at each of the axis ticks
    if let Config::On((xmajor, ymajor)) = fig.major_ticks {
        // Amount to offset the text from the axes
        let h_buffer: f32 = text_width("n", fig.tick_label_font_size);
        let v_buffer: f32 = text_height(fig.tick_label_font_size) + h_buffer;
        // Scaling for axis coordinates to pdf coordinates
        let x_scale = fig.ax_size.0/(xmax - xmin);
        let y_scale = fig.ax_size.1/(ymax - ymin);
        // Closure to convert to pdf coordinates
        let pdf_x = |x: f32| (x - xmin)*x_scale + fig.ax_position.0;
        let pdf_y = |y: f32| (ymax - y)*y_scale + fig.ax_position.1;
        // Set the color and fill
        surface.set_fill(Color::BLACK.into());
        surface.set_stroke(None);
        // x-axis
        for i in ((xmin/xmajor).ceil() as i32)..=((xmax/xmajor).floor() as i32) {
            // x coordinate of the tick mark
            let x = xmajor * i as f32;
            // Format the label using num_decimals
            let label = format!("{:.1$}", x, fig.num_decimals.0);
            // Draw the text
            surface.draw_text(
                krilla::geom::Point::from_xy(
                    pdf_x(x) - text_width(&label, fig.tick_label_font_size)/2.,
                    fig.ax_position.1 + fig.ax_size.1 + v_buffer
                ),
                default_font(),
                fig.tick_label_font_size,
                &label,
                false,
                krilla::text::TextDirection::LeftToRight
            );
        }
        // y-axis
        for i in ((ymin/ymajor).ceil() as i32)..=((ymax/ymajor).floor() as i32) {
            // y coordinate of the tick mark
            let y = ymajor * i as f32;
            // Format the label using num_decimals
            let label = format!("{:.1$}", y, fig.num_decimals.1);
            // Draw the text
            surface.draw_text(
                krilla::geom::Point::from_xy(
                    fig.ax_position.0 - text_width(&label, fig.tick_label_font_size) - h_buffer,
                    pdf_y(y) + text_height(fig.tick_label_font_size)/2.
                ),
                default_font(),
                fig.tick_label_font_size,
                &label,
                false,
                krilla::text::TextDirection::LeftToRight
            );
        }
    }
}


//
fn draw_axis_labels(surface: &mut krilla::surface::Surface, fig: &Figure) {
    // Get the axis limits
    let (xmin, xmax) = fig.xlim.unwrap();
    let (ymin, ymax) = fig.ylim.unwrap();
    // Write the axis labels
    // x-axis
    if let Some(label) = &fig.xlabel {
        // Width and height of the label
        let label_width = text_width(label, fig.axis_label_font_size);
        let label_height = text_height(fig.axis_label_font_size);
        // Location of the label, 0.05x figure height off of the bottom, center of the axis
        let x = fig.ax_position.0 + 0.5*fig.ax_size.0 - 0.5*label_width;
        let y = fig.ax_position.1 + fig.ax_size.1 + fig.tick_label_font_size + 2.*text_width("n", fig.tick_label_font_size) + label_height;

        // Set the color and fill
        surface.set_fill(Color::BLACK.into());
        surface.set_stroke(None);
        // Draw the text
        surface.draw_text(
            krilla::geom::Point::from_xy(x, y),
            default_font(),
            fig.axis_label_font_size,
            &label,
            false,
            krilla::text::TextDirection::LeftToRight
        );
    }
    // y-axis
    if let Some(label) = &fig.ylabel {
        // Width and height of the label
        let label_width = text_width(label, fig.axis_label_font_size);
        // Location of the label, 0.05x figure height off of the bottom, center of the axis
        let offset = match fig.major_ticks {
            Config::On((.., ymajor)) => text_width(&format!("{:.1$}n", (ymin/ymajor).ceil(), fig.num_decimals.1+2), fig.tick_label_font_size),
            _ => 0.
        };
        let x = fig.ax_position.0 - offset;
        let y = 0.5*fig.fig_size.1 + 0.5*label_width;

        // Set the color and fill
        surface.set_fill(Color::BLACK.into());
        surface.set_stroke(None);
        // Rotate the surface 90 deg
        surface.push_transform(&krilla::geom::Transform::from_rotate(-90.));
        // Draw the text
        surface.draw_text(
            krilla::geom::Point::from_xy(-y, x),
            default_font(),
            fig.axis_label_font_size,
            &label,
            false,
            krilla::text::TextDirection::LeftToRight
        );
        // Undo rotation
        surface.pop();
    }
}


fn draw_title(surface: &mut krilla::surface::Surface, fig: &Figure) {
    // Check if figure has a title
    if let Some(title) = &fig.title {
        // Split into lines
        let title: Vec<&str> = title.lines().collect();
        // If there is no title exit the function
        if title.len() == 0 {return}
        // Height of the title
        // The first line gets font height as height allowance, and the rest get 1.3*font_size
        let font_line_height = text_height(fig.title_font_size);
        let title_height = font_line_height + 1.3*fig.title_font_size*((title.len() - 1) as f32);
        
        // Set the stroke and fill
        surface.set_stroke(None);
        surface.set_fill(Color::BLACK.into());
        // y coordinate to draw the first line at
        let mut y = (fig.ax_position.1 - title_height)/2. + font_line_height;
        // Loop through the lines in title
        for line in title {
            // x coordinate to draw the line at
            let x = 0.5*(fig.fig_size.0 - text_width(&line, fig.title_font_size));
            // Write the line
            surface.draw_text(
                krilla::geom::Point::from_xy(x, y),
                default_font(),
                fig.title_font_size,
                &line,
                false,
                krilla::text::TextDirection::LeftToRight
            );
            // Increment y for the next line
            y += 1.3*fig.title_font_size;
        }
    }
}

fn draw_legend(surface: &mut krilla::surface::Surface, fig: &Figure) {
    //
    if let Config::On((left, top, right, bottom)) = fig.legend_ltrb {
        // Font size to use for rendering
        let mut legend_font_size: f32 = fig.legend_font_size;

        // Set the stroke and fill
        surface.set_stroke(Stroke::default().into());
        surface.set_fill(Color::WHITE.into());
        // Draw the bounding box
        let mut pb = krilla::geom::PathBuilder::new();
        pb.push_rect(krilla::geom::Rect::from_ltrb(left, top, right, bottom).unwrap());
        surface.draw_path(&pb.finish().unwrap());

        // Find the widest legend label
        let widest_label: f32 = {
            let mut max_width: f32 = 0.;
            for series in fig.data.iter() {
                // If series has a label
                if let Some(label) = &series.label {
                    // Pick what is largest between running max and the label of the current series
                    max_width = max_width.max(text_width(label, legend_font_size));
                }
            }
            max_width
        };
        // If padding is less than 5.0 we are going to adjust the font size so that padding is 5.0
        let padding: f32 = {
            // Calculated padding
            let mut padding = 0.5 * (right - left - 20.5 - widest_label);
            // If the padding is less than 5.0 going to set it to 5.0 and then change font size to match
            if padding < 5.0 {
                padding = 5.0;
                // String is constant so just need to apply scaling to get the max width down to what we want
                // (right - left -  30.5) is width of the label if padding is 5.0
                legend_font_size = legend_font_size * (right - left - 30.5) / widest_label;

            }
            padding
        };
        
        // Find the spacing and starting height for the legend entries based on the bounding box height
        let num_entries: usize = fig.data.iter().filter(|s| s.label.is_some()).count();
        let text_height = text_height(legend_font_size);
        let step: f32 = (bottom - top + text_height) / (num_entries as f32 + 1.);

        // Loop through the series
        let half_text_height = 0.5 * text_height;
        let x = left + padding;
        let mut y = top;
        for (i, series) in fig.data.iter().enumerate() {
            y += step;
            let mut pb = krilla::geom::PathBuilder::new();
            pb.move_to(x, y - half_text_height);
            pb.line_to(x + 18., y - half_text_height);

            //
            surface.set_stroke(series.stroke.clone().into());
            surface.set_fill(None);
            surface.draw_path(&pb.finish().unwrap());

            //
            surface.set_stroke(None);
            surface.set_fill(Color::BLACK.into());
            surface.draw_text(
                krilla::geom::Point::from_xy(x + 20.5, y), 
                default_font(), 
                legend_font_size, 
                &series.label.clone().unwrap(), 
                false, 
                krilla::text::TextDirection::LeftToRight
            );
        }

    }
}


fn draw_text_boxes(surface: &mut krilla::surface::Surface, fig: &Figure) {
    //
    for textbox in fig.text_boxes.iter() {
        // Set the stroke and fill for the box
        surface.set_stroke(textbox.box_stroke.clone().into());
        surface.set_fill(textbox.box_fill.clone().into());

        // Draw the bounding box
        let mut pb = krilla::geom::PathBuilder::new();
        let (l, t, r, b) = textbox.ltrb;
        pb.push_rect(krilla::geom::Rect::from_ltrb(l, t, r, b).unwrap());
        surface.draw_path(&pb.finish().unwrap());

        // Set the stroke and fill for the text
        surface.set_stroke(None);
        surface.set_fill(textbox.font_color.clone().into());

        // Split the text into lines
        let text_by_line = string_to_lines(&textbox.text, textbox.font_size, textbox.ltrb.2 - textbox.ltrb.0 - 2.*textbox.padding);
        // Spacing for each line
        let line_height = textbox.line_spacing*textbox.font_size;
        // Starting y coordinate
        let mut y = match textbox.vertical_alignment {
            // Aligned on top edge of the textbox
            VerticalAlignment::Top => textbox.ltrb.1 + textbox.padding,
            // Centered in text box
            VerticalAlignment::Middle => {
                // Height of the textbox
                let height: f32 = textbox.ltrb.3 - textbox.ltrb.1 - 2.*textbox.padding;
                // Space on top or bottom of text box (box height - paragraph height)/2
                let empty_space = 0.5*(height - line_height*text_by_line.iter().count() as f32);
                // Starting y for the text
                textbox.ltrb.1 + empty_space
            },
            // Aligned on bottom edge of text box
            VerticalAlignment::Bottom => {
                // Height of the textbox
                let height: f32 = textbox.ltrb.3 - textbox.ltrb.1 - 2.*textbox.padding;
                // Space on top or bottom of text box (box height - paragraph height - bottom padding)
                let empty_space = (height - line_height*text_by_line.iter().count() as f32 - textbox.padding);
                // Starting y for the text
                textbox.ltrb.1 + empty_space
            },
        };

        // Width of the textbox
        let width: f32 = textbox.ltrb.2 - textbox.ltrb.0;

        // Iterate through the lines
        for line in text_by_line {
            // increment y coordinate
            y += line_height;
            // x coordinate to start drawing the text at
            let x: f32 = match textbox.horizontal_alignment {
                // Aligned on left edge of textbox
                HorizontalAlignment::Left => textbox.ltrb.0 + textbox.padding,
                //
                HorizontalAlignment::Center => textbox.ltrb.0 + 0.5*width - 0.5*text_width(&line, textbox.font_size),
                //
                HorizontalAlignment::Right => textbox.ltrb.2 - textbox.padding - text_width(&line, textbox.font_size),
            };

            // Draw the text
            surface.draw_text(
                krilla::geom::Point::from_xy(x, y),
                default_font(),
                textbox.font_size,
                &line,
                false,
                krilla::text::TextDirection::LeftToRight
            );

        }


    }
}