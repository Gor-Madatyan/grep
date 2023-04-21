use core::convert::AsRef;
use std::ops::{Deref, DerefMut};

use aho_corasick::AhoCorasick;

use crate::kinds::Config;

mod impls;

pub struct Grep<H: AsRef<[u8]>, M: AsRef<[u8]>> {
    corasick: AhoCorasick,
    pub haystack: H,
    pub config: Config<M>,
}

