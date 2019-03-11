#![allow(dead_code)]

mod raw;

use libc::{fclose, fdopen};

use std::error;
use std::ffi::{CStr, CString, IntoStringError};
use std::fmt;
use std::fs::File;
use std::marker::PhantomData;
use std::os::unix::io::IntoRawFd;
use std::path::Path;
use std::result;

use std::os::raw::{c_char, c_int};

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        // Displaying an `Error` simply displays the pool's error string.
        self.message.fmt(f)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }
}

impl From<std::ffi::NulError> for Error {
    fn from(e: std::ffi::NulError) -> Error {
        Error {
            message: format!("interior nul byte found at {}", e.nul_position()).to_owned(),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error {
            message: format!("{}", e).to_owned(),
        }
    }
}

pub type Result<T> = result::Result<T, Error>;

pub struct Pool {
    ptr: *mut raw::Pool,
}

impl Pool {
    pub fn new() -> Pool {
        unsafe {
            Pool {
                ptr: raw::pool_create(),
            }
        }
    }

    pub fn set_arch(&mut self, arch: &str) -> Result<()> {
        let arch = CString::new(arch)?;
        unsafe {
            raw::pool_setarch(self.ptr, arch.as_ptr());
        }
        Ok(())
    }
}

impl Drop for Pool {
    fn drop(&mut self) {
        unsafe { raw::pool_free(self.ptr) }
    }
}

pub struct Repository<'pool> {
    ptr: *mut raw::Repo,
    pool: &'pool Pool,
}

pub struct Solvable<'pool> {
    id: c_int,
    pool: &'pool Pool
}

impl<'pool> Solvable<'pool> {
    pub fn to_string(&self) -> String {
        unsafe {
            CStr::from_ptr(raw::pool_id2str(self.pool.ptr, self.id)).to_string_lossy().into_owned()
            // return CStr::from_ptr(raw::pool_id2str(self.pool.ptr, self.id));
        }
    }
    pub fn set_name(&self, name: &str) -> Result<()> {
        let cname = CString::new(name)?;
        unsafe {
            // pool->solvables[xs->id].name = raw::pool_str2id(self.pool.ptr, cname.as_ptr(), 1);
        }
        return Ok(());
    }
}

impl<'pool> Drop for Solvable<'pool> {
    fn drop(&mut self) {
        unsafe {}
    }
}

impl<'pool> Repository<'pool> {
    pub fn new(pool: &'pool Pool, name: &str) -> Result<Repository<'pool>> {
        let name = CString::new(name)?;
        unsafe {
            let repo = raw::repo_create(pool.ptr, name.as_ptr());
            Ok(Repository {
                ptr: repo,
                pool: pool,
            })
        }
    }

    pub fn add_solvable(&mut self) -> Result<Solvable<'pool>> {
        unsafe {
            let new_id = raw::repo_add_solvable(self.ptr);
            return Ok(Solvable {id: new_id, pool: &self.pool });
        }
    }

    // Read repo from .solv file and add it to pool
    pub fn set_solv_file(&mut self, solv_file: &Path, flags: Option<i32>) -> Result<()> {
        let f = File::open(solv_file)?;

        unsafe {
            let fp = fdopen(
                f.into_raw_fd(),
                CStr::from_bytes_with_nul_unchecked(b"r\0").as_ptr(),
            );
            raw::repo_add_solv(self.ptr, fp, flags.unwrap_or(0));
            fclose(fp);
        }

        Ok(())
    }
}

impl<'pool> Drop for Repository<'pool> {
    fn drop(&mut self) {
        // Free the repo from the pool and don't reuse the IDs of the solvables.
        unsafe { raw::repo_free(self.ptr, 0) }
    }
}

pub enum DistType {
    Rpm = 0,
    Deb = 1,
    Arch = 2,
    Haiku = 3,
}

pub const REPO_REUSE_REPODATA: i32 = 1;
pub const REPO_NO_INTERNALIZE: i32 = 2;
pub const REPO_LOCALPOOL: i32 = 4;
pub const REPO_USE_LOADING: i32 = 8;
pub const REPO_EXTEND_SOLVABLES: i32 = 16;
pub const REPO_USE_ROOTDIR: i32 = 32;
pub const REPO_NO_LOCATION: i32 = 64;
