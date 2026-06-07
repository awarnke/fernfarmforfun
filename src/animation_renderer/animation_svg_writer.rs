//! The module provides functions to generate svg files.

use crate::animation_renderer::animation_renderer::AnimationRenderer;
use crate::geometry::path::DrawDirective;
use crate::geometry::path::Rect;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::io::Write;

/// This is the rounding unit value for absolute coordinates.
/// This unit has an exact representation in both binary and decimal.
const ABS_UNIT: f32 = 0.125;

/// This is the rounding unit value for relative coordinates.
/// It has higher precision than ABS_UNIT to keep the sum of errors low.
/// This unit has an exact representation in both binary and decimal.
const REL_UNIT: f32 = 0.0625;

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
    /// * `folder_name` - The folder name
    /// * `file_name` - The file name
    ///
    /// # Panics
    ///
    /// This function panics if the vector graphics cannot be written to a file.
    ///
    pub fn new(folder_name: &str, file_name: &str) -> SvgWriter {
        fs::create_dir_all(folder_name).unwrap();
        let mut file_path = PathBuf::from(folder_name);
        file_path.push(file_name);
        SvgWriter {
            output_file: File::create(&file_path).unwrap(),
        }
    }

    /// The function writes a list of draw direcrtives
    ///
    /// # Arguments
    ///
    /// * `file_name` - The file name
    ///
    /// # Panics
    ///
    /// This function panics if the vector graphics cannot be written to a file.
    ///
    fn write_path(self: &mut Self, d: &[DrawDirective]) -> () {
        for seg in d {
            match seg {
                DrawDirective::Move(target) => {
                    write!(
                        self.output_file,
                        "M {},{} ",
                        target.round_x(ABS_UNIT),
                        target.round_y(ABS_UNIT)
                    )
                }
                DrawDirective::MoveRel(offset) => {
                    write!(
                        self.output_file,
                        "m {},{} ",
                        offset.round_dx(REL_UNIT),
                        offset.round_dy(REL_UNIT)
                    )
                }
                DrawDirective::Line(target) => {
                    write!(
                        self.output_file,
                        "L {},{} ",
                        target.round_x(ABS_UNIT),
                        target.round_y(ABS_UNIT)
                    )
                }
                DrawDirective::LineRel(offset) => {
                    write!(
                        self.output_file,
                        "l {},{} ",
                        offset.round_dx(REL_UNIT),
                        offset.round_dy(REL_UNIT)
                    )
                }
                DrawDirective::Continue(target) => {
                    write!(
                        self.output_file,
                        "{},{} ",
                        target.round_x(ABS_UNIT),
                        target.round_y(ABS_UNIT)
                    )
                }
                DrawDirective::ContinueRel(offset) => {
                    write!(
                        self.output_file,
                        "{},{} ",
                        offset.round_dx(REL_UNIT),
                        offset.round_dy(REL_UNIT)
                    )
                }
                DrawDirective::Curve(p1, p2, target) => {
                    write!(
                        self.output_file,
                        "C {},{} {},{} {},{} ",
                        p1.round_x(ABS_UNIT),
                        p1.round_y(ABS_UNIT),
                        p2.round_x(ABS_UNIT),
                        p2.round_y(ABS_UNIT),
                        target.round_x(ABS_UNIT),
                        target.round_y(ABS_UNIT)
                    )
                }
                DrawDirective::CurveRel(o_p1, o_p2, offset) => {
                    write!(
                        self.output_file,
                        "c {},{} {},{} {},{} ",
                        o_p1.round_dx(REL_UNIT),
                        o_p1.round_dy(REL_UNIT),
                        o_p2.round_dx(REL_UNIT),
                        o_p2.round_dy(REL_UNIT),
                        offset.round_dx(REL_UNIT),
                        offset.round_dy(REL_UNIT)
                    )
                }
                DrawDirective::Symmetric(p2, target) => {
                    write!(
                        self.output_file,
                        "S {},{} {},{} ",
                        p2.round_x(ABS_UNIT),
                        p2.round_y(ABS_UNIT),
                        target.round_x(ABS_UNIT),
                        target.round_y(ABS_UNIT)
                    )
                }
                DrawDirective::SymmetricRel(o_p2, offset) => {
                    write!(
                        self.output_file,
                        "s {},{} {},{} ",
                        o_p2.round_dx(REL_UNIT),
                        o_p2.round_dy(REL_UNIT),
                        offset.round_dx(REL_UNIT),
                        offset.round_dy(REL_UNIT)
                    )
                }
                DrawDirective::Close => {
                    write!(self.output_file, "Z ")
                }
                DrawDirective::CloseRel => {
                    write!(self.output_file, "z ")
                }
            }
            .expect("Error at writing file");
        }
    }
}

