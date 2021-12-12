// Copyright 2018-2021 the Deno authors. All rights reserved. MIT license.


pub mod core;
pub mod permissions;
pub mod os;

use deno_core::OpState;
use std::rc::Rc;
use std::cell::RefCell;
use deno_core::anyhow;
use deno_core::JsRuntime;

/// `UnstableChecker` is a struct so it can be placed inside `GothamState`;
/// using type alias for a bool could work, but there's a high chance
/// that there might be another type alias pointing to a bool, which
/// would override previously used alias.
#[derive(Clone, Copy)]
pub struct UnstableChecker {
  pub unstable: bool,
}

impl UnstableChecker {
  /// Quits the process if the --unstable flag was not provided.
  ///
  /// This is intentionally a non-recoverable check so that people cannot probe
  /// for unstable APIs from stable programs.
  // NOTE(bartlomieju): keep in sync with `cli/program_state.rs`
  pub fn check_unstable(&self, api_name: &str) {
    if !self.unstable {
      eprintln!(
        "Unstable API '{}'. The --unstable flag must be provided.",
        api_name
      );
      std::process::exit(70);
    }
  }
}
/// Helper for checking unstable features. Used for sync ops.
pub fn check_unstable(state: &OpState, api_name: &str) {
  state.borrow::<UnstableChecker>().check_unstable(api_name)
}

/// Helper for checking unstable features. Used for async ops.
pub fn check_unstable2(state: &Rc<RefCell<OpState>>, api_name: &str) {
  let state = state.borrow();
  state.borrow::<UnstableChecker>().check_unstable(api_name)
}

static ACTIVE_TIMERS: &'static str = r#"((window)=>{
  window.Deno.timers.__activateTimerMacrotask();
  delete window.Deno.timers.__activateTimerMacrotask;
})(this);"#;

pub fn activate_timers_macrotask(runtime: &mut JsRuntime) -> anyhow::Result<()> {
  runtime.execute_script("deno:timers_macrotask", ACTIVE_TIMERS)?;
  Ok(())
}