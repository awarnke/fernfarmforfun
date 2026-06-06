//! The module provides functions to generate svg files.

use crate::animation_renderer::animation_renderer::AnimationRenderer;
use std::fs::File;
use std::io::Write;

/// Defines a vector renderer
pub struct SvgWriter {
    /// The file that is open for writing
    pub output_file: File,
}

/// The VecRenderer struct provides some methods to write svg header and footer
impl<'my_lifespan> SvgWriter {
    /// The function header converts the vector graphics drawing directive header to svg format
    ///
    /// # Arguments
    ///
    /// * `file_name` - The file name
    ///
    /// # Panics
    ///
    /// This function panics if the vector graphics cannot be written to a file.
    ///
    pub fn new(file_name: &str) -> SvgWriter {
        SvgWriter {
            output_file: File::create(file_name).unwrap(),
        }
    }

    /// The function header converts the vector graphics drawing directive header to svg format
    ///
    /// # Arguments
    ///
    /// * `view` - The bounding box of the visible area
    ///
    /// # Panics
    ///
    /// This function panics if the vector graphics cannot be written to a file.
    ///
    pub(super) fn header(self: &mut Self) -> () {
        write!(
            self.output_file,
            "\
<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<svg
    width=\"100%\"
    height=\"100%\"
    viewBox=\"{} {} {} {}\"
    preserveAspectRatio=\"xMidYMid meet\"
    xmlns=\"http://www.w3.org/2000/svg\"
    version=\"1.1\"
    >
\
            ",
            0.0, 0.0, 800.0, 600.0
        )
        .expect("Error at writing file");
    }

    /// The function footer converts the vector graphics drawing directive footer to svg format
    ///
    /// # Panics
    ///
    /// This function panics if the vector graphics cannot be written to a file.
    ///
    pub(super) fn footer(self: &mut Self) -> () {
        write!(
        self.output_file,
        "\
    <path fill=\"#FF5722\" d=\"M 10 90 L 50 10 L 90 90 L 90 90 Z\">
        <animate
        attributeName=\"d\"
        dur=\"2s\"
        repeatCount=\"indefinite\"
        values=\"
        M 10 90 L 50 10 L 90 90 L 90 90  Z;
        M 10 10 L 90 10 L 90 90 L 10 90 Z;
        M 10 90 L 50 10 L 90 90 L 90 90 Z
        \"
        />
    </path>
    <rect xml:id=\"RectElement\" x=\"300\" y=\"100\" width=\"300\" height=\"100\"
        fill=\"none\" stroke=\"rgb(255,255,0)\"  >
        <animate attributeName=\"x\"
        begin=\"0s\" dur=\"9s\" fill=\"freeze\" from=\"300\" to=\"0\" />
        <animate attributeName=\"y\"
        begin=\"0s\" dur=\"9s\" fill=\"freeze\" from=\"100\" to=\"0\" />
        <animate attributeName=\"width\"
        begin=\"0s\" dur=\"9s\" fill=\"freeze\" from=\"300\" to=\"800\" />
        <animate attributeName=\"height\"
        begin=\"0s\" dur=\"9s\" fill=\"freeze\" from=\"100\" to=\"300\" />
    </rect>
\
        ",
        )
        .expect("Error at writing file");

        write!(self.output_file, "</svg>\n").expect("Error at writing file");
    }
}

/// The VecRenderer struct provides some methods that implement a PathRenderer
impl<'my_lifespan> AnimationRenderer<'my_lifespan> for SvgWriter {
    /// This function starts to define an animation path, it writes the header.
    ///
    fn start(self: &mut Self) -> () {
        self.header();
    }

    /// This function ends an animation path, it writes the footer.
    ///
    fn end(self: &mut Self) -> () {
        self.footer();
    }

    /// The function define_animation_path defines an animation path
    ///
    /// # Arguments
    ///
    /// * `x` - The segments of the path
    /// * `y` - The foreground color and width by which the path is stroked
    ///
    /// # Panics
    ///
    /// This function panics if the vector graphics cannot be written to a file.
    ///
    fn define_animation_path(self: &mut Self, _x: f32, _y: f32) -> () {
        write!(self.output_file, "    <path").expect("Error at writing file");
        write!(self.output_file, " stroke=\"none\"").expect("Error at writing file");
        write!(self.output_file, "\"\n    />\n").expect("Error at writing file");
    }
}

/*
Copyright 2026-2026 Andreas Warnke

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/
