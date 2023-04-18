use core::convert::AsRef;
use std::ops::{Deref, DerefMut};

use aho_corasick::AhoCorasick;

use crate::kinds::Config;

#[cfg(test)]
mod tests {
    use aho_corasick::MatchKind;

    use super::*;

    #[test]
    fn recomply() {
        let mut config: Config<_> = ["1234", "5678!", "2142"].into();
        config.match_kind(MatchKind::LeftmostLongest);
        let mut grep = config.build_grep("GI12H_5678!_G".to_owned());
        let new_matches = ["4232", "3424566"];
        grep.config.matches = new_matches.into();
        grep.recomply();
        assert_eq!(grep.pattern_count(), new_matches.len());
        assert_eq!(grep.config.get_matches(), new_matches);
        assert_eq!(grep.match_kind(), &MatchKind::LeftmostLongest);
    }
}


pub struct Grep<H: AsRef<[u8]>, M: AsRef<[u8]>> {
    corasick: AhoCorasick,
    pub haystack: H,
    config: Config<M>,

}

impl<H: AsRef<[u8]>, M: AsRef<[u8]>> Grep<H, M> {
    pub fn new(config: Config<M>, haystack: H) -> Self {
        let corasick = config.build(config.get_matches());
        Grep { config, corasick, haystack }
    }

    fn recomply(&mut self) {
        self.corasick = self.config.corasick_builder.build(self.config.get_matches())
    }


    pub fn set_matches(&mut self, matches: impl Into<Vec<M>>) {
        self.config.matches = matches.into();
        self.recomply();
    }

    pub fn set_matches_and_auto_configure(&mut self, matches: impl Into<Vec<M>>) {
        let vector = matches.into();
        self.config.corasick_builder.auto_configure(&vector);
        self.set_matches(vector);
    }


    pub fn find(&self) -> Option<aho_corasick::Match> {
        self.deref().find(&self.haystack)
    }

    pub fn find_iter(&self) -> aho_corasick::FindIter<usize> {
        self.deref().find_iter(&self.haystack)
    }


    pub fn find_overlapping_iter(&self) -> aho_corasick::FindOverlappingIter<usize> {
        self.deref()
            .find_overlapping_iter(&self.haystack)
    }

    pub fn earliest_find(&self) -> Option<aho_corasick::Match> {
        self.deref().earliest_find(&self.haystack)
    }
    pub fn is_match(&self) -> bool {
        self.deref().is_match(&self.haystack)
    }

    pub fn replace_all_bytes<B: AsRef<[u8]>>(&self, replace_with: &[B]) -> Vec<u8> {
        self.deref()
            .replace_all_bytes(self.haystack.as_ref(), replace_with)
    }


    pub fn replace_all_with_bytes<F>(&self, dst: &mut Vec<u8>, func: F)
        where
            F: FnMut(&aho_corasick::Match, &[u8], &mut Vec<u8>) -> bool,
    {
        self.deref()
            .replace_all_with_bytes(self.haystack.as_ref(), dst, func)
    }
}


impl<H: AsRef<str> + AsRef<[u8]>, M: AsRef<[u8]>> Grep<H, M> {
    pub fn replace_all<B: AsRef<str>>(&self, rep_to: &[B]) -> String {
        self.deref()
            .replace_all(self.haystack.as_ref(), rep_to)
    }
    pub fn replace_all_with<F>(&self, dst: &mut String, func: F)
        where
            F: FnMut(&aho_corasick::Match, &str, &mut String) -> bool,
    {
        self.deref()
            .replace_all_with(self.haystack.as_ref(), dst, func)
    }
}

impl<M: AsRef<[u8]>> Grep<Vec<u8>, M> {
    pub fn replace_all_bytes_and_save<B: AsRef<[u8]>>(&mut self, replace_with: &[B]) {
        self.haystack = self.replace_all_bytes(replace_with);
    }
}

impl<M: AsRef<[u8]>> Grep<String, M> {
    pub fn replace_all_bytes_and_save<B: AsRef<str>>(&mut self, replace_with: &[B]) {
        self.haystack = self.replace_all(replace_with);
    }
}

impl<H: AsRef<[u8]>, M: AsRef<[u8]>> Deref for Grep<H, M> {
    type Target = AhoCorasick;
    fn deref(&self) -> &Self::Target {
        &self.corasick
    }
}

impl<H: AsRef<[u8]>, M: AsRef<[u8]>> DerefMut for Grep<H, M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.corasick
    }
}
