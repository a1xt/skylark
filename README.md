# Raytron

[![license](https://img.shields.io/badge/license-MIT_or_Apache_2.0-blue.svg)](#license)
[![](https://tokei.rs/b1/github/a1xt/raytron)](#)
[![Build Status](https://travis-ci.org/a1xt/raytron.svg?branch=master)](https://travis-ci.org/a1xt/raytron)
[![Build status](https://ci.appveyor.com/api/projects/status/3sbwhwio9x9isghb?svg=true)](https://ci.appveyor.com/project/a1xt/raytron)

Yet another path tracer.

## Building:
```
git clone https://github.com/a1xt/raytron.git
cd raytron
git submodule init
git submodule update
cargo run --example envmap --release
```

## Implemented:
- Primitives: sphere, triangle
- Direct lighting
- Kd-tree accelerator
- Materials: Lambertian, Phong, Cook-Torrance
- Textures

## Gallery:
<p align="left">
  <img src="https://github.com/a1xt/raytron/blob/master/gallery/rusted_teapot1_3000spp.png" height="200" alt="RustedTeapot"/>
  <img src="https://github.com/a1xt/raytron/blob/master/gallery/materials_3600spp.png" height="200" alt="Materials"/>
  <img src="https://github.com/a1xt/raytron/blob/master/gallery/spheres_4500spp.png" height="200" alt="Spheres"/>
</p>

## License
Raytron is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.
