// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate test;

use Point;
use freetype;
use sdf_from_arc_list;

use freetype_rs::library::Library;
use freetype_rs::face::{Face, NO_SCALE, RENDER};
use self::test::Bencher;
use std::iter;

const GLYPH_BUFFER_SIZE_RATIO: f64 = 0.3;

static FONT_PATH: &'static str = "/Library/Fonts/Arial.ttf";

#[bench]
fn small_font_arc_conversion(bencher: &mut Bencher) {
    benchmark_font_arc_conversion_at_size(bencher, FONT_PATH.to_owned(), 12, 'S');
}

#[bench]
fn small_font_distance_field_creation(bencher: &mut Bencher) {
    benchmark_font_distance_field_creation_at_size(bencher, FONT_PATH.to_owned(), 12, 'S');
}

#[bench]
fn small_traditional_font_rasterization(bencher: &mut Bencher) {
    benchmark_traditional_font_rasterization_at_size(bencher, FONT_PATH.to_owned(), 12, 'S');
}

fn create_face(freetype_library: &Library, font_path: String, font_size: i32) -> Face {
    let face = freetype_library.new_face(font_path, 0).unwrap();
    face.set_char_size(font_size as isize * 64, 0, 50, 0).unwrap();
    face
}

fn benchmark_font_arc_conversion_at_size(bencher: &mut Bencher,
                                         font_path: String,
                                         font_size: i32,
                                         character: char) {
    let freetype_library = Library::init().unwrap();
    let face = create_face(&freetype_library, font_path, font_size);
    face.load_char(character as usize, NO_SCALE).unwrap();
    let glyph = face.glyph();
    let outline = glyph.outline().expect(&*format!("No outline for glyph: '{}'!", character));

    bencher.iter(|| freetype::convert_outline_to_arcs(&outline));
}

fn benchmark_font_distance_field_creation_at_size(bencher: &mut Bencher,
                                                  font_path: String,
                                                  font_size: i32,
                                                  character: char) {
    let freetype_library = Library::init().unwrap();
    let face = create_face(&freetype_library, font_path, font_size);
    face.load_char(character as usize, NO_SCALE).unwrap();
    let glyph = face.glyph();
    let outline = glyph.outline().expect(&*format!("No outline for glyph: '{}'!", character));
    let arcs = freetype::convert_outline_to_arcs(&outline);

    let glyph_width = (glyph.metrics().width as u32) / 64;
    let glyph_height = (font_size - glyph.bitmap_top()) as u32;
    let distance_field_size = ((((glyph_width as f64) * GLYPH_BUFFER_SIZE_RATIO)) as usize,
                                (((glyph_height as f64) * GLYPH_BUFFER_SIZE_RATIO)) as usize);
    let mut result: Vec<u8> =
        iter::repeat(0).take(distance_field_size.0 * distance_field_size.1).collect();
    bencher.iter(|| {
        for y in 0..distance_field_size.1 {
            for x in 0..distance_field_size.0 {
                let point = Point::new(x as f64, y as f64);
                let value = sdf_from_arc_list(&arcs[..], &point, None);
                result[y * distance_field_size.0 + x] = value as u8;
            }
        }
    })
}

fn benchmark_traditional_font_rasterization_at_size(bencher: &mut Bencher,
                                                    font_path: String,
                                                    font_size: i32,
                                                    character: char) {
    bencher.iter(|| {
        let freetype_library = Library::init().unwrap();
        let face = create_face(&freetype_library, font_path.clone(), font_size);
        face.load_char(character as usize, RENDER).unwrap();
        let glyph = face.glyph();
        glyph.bitmap().buffer();
    })
}

