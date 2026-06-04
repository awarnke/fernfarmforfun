//! The module provides functions to generate svg files.

use crate::animation_renderer::animation_renderer::AnimationRenderer;
use std::fs::File;
use std::path::PathBuf;
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
        let mut file_to_use = PathBuf::from(file_name);
        file_to_use.push(file_name);
        SvgWriter{ output_file: File::create(&file_to_use).unwrap() }
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
        write!(self.output_file, "</svg>\n").expect("Error at writing file");
    }
}

/// The VecRenderer struct provides some methods that implement a PathRenderer
impl<'my_lifespan> AnimationRenderer<'my_lifespan> for SvgWriter {
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
    fn define_animation_path(
        self: &mut Self,
        _x: f32,
        _y: f32
    ) -> () {
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
