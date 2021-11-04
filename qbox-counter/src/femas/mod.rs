mod api;

use anyhow::Result;
use qbox_core::counter::{Counter, Driver};
#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStrExt;
use std::sync::Arc;
use url::Url;

#[derive(Debug, Default)]
pub struct FemasDriver;

impl Driver for FemasDriver {
    fn create(&self, opt: Url) -> Result<Arc<dyn Counter>> {
        unimplemented!()
    }
}
