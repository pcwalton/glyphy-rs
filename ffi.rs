// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_camel_case_types)]

use libc::{c_double, c_int, c_uint, c_void};

pub type glyphy_bool_t = c_int;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct glyphy_point_t {
    pub x: c_double,
    pub y: c_double,
}

#[repr(C)]
pub struct glyphy_extents_t {
    pub min_x: c_double,
    pub min_y: c_double,
    pub max_x: c_double,
    pub max_y: c_double,
}

#[repr(C)]
pub struct glyphy_arc_t {
    pub p0: glyphy_point_t,
    pub p1: glyphy_point_t,
    pub d: c_double,
}

#[link(name = "glyphy")]
extern "C" {
    pub fn glyphy_arc_from_line(p0: *const glyphy_point_t,
                                p1: *const glyphy_point_t,
                                arc: *mut glyphy_arc_t);
    pub fn glyphy_arc_from_conic(p0: *const glyphy_point_t,
                                 p1: *const glyphy_point_t,
                                 p2: *const glyphy_point_t,
                                 arc: *mut glyphy_arc_t,
                                 error: *mut c_double);
    pub fn glyphy_arc_from_cubic(p0: *const glyphy_point_t,
                                 p1: *const glyphy_point_t,
                                 p2: *const glyphy_point_t,
                                 p3: *const glyphy_point_t,
                                 arc: *mut glyphy_arc_t,
                                 error: *mut c_double);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct glyphy_arc_endpoint_t {
    pub p: glyphy_point_t,
    pub d: c_double,
}

pub type glyphy_arc_endpoint_accumulator_callback_t =
    extern "C" fn(endpoint: *mut glyphy_arc_endpoint_t, user_data: *mut c_void) -> glyphy_bool_t;

pub enum glyphy_arc_accumulator_t {}

#[link(name = "glyphy")]
extern "C" {
    pub fn glyphy_arc_accumulator_create() -> *mut glyphy_arc_accumulator_t;
    pub fn glyphy_arc_accumulator_destroy(acc: *mut glyphy_arc_accumulator_t);
    pub fn glyphy_arc_accumulator_reset(acc: *mut glyphy_arc_accumulator_t);
    pub fn glyphy_arc_accumulator_set_callback(
        acc: *mut glyphy_arc_accumulator_t,
        callback: glyphy_arc_endpoint_accumulator_callback_t,
        user_data: *mut c_void);
    pub fn glyphy_arc_accumulator_move_to(acc: *mut glyphy_arc_accumulator_t,
                                          p0: *const glyphy_point_t);
    pub fn glyphy_arc_accumulator_line_to(acc: *mut glyphy_arc_accumulator_t,
                                          p1: *const glyphy_point_t);
    pub fn glyphy_arc_accumulator_conic_to(acc: *mut glyphy_arc_accumulator_t,
                                           p1: *const glyphy_point_t,
                                           p2: *const glyphy_point_t);
    pub fn glyphy_arc_accumulator_cubic_to(acc: *mut glyphy_arc_accumulator_t,
                                           p1: *const glyphy_point_t,
                                           p2: *const glyphy_point_t,
                                           p3: *const glyphy_point_t);
    pub fn glyphy_arc_accumulator_arc_to(acc: *mut glyphy_arc_accumulator_t,
                                         p1: *const glyphy_point_t,
                                         d: c_double);
    pub fn glyphy_arc_accumulator_close_path(acc: *mut glyphy_arc_accumulator_t);
    pub fn glyphy_arc_list_extents(endpoints: *const glyphy_arc_endpoint_t,
                                   num_endpoints: c_uint,
                                   extents: *mut glyphy_extents_t);
}

#[link(name = "glyphy")]
extern "C" {
    pub fn glyphy_sdf_from_arc_list(endpoints: *const glyphy_arc_endpoint_t,
                                    num_endpoints: c_uint,
                                    p: *const glyphy_point_t,
                                    closest_p: *mut glyphy_point_t)
                                    -> c_double;
}

