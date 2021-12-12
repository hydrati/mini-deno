// Copyright 2018-2021 the Deno authors. All rights reserved. MIT license.

use deno_core::op_sync;
use deno_core::Extension;
use deno_core::include_js_files;
use deno_core::OpState;
use deno_core::anyhow;
use deno_core::error::type_error;

use std::collections::HashMap;
use crate::permissions::Permissions;

use sysinfo::{System, SystemExt, ProcessorExt, Processor};
use sysinfo::AsU32;
use platforms::{TARGET_ARCH, TARGET_ENV, TARGET_OS};
use serde::Serialize;
use std::env;
use std::process;

pub fn op_os_physical_core_count(state: &mut OpState, _: (), _: ()) -> anyhow::Result<Option<usize>> {
  state.borrow_mut::<Permissions>().env.check_all()?;
  let sys = state.borrow_mut::<System>();
  Ok(sys.physical_core_count())
}

pub fn op_os_hostname(state: &mut OpState, _: (), _: ()) -> anyhow::Result<Option<String>> {
  state.borrow_mut::<Permissions>().env.check_all()?;
  let sys = state.borrow_mut::<System>();
  Ok(sys.host_name())
}

pub fn op_os_version(state: &mut OpState, _: (), _: ()) -> anyhow::Result<Option<String>> {
  state.borrow_mut::<Permissions>().env.check_all()?;
  let sys = state.borrow_mut::<System>();
  Ok(sys.os_version())
}

pub fn op_os_long_version(state: &mut OpState, _: (), _: ()) -> anyhow::Result<Option<String>> {
  state.borrow_mut::<Permissions>().env.check_all()?;
  let sys = state.borrow_mut::<System>();
  Ok(sys.long_os_version())
}

pub fn op_os_kernel_version(state: &mut OpState, _: (), _: ()) -> anyhow::Result<Option<String>> {
  state.borrow_mut::<Permissions>().env.check_all()?;
  let sys = state.borrow_mut::<System>();
  Ok(sys.kernel_version())
}

pub fn op_os_target_arch(_: &mut OpState, _: (), _: ()) -> anyhow::Result<&'static str> {
  Ok(TARGET_ARCH.as_str())
}

pub fn op_os_target_env(_: &mut OpState, _: (), _: ()) -> anyhow::Result<Option<&'static str>> {
  Ok(TARGET_ENV.map(|s| s.as_str()))
}

pub fn op_os_target_os(_: &mut OpState, _: (), _: ()) -> anyhow::Result<&'static str> {
  Ok(TARGET_OS.as_str())
}


pub fn op_os_env_get(state: &mut OpState, key: String, _: ()) -> anyhow::Result<Option<String>> {
  state.borrow_mut::<Permissions>().env.check(&key)?;
  if key.is_empty() || key.contains(&['=', '\0'] as &[char]) {
    return Err(type_error("Key contains invalid characters."));
  }
  let r = match env::var(key) {
    Err(env::VarError::NotPresent) => None,
    v => Some(v?),
  };

  Ok(r)
}

pub fn op_os_env_set(state: &mut OpState, key: String, value: String) -> anyhow::Result<()> {
  state.borrow_mut::<Permissions>().env.check(&key)?;
  let invalid_key = key.is_empty() || key.contains(&['=', '\0'] as &[char]);
  let invalid_value = value.contains('\0');
  if invalid_key || invalid_value {
    return Err(type_error("Key or value contains invalid characters."));
  }
  env::set_var(key, value);
  Ok(())
}

fn op_os_env_delete(
  state: &mut OpState,
  key: String,
  _: (),
) -> anyhow::Result<()> {
  state.borrow_mut::<Permissions>().env.check(&key)?;
  if key.is_empty() || key.contains(&['=', '\0'] as &[char]) {
    return Err(type_error("Key contains invalid characters."));
  }
  env::remove_var(key);
  Ok(())
}

fn op_os_env_get_kv(
  state: &mut OpState,
  _: (),
  _: (),
) -> anyhow::Result<HashMap<String, String>> {
  state.borrow_mut::<Permissions>().env.check_all()?;
  Ok(env::vars().collect())
}

fn op_os_env_get_entries(
  state: &mut OpState,
  _: (),
  _: (),
) -> anyhow::Result<Vec<(String, String)>> {
  state.borrow_mut::<Permissions>().env.check_all()?;
  Ok(env::vars().collect())
}


