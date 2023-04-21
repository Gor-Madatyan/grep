use std::io::{self, Read, Seek, SeekFrom, Write};
use std::ops::{Deref, DerefMut};

use crate::Config;
use crate::grep::Grep;

mod impls;

pub struct GOFF<M, R>
    where
        M: AsRef<[u8]>,
        R: Read
{
    grep: Grep<Vec<u8>, M>,
    stream: R,
}

