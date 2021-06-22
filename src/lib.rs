//! # Buffy
//! <p align="center">
//!   <a><img alt="logo" src="./logo.png"></a>
//! </p>
//! <h1 align="center"> Buffy </h1>
//! <p align="center">
//!   <a><img alt="MAINTAINED" src="https://img.shields.io/badge/Maintained%3F-yes-green.svg"></a>
//!   <a><img alt="Downloads" src="https://img.shields.io/crates/d/buffy"></a>
//!   <a href="https://crates.io/crates/buffy"><img alt="crates.io" src="https://img.shields.io/crates/v/buffy.svg"></a>
//!   <a><img alt="License" src="https://img.shields.io/badge/License-MIT-blue.svg"></a>
//! </p>
//! <p align="center">
//!   <a><img alt="issues" src="https://img.shields.io/github/issues/cowboy8625/buffy"></a>
//!   <a><img alt="last commit" src="https://img.shields.io/github/last-commit/cowboy8625/buffy"></a>
//!   <a><img alt="repo size" src="https://img.shields.io/github/repo-size/cowboy8625/buffy"></a>
//!   <a href="https://discord.gg/KwnGX8P"><img alt="Discord Chat" src="https://img.shields.io/discord/509849754155614230"></a>
//! </p>
//! <p align="center">
//!   <a><img alt="RUST" src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white"></a>
//! </p>
//!
//! # Table Of Contents:
//!
//!   - [**About**](#about)
//!   - [**Docs**]("https://crates.io/crates/buffy")
//!   - [**Road Map**](#road-map)
//!
//! # About
//!
//! > The motivation be hide creating ***Buffy*** was the need for a buffer for the screen in my text
//! editor [ReVi](https://github.com/revi-editor/revi).  The aim for this project can be seen in the
//! [road map](#road-map) which is not that detailed yet.
//!
//! # Road Map
//!
//! - [ ] **Complete Unicode Support**
//! - [ ] **Smart Update Buffer** Knowing what line or character was change to just update over redrawing screen.
mod test;

use std::slice::SliceIndex;
use unicode_segmentation::UnicodeSegmentation;

pub struct Buffy {
    _inner: Vec<String>,
}

impl Buffy {
    pub fn new(item: &str) -> Self {
        let _inner = item.graphemes(true).map(ToString::to_string).collect::<Vec<String>>();
        Self {
            _inner,
        }
    }

    pub fn insert(&mut self, index: usize, item: &str) {
        let mut graph = item.graphemes(true).map(ToString::to_string).collect::<Vec<String>>();
        graph.reverse();
        for g in graph {
            self._inner.insert(index, g);
        }
    }

    pub fn get<I>(&self, index: I) ->
        Option<&<I as SliceIndex<[String]>>::Output>
        where I: SliceIndex<[String]> {
        self._inner.get(index)
    }

    pub fn words(&self) -> Vec<String> {
        // FIXME: Works......
        self._inner.iter().map(Clone::clone).collect::<String>().unicode_words().map(ToString::to_string).collect::<Vec<String>>()
    }
}
