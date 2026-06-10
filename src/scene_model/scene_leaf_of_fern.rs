//! The module provides functions to render an icon to vector graphics.

use crate::animation_renderer::animation_renderer::AnimationRenderer;
use crate::geometry::path::DrawDirective;
use crate::geometry::path::Point;
use crate::geometry::path::Rect;

/// The function defines the draw directives for the leaf of fern fractal
///
/// # Arguments
///
/// * `start` - point where to start
/// * `angle` - the angle of the main trunk
/// * `bend_left` - flag if the main trunk shall bend left or right
/// * `tension` - the additional tension on the trunk and branches
/// * `seg_len` - length of next segment
/// * `out_segs` - points to be filled in
///
fn leaf_of_fern(
    start: &Point,
    angle: f32,
    bend_left: bool,
    tension: f32,
    seg_len: f32,
    out_segs: &mut [DrawDirective],
) -> () {
    let out_len: usize = out_segs.len();
    let reserve: usize = if out_len >= 6 { 4 } else { 0 };
    let left_space: usize = out_len / 8 + reserve / 2;
    let right_space: usize = out_len / 7 + reserve / 2;
    let fwd_space: usize = out_len - left_space - right_space;

    if fwd_space >= 1 {
        /* determine fwd point */
        let fwd: Point = {
            Point {
                x: start.x - seg_len * (angle + tension).sin(),
                y: start.y - seg_len * (angle + tension).cos(),
            }
        };
        out_segs[0] = DrawDirective::Line(fwd);
        /* determine up angle*/
        let bend: f32 = if bend_left { 0.02 } else { -0.02 };
        /* recursion */
        leaf_of_fern(
            &fwd,
            angle + bend + tension,
            bend_left,
            tension,
            0.92 * seg_len,
            &mut out_segs[1..fwd_space],
        );
    }
    if left_space >= 1 {
        /* determine left point */
        let seg_ninety_len = seg_len * 0.9;
        let left: Point = {
            Point {
                x: start.x - seg_ninety_len * (angle + tension).sin(),
                y: start.y - seg_ninety_len * (angle + tension).cos(),
            }
        };
        out_segs[fwd_space] = DrawDirective::Move(left);
        /* recursion */
        leaf_of_fern(
            &left,
            angle + 1.0,
            !bend_left,
            tension + tension,
            0.3 * seg_len,
            &mut out_segs[fwd_space + 1..fwd_space + left_space],
        );
    }
    if right_space >= 1 {
        /* determine left point */
        let seg_sixty_len = seg_len * 0.6;
        let right: Point = {
            Point {
                x: start.x - seg_sixty_len * (angle + tension).sin(),
                y: start.y - seg_sixty_len * (angle + tension).cos(),
            }
        };
        out_segs[fwd_space + left_space] = DrawDirective::Move(right);
        /* recursion */
        leaf_of_fern(
            &right,
            angle - 1.0,
            bend_left,
            tension + tension,
            0.4 * seg_len,
            &mut out_segs[fwd_space + left_space + 1..out_len],
        );
    }
}

/// The function simulates the fern leaf and renders it to an animation
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
    const SEGMENTS: usize = 5000;

    anim_out.begin_scene(Rect {
        left: 0.0,
        top: 0.0,
        width: 840.0,
        height: 630.0,
    });

    let soil: [DrawDirective; 2] = [
        DrawDirective::Move(Point {
            x: view_l,
            y: view_b + gap,
        }),
        DrawDirective::Line(Point {
            x: view_l + view_w,
            y: view_b + gap,
        }),
    ];
    anim_out.fix_path(&soil);

    let mut path: [DrawDirective; SEGMENTS] = [DrawDirective::Move(Point {
        x: view_l + 0.5 * view_w,
        y: view_b,
    }); SEGMENTS];

    // first animation step
    let root: Point = {
        Point {
            x: view_l + 0.5 * view_w,
            y: view_b,
        }
    };
    path[0] = DrawDirective::Move(root);
    leaf_of_fern(
        &root,
        -0.15,
        true,
        -0.02,
        0.1 * view_h,
        &mut path[1..SEGMENTS],
    );
    anim_out.begin_morph(&path);

    // add further animation steps
    let tension_steps: [f32; 6] = [-0.01, 0.01, 0.02, 0.01, -0.01, -0.02];
    for step in 0..6 {
        let tension: f32 = tension_steps[step];
        path[0] = DrawDirective::Move(root);
        leaf_of_fern(
            &root,
            -0.15,
            true,
            tension,
            0.1 * view_h,
            &mut path[1..SEGMENTS],
        );
        anim_out.add_morph_step(&path);
    }
    anim_out.end_morph();

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
