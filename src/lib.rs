//! This package provides bindings to the PAPI performance counters
//! library.

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(non_upper_case_globals)]
#[allow(deref_nullptr)]
mod bindings;
pub mod counter;
pub mod events_set;

use std::ffi::CStr;
use std::fmt::Debug;
use std::os::raw::{c_int, c_ulong};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::bindings::*;

fn papi_version_number(maj: u32, min: u32, rev: u32, inc: u32) -> u32 {
    (maj << 24) | (min << 16) | (rev << 8) | inc
}

fn current_papi_version() -> Result<c_int, PapiError> {
    let output = Command::new("papi_version")
        .output()
        .map_err(|_| PapiError { code: PAPI_ESYS })?;

    if !output.status.success() {
        return Err(PapiError { code: PAPI_ESYS });
    }

    let cur_version = std::str::from_utf8(&output.stdout)
        .map_err(|_| PapiError { code: PAPI_EINVAL })?;

    let mut digits = cur_version
        .trim()
        .split_whitespace()
        .last()
        .ok_or(PapiError { code: PAPI_EINVAL })?
        .split('.')
        .map(|d| d.parse::<u32>().map_err(|_| PapiError { code: PAPI_EINVAL }));

    let maj = digits.next().ok_or(PapiError { code: PAPI_EINVAL })??;
    let min = digits.next().ok_or(PapiError { code: PAPI_EINVAL })??;
    let rev = digits.next().ok_or(PapiError { code: PAPI_EINVAL })??;
    let inc = digits.next().ok_or(PapiError { code: PAPI_EINVAL })??;

    Ok((papi_version_number(maj, min, rev, inc) & 0xffff0000) as c_int)
}

#[link(name = "papi")]
extern "C" {}

#[allow(dead_code)]
pub struct PapiError {
    code: i32,
}

impl Debug for PapiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_msg_buf = unsafe { PAPI_strerror(self.code) };
        let err_msg = unsafe { CStr::from_ptr(err_msg_buf) };
        write!(
            f,
            "PapiError with error code {}, i.e \"{}\"",
            self.code,
            err_msg.to_str().unwrap_or_else(|_| "NULL")
        )
    }
}
pub(crate) fn check_error(code: i32) -> Result<(), PapiError> {
    if code == (PAPI_OK as i32) {
        Ok(())
    } else {
        Err(PapiError { code })
    }
}

static THREAD_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

thread_local! {
    static THREAD_INDEX: u64 = THREAD_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
}

extern "C" fn get_thread_id() -> c_ulong {
    THREAD_INDEX.with(|id| *id as c_ulong)
}

pub fn initialize(multithread: bool) -> Result<(), PapiError> {
    let cur_version = current_papi_version()?;

    unsafe {
        let version = PAPI_library_init(cur_version);
        if version != cur_version {
            return Err(PapiError { code: version });
        }

        if multithread {
            check_error(PAPI_thread_init(Some(get_thread_id)))?;
        }
    }

    Ok(())
}

/// ALL EventsSet should be dropped before calling this
pub fn terminate() {
    unsafe {
        PAPI_shutdown();
    }
}

pub fn is_initialized() -> bool {
    unsafe { check_error(PAPI_is_initialized()).is_ok() }
}

// The only reasonable action for counters_in_use is to
// retry. Otherwise, you might as well just fail yourself.
#[derive(PartialEq, Eq)]
pub enum Action {
    Retry,
}

#[cfg(test)]
mod tests {
    use crate::counter::Counter;
    use crate::events_set::EventsSet;
    use crate::initialize;
    use crate::terminate;
    use crate::PapiError;

    #[test]
    fn test_papi_error() {
        // https://bitbucket.org/icl/papi/wiki/PAPI-Error-Handling.md
        // source for expected error messages
        initialize(true).unwrap();
        let error = PapiError { code: -7 };
        let msg = format!("{error:?}");
        assert_eq!(
            msg,
            "PapiError with error code -7, i.e \"Event does not exist\""
        );
        terminate()
    }
}
