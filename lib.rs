// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(test)]

extern crate freetype as freetype_rs;
extern crate libc;

use libc::{c_double, c_uint, c_void};
use std::mem;
use std::ptr;

mod ffi;
mod freetype;

#[cfg(test)]
mod bench;

pub type Point = ffi::glyphy_point_t;

impl Point {
    pub fn new(x: c_double, y: c_double) -> Point {
        Point {
            x: x,
            y: y,
        }
    }
}

pub type Extents = ffi::glyphy_extents_t;

pub type Arc = ffi::glyphy_arc_t;

impl Arc {
    pub fn from_line(p0: &Point, p1: &Point) -> Arc {
        unsafe {
            let mut result = mem::uninitialized();
            ffi::glyphy_arc_from_line(p0, p1, &mut result);
            result
        }
    }

    pub fn from_conic(p0: &Point, p1: &Point, p2: &Point) -> (Arc, c_double) {
        unsafe {
            let mut result = mem::uninitialized();
            let mut error = 0.0;
            ffi::glyphy_arc_from_conic(p0, p1, p2, &mut result, &mut error);
            (result, error)
        }
    }

    pub fn from_cubic(p0: &Point, p1: &Point, p2: &Point, p3: &Point) -> (Arc, c_double) {
        unsafe {
            let mut result = mem::uninitialized();
            let mut error = 0.0;
            ffi::glyphy_arc_from_cubic(p0, p1, p2, p3, &mut result, &mut error);
            (result, error)
        }
    }
}

pub type ArcEndpoint = ffi::glyphy_arc_endpoint_t;

pub type ArcEndpointAccumulatorCallback = Box<FnMut(&mut ArcEndpoint) -> bool>;

extern "C" fn arc_endpoint_accumulator_callback(endpoint: *mut ffi::glyphy_arc_endpoint_t,
                                                mut user_data: *mut c_void)
                                                -> ffi::glyphy_bool_t {
    unsafe {
        let callback: &mut Box<ArcEndpointAccumulatorCallback> = mem::transmute(&mut user_data);
        let endpoint: &mut ArcEndpoint = mem::transmute(endpoint);
        let result = callback(endpoint);
        result as ffi::glyphy_bool_t
    }
}

pub struct ArcAccumulator {
    glyphy_arc_accumulator: *mut ffi::glyphy_arc_accumulator_t,
    callback: Option<Box<ArcEndpointAccumulatorCallback>>,
}

impl Drop for ArcAccumulator {
    fn drop(&mut self) {
        unsafe {
            assert!(!self.glyphy_arc_accumulator.is_null());
            ffi::glyphy_arc_accumulator_destroy(self.glyphy_arc_accumulator);
        }
    }
}

impl ArcAccumulator {
    pub fn new() -> ArcAccumulator {
        unsafe {
            let glyphy_arc_accumulator = ffi::glyphy_arc_accumulator_create();
            assert!(!glyphy_arc_accumulator.is_null());
            ArcAccumulator {
                glyphy_arc_accumulator: glyphy_arc_accumulator,
                callback: None,
            }
        }
    }

    pub fn reset(&self) {
        unsafe {
            ffi::glyphy_arc_accumulator_reset(self.glyphy_arc_accumulator)
        }
    }

    pub fn set_callback(&mut self, callback: ArcEndpointAccumulatorCallback) {
        unsafe {
            self.callback = Some(Box::new(callback));
            let user_data =
                mem::transmute::<&Box<ArcEndpointAccumulatorCallback>,*const *mut c_void>(
                    self.callback.as_ref().unwrap());
            ffi::glyphy_arc_accumulator_set_callback(self.glyphy_arc_accumulator,
                                                     arc_endpoint_accumulator_callback,
                                                     *user_data);
        }
    }

    pub fn move_to(&self, p0: &Point) {
        unsafe {
            ffi::glyphy_arc_accumulator_move_to(self.glyphy_arc_accumulator, p0)
        }
    }

    pub fn line_to(&self, p1: &Point) {
        unsafe {
            ffi::glyphy_arc_accumulator_line_to(self.glyphy_arc_accumulator, p1)
        }
    }

    pub fn conic_to(&self, p1: &Point, p2: &Point) {
        unsafe {
            ffi::glyphy_arc_accumulator_conic_to(self.glyphy_arc_accumulator, p1, p2)
        }
    }

    pub fn cubic_to(&self, p1: &Point, p2: &Point, p3: &Point) {
        unsafe {
            ffi::glyphy_arc_accumulator_cubic_to(self.glyphy_arc_accumulator, p1, p2, p3)
        }
    }

    pub fn arc_to(&self, p1: &Point, d: c_double) {
        unsafe {
            ffi::glyphy_arc_accumulator_arc_to(self.glyphy_arc_accumulator, p1, d)
        }
    }

    pub fn close_path(&self) {
        unsafe {
            ffi::glyphy_arc_accumulator_close_path(self.glyphy_arc_accumulator)
        }
    }
}

pub fn arc_list_extents(endpoints: &[ArcEndpoint]) -> Extents {
    unsafe {
        let mut extents = mem::uninitialized();
        ffi::glyphy_arc_list_extents(endpoints.as_ptr(), endpoints.len() as c_uint, &mut extents);
        extents
    }
}

pub fn sdf_from_arc_list(endpoints: &[ArcEndpoint], p: &Point, closest_p: Option<&mut Point>)
                         -> c_double {
    let closest_p: *mut ffi::glyphy_point_t = match closest_p {
        None => ptr::null_mut(),
        Some(closest_p) => closest_p,
    };
    unsafe {
        ffi::glyphy_sdf_from_arc_list(endpoints.as_ptr(), endpoints.len() as c_uint, p, closest_p)
    }
}

