use std::{fs, io};
use std::fs::File;
use std::io::Seek;

use crate::Config;

#[test]
fn write_haystack() -> io::Result<()> {
    let path = "tests/util_text_files/changable_for_tests.txt";
    File::create(path)?;
    let mut gf = Config::new(["1"]).build_goff(File::options().read(true).write(true).open(path)?)?;
    let haystack = "fo1fofofo1fffd1ttttui";
    gf.haystack = haystack.to_owned();
    gf.write_haystack()?;
    let text = fs::read_to_string(path)?;
    assert_eq!(&text, haystack);
    Ok(())
}

#[test]
fn fall_write_haystack() {
    let mut gf = Config::new(["34"]).build_goff(File::open("tests/util_text_files/readonly.txt").unwrap()).unwrap();
    let haystack = "ASSA34a";
    gf.haystack = haystack.to_owned();
    assert!(gf.write_haystack().is_err());
}


#[test]
fn set_stream() -> io::Result<()> {
    let new_path = "Cargo.lock";
    let mut gf = Config::new(["23323GHH"]).build_goff(File::open("Cargo.toml").unwrap())?;
    gf.set_stream(File::open(new_path)?)?;
    let text = fs::read_to_string(new_path)?;
    assert_eq!(&gf.haystack, &text);
    Ok(())
}


#[test]
fn fall_set_stream() {
    let mut gf = Config::new(["23323GHH"]).build_goff(File::open("Cargo.toml").unwrap()).unwrap();
    assert!(gf.set_stream(File::open("tests/util_text_files/invutf-8.txt").unwrap()).is_err())
}


#[test]
fn fall_update_haystack_with_stream() {
    let mut gf = Config::new(["23323GHH"]).build_goff(File::open("Cargo.toml").unwrap()).unwrap();
    gf.stream = File::open("tests/util_text_files/invutf-8.txt").unwrap();
    assert!(gf.update_haystack_with_stream().is_err());
}

#[test]
fn update_haystack_with_stream() -> io::Result<()> {
    let path = "Cargo.toml";
    let cargo = fs::read_to_string(path)?;
    let mut gf = Config::new(["23323GHH"]).build_goff(File::open(path).unwrap())?;
    gf.stream.rewind()?;
    gf.haystack = "b23323GHH".to_owned();
    gf.update_haystack_with_stream()?;
    assert_eq!(&gf.haystack, &cargo);
    Ok(())
}




