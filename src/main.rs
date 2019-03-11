//#![feature(custom_derive, plugin)]
//#![plugin(serde_macros)]

extern crate reqwest;
mod solv;
use std::collections::HashMap;
use std::mem::forget;
extern crate serde;
use serde_json::{self, Value};


extern crate libc;
extern crate libsolv;
use libc::*;
use libsolv::*;

// use libsolv::{s_Pool};


fn main() -> Result<(), Box<std::error::Error>> {
    // let resp: Value = reqwest::get("https://conda.anaconda.org/quantstack/linux-64/repodata.json")?.json()?;
    // println!("{:?}", resp["info"]["platform"]);
    Pool p;
    // let mut pool = solv::Pool::new();
    // std::mem::forget(&pool);

    // pool.set_arch("linux_x64")?;

    // let mut updates = solv::Repository::new(&pool , "updates")?;
    // let mut id = updates.add_solvable()?;
    // id.set_name("test");
    // println!("{:?}", id.to_string());
    
    // println!("Hello, world!");
    Ok(())
}
