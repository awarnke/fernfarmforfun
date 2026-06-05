//! The module provides functions to render an animation.

/// Defines the data needed to render an icon
///
/// # Lifetimes
///
/// * `'my_lifespan` refers to the lifetime of `AnimationRenderer`
///
pub trait AnimationRenderer<'my_lifespan> {
    /// This function starts to define an animation path, it writes the header.
    ///
    fn start(self: &mut Self) -> ();

    /// This function ends an animation path, it writes the footer.
    ///
    fn end(self: &mut Self) -> ();

    /// This function defines an animation path.
    ///
    /// # Arguments
    ///
    /// * `x` - x coordinate
    /// * `y` - y coordinate
    fn define_animation_path(self: &mut Self, x: f32, y: f32) -> ();
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
