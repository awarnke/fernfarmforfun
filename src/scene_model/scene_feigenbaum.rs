//! The module provides functions to render an icon to vector graphics.

use crate::animation_renderer::animation_renderer::AnimationRenderer;
use crate::geometry::path::DrawDirective;
use crate::geometry::path::Point;
use crate::geometry::path::Rect;

/// The function defines the draw directives for the reflections per facet
///
/// The first edge is completely covered by the result polygon
///
/// # Arguments
///
///
pub fn render_animation(anim_out: &mut dyn AnimationRenderer) -> () {
    let _take: f32 = 1.0;
    let _rest: f32 = 1.0;
    let _one_third: f32 = 1.0 / 3.0;
    let _two_thirds: f32 = 2.0 / 3.0;

    anim_out.begin_scene(Rect {
        left: 0.0,
        top: 0.0,
        width: 800.0,
        height: 600.0,
    });
    for b in 0..7
    {
        let mut path: [DrawDirective; 401] = [DrawDirective::Move(Point { x: 0.0, y: 600.0 });401];
        for i in 0..400
        {
            let x_param = i as f32;
            path[1+i] = DrawDirective::Line(Point { x: 2.0*x_param, y: 600.0-20.0*((x_param)%5.0) });
        }
        anim_out.begin_morph(&path);
        for i in 0..400
        {
            let x_param = i as f32;
            path[1+i] = DrawDirective::Line(Point { x: 2.0*x_param, y: 600.0-20.0*((x_param)%5.0) });
        }
        anim_out.add_morph_step(&path);
        anim_out.end_morph();
    }
    anim_out.end_scene();
}

/*
 * Copyright 2026-2026 Andreas Warnke
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *    http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
