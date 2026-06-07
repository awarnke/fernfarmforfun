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

fn stretch_scale(x: f32, max: f32) -> f32 {
    // scale
    const SCALED_LEFT: f32 = 0.25;
    const SCALED_BOUNDS: f32 = 0.5;
    let from_right = max - x;
    let scaled_from_right = from_right * SCALED_BOUNDS / max + SCALED_LEFT;
    //adjust
    let adjusted = scaled_from_right * scaled_from_right;
    let adjusted_min = SCALED_LEFT * SCALED_LEFT;
    let adjusted_max = (SCALED_LEFT + SCALED_BOUNDS) * (SCALED_LEFT + SCALED_BOUNDS);
    // convert back
    max - (adjusted - adjusted_min) / (adjusted_max - adjusted_min) * max
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
    let view_l: f32 = 20.0;
    let view_w: f32 = 800.0;
    let view_b: f32 = 610.0;
    let view_h: f32 = 600.0;
    let gap: f32 = 5.0;
    const STEPS: usize = 160;
    const X_RANGE: f32 = 4.0;

    anim_out.begin_scene(Rect {
        left: 0.0,
        top: 0.0,
        width: 840.0,
        height: 630.0,
    });

    let axis: [DrawDirective; 3] = [
        DrawDirective::Move(Point {
            x: view_l - gap,
            y: view_b - view_h,
        }),
        DrawDirective::Line(Point {
            x: view_l - gap,
            y: view_b + gap,
        }),
        DrawDirective::Line(Point {
            x: view_l + view_w,
            y: view_b + gap,
        }),
    ];
    anim_out.fix_path(&axis);

    for branch in 0..8 {
        let mut path: [DrawDirective; 1 + STEPS] = [DrawDirective::Move(Point {
            x: view_l,
            y: view_b,
        }); 1 + STEPS];

        // init path with first animation step
        let f0 = feigenbaum(1 + branch, 0.0);
        path[0] = DrawDirective::Move(Point {
            x: view_l,
            y: view_b - f0 * view_h,
        });
        for i in 1..=STEPS {
            let x_param = (i as f32) / (STEPS as f32) * X_RANGE;
            let x_adjusted = stretch_scale(x_param, X_RANGE);
            let f = feigenbaum(1 + branch, x_adjusted);
            path[i] = DrawDirective::Line(Point {
                x: view_l + x_adjusted / X_RANGE * view_w,
                y: view_b - f * view_h,
            });
        }
        anim_out.begin_morph(&path);

        // add further animation steps
        for depth in 1..10 {
            let f0 = feigenbaum(depth * 8 + branch, 0.0);
            path[0] = DrawDirective::Move(Point {
                x: view_l,
                y: view_b - f0 * view_h,
            });
            for i in 1..=STEPS {
                let x_param = (i as f32) / (STEPS as f32) * X_RANGE;
                let x_adjusted = stretch_scale(x_param, X_RANGE);
                let f = feigenbaum(depth * 8 + branch, x_adjusted);
                path[i] = DrawDirective::Line(Point {
                    x: view_l + x_adjusted / X_RANGE * view_w,
                    y: view_b - f * view_h,
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