/// The VecRenderer struct provides some methods that implement a PathRenderer
impl<'my_lifespan> AnimationRenderer<'my_lifespan> for SvgWriter {
    /// This function begins to define an animation path, it writes the header.
    ///
    /// # Panics
    ///
    /// This function panics if the vector graphics cannot be written to a file.
    ///
    fn begin_scene(self: &mut Self, bounds: Rect) -> () {
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
            bounds.left, bounds.top, bounds.width, bounds.height
        )
        .expect("Error at writing file");
    }

    /// This function ends an animation path, it writes the footer.
    ///
    /// # Panics
    ///
    /// This function panics if the vector graphics cannot be written to a file.
    ///
    fn end_scene(self: &mut Self) -> () {
        write!(self.output_file, "</svg>\n").expect("Error at writing file");
    }

    /// This function defines a non-animated path.
    ///
    /// # Arguments
    ///
    /// * `d` - list of drawing directives
    ///
    /// # Panics
    ///
    /// This function panics if the vector graphics cannot be written to a file.
    ///
    fn fix_path(self: &mut Self, d: &[DrawDirective]) -> () {
        write!(
            self.output_file,
            "\
<path stroke=\"#222222\" fill=\"none\" d=\"
\
",
        )
        .expect("Error at writing file");

        self.write_path(d);

        write!(
            self.output_file,
            "\
\" />
\
            ",
        )
        .expect("Error at writing file");
    }

    /// This function starts the definition of a morphing path.
    ///
    /// # Arguments
    ///
    /// * `d` - list of drawing directives
    ///
    /// # Panics
    ///
    /// This function panics if the vector graphics cannot be written to a file.
    ///
    fn begin_morph(self: &mut Self, d: &[DrawDirective]) -> () {
        write!(
            self.output_file,
            "\
<path stroke=\"#222222\" fill=\"none\" d=\"
\
            ",
        )
        .expect("Error at writing file");

        self.write_path(d);

        write!(
            self.output_file,
            "\
\">
<animate
attributeName=\"d\"
dur=\"10s\"
repeatCount=\"indefinite\"
values=\"
\
            ",
        )
        .expect("Error at writing file");

        self.write_path(d);
    }

    /// This function adds a step to the definition of a morphing path.
    ///
    /// # Arguments
    ///
    /// * `d` - list of drawing directives
    ///
    /// # Panics
    ///
    /// This function panics if the vector graphics cannot be written to a file.
    ///
    /// # Panics
    ///
    /// This function panics if the vector graphics cannot be written to a file.
    ///
    fn add_morph_step(self: &mut Self, d: &[DrawDirective]) -> () {
        write!(
            self.output_file,
            "\
;
\
",
        )
        .expect("Error at writing file");

        self.write_path(d);
    }

    /// This function ends the definition of a morphing path.
    ///
    /// # Panics
    ///
    /// This function panics if the vector graphics cannot be written to a file.
    ///
    fn end_morph(self: &mut Self) -> () {
        write!(
            self.output_file,
            "\
\"
/>
</path>
\
            ",
        )
        .expect("Error at writing file");
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
