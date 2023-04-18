use std::io::{self, Read, Seek, SeekFrom, Write};
use std::ops::{Deref, DerefMut};

use crate::Config;
use crate::grep::Grep;

pub struct GOFF<M, R>
    where
        M: AsRef<[u8]>,
        R: Read
{
    grep: Grep<Vec<u8>, M>,
    stream: R
}


impl<M, WR> GOFF<M, WR>
    where
        M: AsRef<[u8]>,
        WR: Write + Read
{
    pub fn write_haystack(&mut self) -> io::Result<()> {
        let data: &[u8] = self.grep.haystack.as_ref();
        self.stream.write_all(data)
    }
}

impl<M, R> GOFF<M, R>
    where
        M: AsRef<[u8]>,
        R: Read
{
    pub fn set_stream_and_update_haystack(&mut self, stream: R) -> io::Result<usize> {
        self.stream = stream;
        self.update_haystack_with_stream()
    }

    pub fn new(config: Config<M>, mut stream: R) -> Result<Self, io::Error> {
        let mut haystack = Vec::new();
        stream.read_to_end(&mut haystack)?;
        let grep = config.build_grep(haystack);

        Ok(GOFF {
            grep,
            stream,
        })
    }


    pub fn update_haystack_with_stream(&mut self) -> io::Result<usize> {
        let haystack = &mut self.grep.haystack;
        haystack.clear();
        self.stream.read_to_end(haystack)
    }
}


impl<M, R> Seek for GOFF<M, R>
    where
        M: AsRef<[u8]>,
        R: Read + Seek {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.stream.seek(pos)
    }
    fn rewind(&mut self) -> io::Result<()> {
        self.stream.rewind()
    }

    fn stream_position(&mut self) -> io::Result<u64> {
        self.stream.stream_position()
    }
}


impl<M, R> Deref for GOFF<M, R>
    where
        M: AsRef<[u8]>,
        R: Read {
    type Target = Grep<Vec<u8>, M>;
    fn deref(&self) -> &Self::Target {
        &self.grep
    }
}


impl<M, R> DerefMut for GOFF<M, R>
    where

        M: AsRef<[u8]>,
        R: Read
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.grep
    }
}