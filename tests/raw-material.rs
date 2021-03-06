// Copyright 2014-2017 Hyeon Kim
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate obj;

use obj::raw::material::{RawMtl, Material, MtlColor, MtlTextureMap, parse_mtl};

fn fixture(filename: &str) -> RawMtl {
    use std::path::Path;
    use std::fs::File;
    use std::io::BufReader;

    let path = Path::new("tests").join("fixtures").join(filename);
    let file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => panic!("Failed to open \"{}\". \x1b[31m{}\x1b[0m", path.to_string_lossy(), e)
    };
    let input = BufReader::new(file);

    parse_mtl(input).unwrap()
}

#[test]
fn cube() {
    let mtl = fixture("cube.mtl");
    assert_eq!(mtl.materials.len(), 1);

    let mat = mtl.materials.get("Material").unwrap();
    assert_eq!(mat, &Material {
        ambient:              Some(MtlColor::Rgb(0.0, 0.0, 0.0)),
        diffuse:              Some(MtlColor::Rgb(0.64, 0.64, 0.64)),
        specular:             Some(MtlColor::Rgb(0.5, 0.5, 0.5)),
        dissolve:             Some(1.0),
        specular_exponent:    Some(96.078431),
        optical_density:      Some(1.0),
        illumination_model:   Some(2),
        diffuse_map:          Some(MtlTextureMap { file: "cube-uv-num.png".to_owned() }),
        ..Material::default()
    });
}

#[test]
fn untitled() {
    let mtl = fixture("untitled.mtl");
    assert_eq!(mtl.materials.len(), 2);

    let mat = mtl.materials.get("Material").unwrap();
    assert_eq!(mat, &Material {
        ambient:              Some(MtlColor::Rgb(0.0, 0.0, 0.0)),
        diffuse:              Some(MtlColor::Rgb(0.64, 0.64, 0.64)),
        specular:             Some(MtlColor::Rgb(0.5, 0.5, 0.5)),
        dissolve:             Some(1.0),
        specular_exponent:    Some(96.078431),
        optical_density:      Some(1.0),
        illumination_model:   Some(2),
        ..Material::default()
    });

    let mat = mtl.materials.get("None").unwrap();
    assert_eq!(mat, &Material {
        ambient:              Some(MtlColor::Rgb(0.0, 0.0, 0.0)),
        diffuse:              Some(MtlColor::Rgb(0.8, 0.8, 0.8)),
        specular:             Some(MtlColor::Rgb(0.8, 0.8, 0.8)),
        dissolve:             Some(1.0),
        specular_exponent:    Some(0.0),
        illumination_model:   Some(2),
        ..Material::default()
    });
}
