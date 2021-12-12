pub mod extensions;
pub mod fs_util;
pub mod colors;
pub mod permissions;

use deno_core::anyhow;
use deno_core::JsRuntime;
use deno_core::RuntimeOptions;
use crate::permissions::Permissions;
use std::str::FromStr;
use std::path::PathBuf;
use std::fs;
// use std::env;

fn main() -> anyhow::Result<()> {
  let out_dir = PathBuf::from_str(".")?; // &env::var("OUT_DIR")?
  let mut runtime = create_runtime();
  let snapshot = runtime.snapshot();
  fs::write(out_dir.join("v8_context_snapshot.bin"), &snapshot)?;
  Ok(())
}

fn create_runtime() -> JsRuntime {

  let console = deno_console::init();

  let web = deno_web::init(Default::default(), None);
  let webidl = deno_webidl::init();
  let url = deno_url::init();
  let timers = deno_timers::init::<Permissions>();
  let core = extensions::core::init();
  let os = extensions::os::init();

  let runtime = JsRuntime::new(RuntimeOptions {
      js_error_create_fn: None,
      get_error_class_fn: None,
      module_loader: None,
      extensions: vec![
          webidl,
          url,
          web,
          console,
          timers, 
          os,
          core,
      ],
      startup_snapshot: None,
      will_snapshot: true,
      create_params: None,
      v8_platform: None,
      shared_array_buffer_store: None,
      compiled_wasm_module_store: None,
  });

  return runtime;
}