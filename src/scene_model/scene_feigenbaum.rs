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
/// * `iter_max` - number of iterations after that the value shall be returned
/// * `system_param` - parameter describing growth and decay in the given system
///
fn feigenbaum(iter_max: i32, system_param: f32) -> f32 {
    let precise_b: f64 = system_param as f64;
    let mut iter_value: f64 = 0.5_f64;
    for _i in 1..iter_max {
        iter_value = precise_b * iter_value * (1.0_f64 - iter_value);
    }
    iter_value as f32
}

/// The function defines the draw directives for the reflections per facet
///
/// The first edge is completely covered by the result polygon
///
/// # Arguments
///
/// * `anim_out` - the writer that converts the animated scene to svg
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
    for b in 0..8 {
        let mut path: [DrawDirective; 401] = [DrawDirective::Move(Point { x: 0.0, y: 600.0 }); 401];
        for i in 0..400 {
            let x_param = i as f32;
            let f = feigenbaum(1 + b, x_param / 100.0);
            path[1 + i] = DrawDirective::Line(Point {
                x: 2.0 * x_param,
                y: 600.0 - 600.0 * f,
            });
        }
        anim_out.begin_morph(&path);
        for depth in 1..10 {
            for i in 0..400 {
                let x_param = i as f32;
                let f = feigenbaum(depth * 8 + b, x_param / 100.0);
                path[1 + i] = DrawDirective::Line(Point {
                    x: 2.0 * x_param,
                    y: 600.0 - 600.0 * f,
                });
            }
            anim_out.add_morph_step(&path);
        }
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
