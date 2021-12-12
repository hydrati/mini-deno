use deno_core::op_sync;
use deno_core::op_async;
use deno_core::Extension;
use deno_core::include_js_files;
use deno_core::OpState;
use deno_core::anyhow;

use crate::permissions::Permissions;
use crate::permissions::PermissionsOptions;
use super::UnstableChecker;

use std::rc::Rc;
use std::cell::RefCell;

fn test_sync(_state: &mut OpState, nums: Vec<f64>, _: ()) -> anyhow::Result<f64> {
  Ok(nums.iter().fold(0.0, |a, v| a + v))
}

async fn test_async(_state: Rc<RefCell<OpState>>, nums: Vec<f64>, _: ()) -> anyhow::Result<f64> {
  Ok(nums.iter().fold(0.0, |a, v| a + v))
}

pub fn init() -> Extension {
  let perms = Permissions::from_options(&PermissionsOptions {
    allow_env: None,
    allow_hrtime: true,
    allow_net: None,
    allow_ffi: None,
    allow_read: None,
    allow_run: None,
    allow_write: None,
    prompt: true,
  });

  let unstable_checker = UnstableChecker { unstable: true, };

  Extension::builder()
    .js(include_js_files!(
        prefix "edgeless:ext/core",
        "scripts/00_util.js",
        "scripts/01_init.js",
        "scripts/01_web_util.js",
        "scripts/01_version.js",
        "scripts/99_main.js",
    ))
    .ops(vec![
        ("op_sum_sync", op_sync(test_sync)),
        ("op_sum_async", op_async(test_async))
    ])
    .state(move |state: &mut OpState| {
      state.put::<Permissions>(perms.to_owned());
      state.put::<UnstableChecker>(unstable_checker);
      Ok(())
    })
    .build()
}