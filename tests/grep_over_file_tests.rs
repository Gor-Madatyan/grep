use std::io;
use std::io::{Cursor, Seek};

use grep_lib::Config;

#[test]
fn write_haystack() -> io::Result<()> {
    let mut vwec = vec![12, 32, 43u8];
    let mut gf = Config::new(["hell"]).build_goff(Cursor::new(&mut vwec))?;
    let haystack = "fo1fofofo1fffd1ttttui".as_bytes();
    gf.rewind()?;
    gf.haystack = haystack.to_owned();
    gf.write_haystack()?;
    assert_eq!(vwec, haystack);
    Ok(())
}



#[test]
fn set_stream() -> io::Result<()> {
    let rsl = [12, 32, 12, 44u8];
    let nrsl = [43, 67, 9u8];
    let mut gf = Config::new(["23323GHH"]).build_goff(rsl.as_slice())?;
    gf.set_stream_and_update_haystack(&nrsl)?;
    assert_eq!(gf.haystack, &nrsl);
    Ok(())
}



#[test]
fn update_haystack_with_stream() -> io::Result<()> {
    let rsl = [12, 21, 43, 76u8];
    let mut gf = Config::new(["23323GHH"]).build_goff(Cursor::new(rsl.as_slice()))?;
    gf.rewind()?;
    gf.haystack = "b23323GHH".into();
    gf.update_haystack_with_stream()?;
    assert_eq!(gf.haystack, &rsl);//if use &[u8] as stream gf.haystack.as_bytes() will be empty
    Ok(())
}



