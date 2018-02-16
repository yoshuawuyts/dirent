#[macro_use]
extern crate arrayref;
extern crate dirent_sys;
#[macro_use]
extern crate quicli;

use std::ptr;
use std::ffi::CString;
use quicli::prelude::*;
use dirent_sys::dirent::{dirent, scandir};

#[derive(StructOpt)]
struct Cli {
  dir: String,
}

main!(|args: Cli| {
  let dir = CString::new(args.dir)?;
  let mut namelist: *mut *mut dirent = ptr::null_mut::<*mut dirent>();
  println!("{:?}", namelist);

  let res = unsafe { scandir(dir.as_ptr(), &mut namelist, None, None) };
  println!("{:?}", res);
  unsafe {
    let data = (**namelist).d_name;
    let mydata = array_ref![data, 0, res];
    println!("{:?}", mydata);
  }
});
