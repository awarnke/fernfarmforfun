#![warn(missing_docs)]

//! This crate renders animated svg files for simulated scenes.

pub mod animation_renderer;
pub mod geometry;
pub mod scene_model;
use animation_renderer::animation_svg_writer;
use scene_model::scene_feigenbaum;
use scene_model::scene_leaf_of_fern;
use std::env;

/// The main function defines parameters and starts the icon_writer.
fn main() {
    let args: Vec<String> = env::args().collect();

    for argument in args {
        let arg = argument.as_str();
        if arg == "-f" {
            let mut svg_writer: animation_svg_writer::SvgWriter =
                animation_svg_writer::SvgWriter::new("out", "feigenbaum.svg");
            scene_feigenbaum::render_animation(&mut svg_writer);
            println!("Generated files have been written to '{}'.", "out");
        }
        if arg == "-l" {
            let mut svg_writer: animation_svg_writer::SvgWriter =
            animation_svg_writer::SvgWriter::new("out", "leaf_of_fern.svg");
            scene_leaf_of_fern::render_animation(&mut svg_writer);
            println!("Generated files have been written to '{}'.", "out");
        }

        if arg == "-h" {
            println!("options are");
            println!("-f animate feigenbaum");
            println!("-l animate leaf of fern");
        }
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
