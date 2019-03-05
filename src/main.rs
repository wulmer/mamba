//#![feature(custom_derive, plugin)]
//#![plugin(serde_macros)]

extern crate reqwest;
mod solv;
use std::collections::HashMap;

extern crate serde;
use serde_json::{self, Value};


fn main() -> Result<(), Box<std::error::Error>> {
    let resp: Value = reqwest::get("https://conda.anaconda.org/quantstack/linux-64/repodata.json")?.json()?;
    println!("{:?}", resp["info"]["platform"]);

    let mut pool = solv::Pool::new();
    pool.set_arch("linux_x64")?;

    let mut updates = solv::Repository::new(&mut pool, "updates")?;
    let id = updates.add_solvable();

    println!("{:?}", id.to_string(&pool));
    

    println!("Hello, world!");
    Ok(())
}
