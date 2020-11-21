# ANTLER
A textureless rendering engine written in Rust.

<!-- [![ANTLER documentation](https://docs.rs/antler/badge.svg)](https://docs.rs/antler) -->
[![ANTLER crate](https://img.shields.io/crates/v/antler.svg)](https://crates.io/crates/antler)
[![ANTLER documentation](https://docs.rs/antler/badge.svg)](https://freddywordingham.github.io/antler/)
[![minimum rustc 1.47](https://img.shields.io/badge/rustc-1.47+-red.svg)](https://www.rust-lang.org/)
[![Build Status](https://travis-ci.org/FreddyWordingham/arctk.svg?branch=master)](https://travis-ci.org/FreddyWordingham/arctk)

# Usage

1) If you don't already have it install Rust: https://www.rust-lang.org/
2) Install Antler using: `cargo install antler`
3) Navigate to input directory
4) Run: `antler parameters.json5`
5) Images written into ./output directory.
6) Add attributes in src/parts/attributes.rs
7) Add new output in src/pipe/output.rs
8) Tinker with the src/engines/antler.rs file to add new effects!

# Documentation
Binary specific https://freddywordingham.github.io/antler/

Supporting library https://freddywordingham.github.io/arctk/

# Examples
![](./light_and_shadow.png)
View the <- light and shadow -> calculations.
![](./metric.png)
![](./data.png)
Render both <- space and time -> (+ other metrics distance/surface normals/etc.)

![](https://github.com/FreddyWordingham/antler/blob/master/res/renders/silver.png)
![](https://github.com/FreddyWordingham/antler/blob/master/res/renders/wide.png)
![](https://github.com/FreddyWordingham/antler/blob/master/res/renders/vivid.png)
![](https://github.com/FreddyWordingham/antler/blob/master/res/renders/dinofluff.png)
![](https://github.com/FreddyWordingham/antler/blob/master/res/renders/shatter.png)
![](https://github.com/FreddyWordingham/antler/blob/master/res/renders/triangle.png)
![](https://github.com/FreddyWordingham/antler/blob/master/res/renders/antler.png)
[More renderings here!](https://www.instagram.com/____f.r.e.d.d.y____/)
