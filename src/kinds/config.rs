use std::io::{self, Read};
use std::ops::{Deref, DerefMut};

use aho_corasick::AhoCorasickBuilder;

use crate::grep::Grep;
use crate::grep_over_file::GOFF;

#[derive(Debug, Clone)]
pub struct Config<M: AsRef<[u8]>> {
    pub(super) matches: Vec<M>,
    pub(super) corasick_builder: AhoCorasickBuilder,
}


impl<M: AsRef<[u8]>> Config<M> {
    pub fn new<V: IntoIterator<Item=M>>(matches: V) -> Self {
        let corasick_builder = AhoCorasickBuilder::new();
        let matches = matches.into_iter().collect::<Vec<M>>();
        Self {
            corasick_builder,
            matches,
        }
    }

    pub fn auto_configure(&mut self) -> &mut Self {
        self.corasick_builder.auto_configure(self.matches.as_slice());
        self
    }


    pub fn build_grep<H: AsRef<[u8]>>(self, haystack: H) -> Grep<H, M> {
        Grep::new(self, haystack)
    }

    pub fn build_goff<R: Read>(self, stream: R) -> Result<GOFF<M, R>, io::Error> {
        GOFF::new(self, stream)
    }

    pub(crate) fn get_matches(&self) -> &[M] {
        &self.matches
    }
}

impl<M: AsRef<[u8]>> Deref for Config<M> {
    type Target = AhoCorasickBuilder;

    fn deref(&self) -> &Self::Target {
        &self.corasick_builder
    }
}

impl<M: AsRef<[u8]>> DerefMut for Config<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.corasick_builder
    }
}

impl<V: IntoIterator<Item=M>, M: AsRef<[u8]>> From<V> for Config<M> {
    fn from(value: V) -> Self {
        Config::new(value)
    }
}