fn op_os_env_get_values(
  state: &mut OpState,
  _: (),
  _: (),
) -> anyhow::Result<Vec<String>> {
  state.borrow_mut::<Permissions>().env.check_all()?;
  Ok(env::vars().map(|v| v.1).collect())
}

fn op_os_env_get_keys(
  state: &mut OpState,
  _: (),
  _: (),
) -> anyhow::Result<Vec<String>> {
  state.borrow_mut::<Permissions>().env.check_all()?;
  Ok(env::vars().map(|v| v.0).collect())
}

pub fn op_os_env_has(state: &mut OpState, key: String, _: ()) -> anyhow::Result<bool> {
  state.borrow_mut::<Permissions>().env.check(&key)?;
  if key.is_empty() || key.contains(&['=', '\0'] as &[char]) {
    return Err(type_error("Key contains invalid characters."));
  }
  let r = match env::var(key) {
    Err(env::VarError::NotPresent) => false,
    v => {
      let _ = v?;
      true
    },
  };
  
  Ok(r)
}

pub fn ppid() -> i64 {
  #[cfg(windows)]
  {
    // Adopted from rustup:
    // https://github.com/rust-lang/rustup/blob/1.21.1/src/cli/self_update.rs#L1036
    // Copyright Diggory Blake, the Mozilla Corporation, and rustup contributors.
    // Licensed under either of
    // - Apache License, Version 2.0
    // - MIT license
    use std::mem;
    use winapi::shared::minwindef::DWORD;
    use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
    use winapi::um::processthreadsapi::GetCurrentProcessId;
    use winapi::um::tlhelp32::{
      CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32,
      TH32CS_SNAPPROCESS,
    };
    unsafe {
      // Take a snapshot of system processes, one of which is ours
      // and contains our parent's pid
      let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
      if snapshot == INVALID_HANDLE_VALUE {
        return -1;
      }

      let mut entry: PROCESSENTRY32 = mem::zeroed();
      entry.dwSize = mem::size_of::<PROCESSENTRY32>() as DWORD;

      // Iterate over system processes looking for ours
      let success = Process32First(snapshot, &mut entry);
      if success == 0 {
        CloseHandle(snapshot);
        return -1;
      }

      let this_pid = GetCurrentProcessId();
      while entry.th32ProcessID != this_pid {
        let success = Process32Next(snapshot, &mut entry);
        if success == 0 {
          CloseHandle(snapshot);
          return -1;
        }
      }
      CloseHandle(snapshot);

      // FIXME: Using the process ID exposes a race condition
      // wherein the parent process already exited and the OS
      // reassigned its ID.
      let parent_id = entry.th32ParentProcessID;
      parent_id.into()
    }
  }
  #[cfg(not(windows))]
  {
    use std::os::unix::process::parent_id;
    parent_id().into()
  }
}

pub fn op_os_current_ppid(state: &mut OpState, _: (), _: ()) -> anyhow::Result<i64> {
  state.borrow_mut::<Permissions>().env.check_all()?;
  Ok(ppid())
}

pub fn op_os_current_pid(state: &mut OpState, _: (), _: ()) -> anyhow::Result<i64> {
  state.borrow_mut::<Permissions>().env.check_all()?;
  match sysinfo::get_current_pid() {
    Err(_) => Ok(-1),
    Ok(p) => Ok(p.as_u32() as i64),
  }
}

pub fn op_os_loadavg(state: &mut OpState, _: (), _: ()) -> anyhow::Result<(f64, f64, f64)> {
  state.borrow_mut::<Permissions>().env.check_all()?;
  let sys = state.borrow_mut::<System>();
  let avg = sys.load_average();
  Ok((avg.one, avg.five, avg.fifteen))
}

#[derive(Serialize, Clone)]
pub struct ProcessorObject {
  name: String,
  vendor_id: String,
  brand: String,
  usage: f32,
  freq: u64,
}

impl From<&Processor> for ProcessorObject {
  fn from(obj: &Processor) -> Self {
    Self {
      name: obj.name().to_string(),
      vendor_id: obj.vendor_id().to_string(),
      brand: obj.brand().to_string(),
      usage: obj.cpu_usage(),
      freq: obj.frequency(),
    }
  }
}

