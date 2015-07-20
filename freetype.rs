// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ArcAccumulator;
use ArcEndpoint;
use Point;

use freetype_rs::Vector;
use freetype_rs::outline::{Curve, Outline};
use libc::c_double;
use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

const FREETYPE_POINT_SCALING_FACTOR: c_double = 64.0;

pub fn convert_outline_to_arcs(outline: &Outline) -> Vec<ArcEndpoint> {
    fn point(freetype_point: &Vector) -> Point {
        Point::new((freetype_point.x as c_double) / FREETYPE_POINT_SCALING_FACTOR,
                   (freetype_point.y as c_double) / FREETYPE_POINT_SCALING_FACTOR)
    }

    let mut accumulator = ArcAccumulator::new();
    let endpoints = Rc::new(RefCell::new(Vec::new()));
    let endpoints_for_callback = endpoints.clone();
    accumulator.set_callback(Box::new(move |endpoint| {
        endpoints_for_callback.borrow_mut().push(*endpoint);
        true
    }));

    for contour in outline.contours_iter() {
        accumulator.move_to(&point(&contour.start()));
        for curve in contour {
            match curve {
                Curve::Line(ref point0) => accumulator.line_to(&point(point0)),
                Curve::Bezier2(ref point0, ref point1) => {
                    accumulator.conic_to(&point(point0), &point(point1))
                }
                Curve::Bezier3(ref point0, ref point1, ref point2) => {
                    accumulator.cubic_to(&point(point0), &point(point1), &point(point2))
                }
            }
        }
        accumulator.close_path();
    }

    let mut endpoints = endpoints.borrow_mut();
    mem::replace(&mut *endpoints, Vec::new())
}

