//! The module provides functions to render an animation.

use crate::geometry::path::DrawDirective;
use crate::geometry::path::Rect;

/// Defines the data needed to render an icon
///
/// # Lifetimes
///
/// * `'my_lifespan` refers to the lifetime of `AnimationRenderer`
///
pub trait AnimationRenderer<'my_lifespan> {
    /// This function begins to define an animation path, it writes the header.
    ///
    /// # Arguments
    ///
    /// * `bounds` - bounding rectangle of the view box
    ///
    fn begin_scene(self: &mut Self, bounds: Rect) -> ();

    /// This function ends an animation path, it writes the footer.
    ///
    fn end_scene(self: &mut Self) -> ();

    /// This function defines a non-animated path.
    ///
    /// # Arguments
    ///
    /// * `d` - list of drawing directives
    fn fix_path(self: &mut Self, d: &[DrawDirective]) -> ();

    /// This function starts the definition of a morphing path.
    ///
    /// # Arguments
    ///
    /// * `d` - list of drawing directives
    fn begin_morph(self: &mut Self, d: &[DrawDirective]) -> ();

    /// This function adds a step to the definition of a morphing path.
    ///
    /// # Arguments
    ///
    /// * `d` - list of drawing directives
    fn add_morph_step(self: &mut Self, d: &[DrawDirective]) -> ();

    /// This function ends the definition of a morphing path.
    fn end_morph(self: &mut Self) -> ();
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