pub fn op_os_global_processor_info(state: &mut OpState, _: (), _: ()) -> anyhow::Result<ProcessorObject> {
  state.borrow_mut::<Permissions>().env.check_all()?;
  let sys = state.borrow_mut::<System>();
  sys.refresh_cpu();
  Ok(sys.global_processor_info().into())
}

pub fn op_os_processors_info(state: &mut OpState, _: (), _: ()) -> anyhow::Result<Vec<ProcessorObject>> {
  state.borrow_mut::<Permissions>().env.check_all()?;
  let sys = state.borrow_mut::<System>();
  sys.refresh_cpu();
  Ok(sys.processors().iter().map(|s| s.into()).collect())
}

pub fn op_os_total_memory(state: &mut OpState, _: (), _: ()) -> anyhow::Result<u64> {
  state.borrow_mut::<Permissions>().env.check_all()?;
  let sys = state.borrow_mut::<System>();
  sys.refresh_memory();
  Ok(sys.total_memory())
}

pub fn op_os_free_memory(state: &mut OpState, _: (), _: ()) -> anyhow::Result<u64> {
  state.borrow_mut::<Permissions>().env.check_all()?;
  let sys = state.borrow_mut::<System>();
  sys.refresh_memory();
  Ok(sys.free_memory())
}

pub fn op_os_available_memory(state: &mut OpState, _: (), _: ()) -> anyhow::Result<u64> {
  state.borrow_mut::<Permissions>().env.check_all()?;
  let sys = state.borrow_mut::<System>();
  sys.refresh_memory();
  Ok(sys.available_memory())
}

pub fn op_os_used_memory(state: &mut OpState, _: (), _: ()) -> anyhow::Result<u64> {
  state.borrow_mut::<Permissions>().env.check_all()?;
  let sys = state.borrow_mut::<System>();
  sys.refresh_memory();
  Ok(sys.used_memory())
}

pub fn op_os_exit(_state: &mut OpState, code: i32, _: ()) -> anyhow::Result<()> {
  process::exit(code);
}

pub fn init() -> Extension {
  Extension::builder()
    .ops(vec![
      ("op_os_physical_core_count", op_sync(op_os_physical_core_count)),
      ("op_os_target_arch", op_sync(op_os_target_arch)),
      ("op_os_target_env", op_sync(op_os_target_env)),
      ("op_os_target_os", op_sync(op_os_target_os)),
      ("op_os_hostname", op_sync(op_os_hostname)),
      ("op_os_version", op_sync(op_os_version)),
      ("op_os_long_version", op_sync(op_os_long_version)),
      ("op_os_kernel_version", op_sync(op_os_kernel_version)),

      ("op_os_env_get", op_sync(op_os_env_get)),
      ("op_os_env_set", op_sync(op_os_env_set)),
      ("op_os_env_delete", op_sync(op_os_env_delete)),
      ("op_os_env_has", op_sync(op_os_env_has)),
      ("op_os_env_get_kv", op_sync(op_os_env_get_kv)),
      ("op_os_env_get_values", op_sync(op_os_env_get_values)),
      ("op_os_env_get_keys", op_sync(op_os_env_get_keys)),
      ("op_os_env_get_entries", op_sync(op_os_env_get_entries)),

      ("op_os_current_ppid", op_sync(op_os_current_ppid)),
      ("op_os_current_pid", op_sync(op_os_current_pid)),

      ("op_os_loadavg", op_sync(op_os_loadavg)),
      ("op_os_global_processor_info", op_sync(op_os_global_processor_info)),
      ("op_os_processors_info", op_sync(op_os_processors_info)),

      ("op_os_total_memory", op_sync(op_os_total_memory)),
      ("op_os_free_memory", op_sync(op_os_free_memory)),
      ("op_os_available_memory", op_sync(op_os_available_memory)),
      ("op_os_used_memory", op_sync(op_os_used_memory)),

      ("op_os_exit", op_sync(op_os_exit)),
    ])
    .js(include_js_files! (
      prefix "deno:ext/os2",
      "scripts/os/01_os_ops.js",
      "scripts/os/02_os_env.js",
      "scripts/os/90_os_ns.js",
    ))
    .state(|state| {
      let mut sys = System::new();
      sys.refresh_cpu();
      state.put(sys);
      Ok(())
    })
    .build()
}

