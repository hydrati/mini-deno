pub mod extensions;
pub mod fs_util;
pub mod colors;
pub mod permissions;

use deno_core::anyhow;
use deno_core::JsRuntime;
use deno_core::Snapshot;
use deno_core::RuntimeOptions;
use deno_core::FsModuleLoader;
use std::process;
use crate::permissions::Permissions;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::rc::Rc;

static CONTEXT_V8_SNAPSHOT: &'static [u8] = include_bytes!(env!("V8_SNAPSHOT"));

fn create_runtime() -> anyhow::Result<JsRuntime> {
    let module = Rc::new(FsModuleLoader {});

    let console = deno_console::init();

    let web = deno_web::init(Default::default(), None);
    let webidl = deno_webidl::init();
    let url = deno_url::init();
    let timers = deno_timers::init::<Permissions>();
    let core = extensions::core::init();
    let os = extensions::os::init();

    let mut runtime = JsRuntime::new(RuntimeOptions {
        js_error_create_fn: None,
        get_error_class_fn: None,
        module_loader: Some(module),
        extensions: vec![
            webidl,
            url,
            web,
            console,
            timers, 
            os,
            core,
        ],
        startup_snapshot: Some(Snapshot::Static(CONTEXT_V8_SNAPSHOT)),
        will_snapshot: false,
        create_params: None,
        v8_platform: None,
        shared_array_buffer_store: None,
        compiled_wasm_module_store: None,
    });

    
    extensions::activate_timers_macrotask(&mut runtime)?;
    runtime.sync_ops_cache();
    Ok(runtime)
}

use std::thread;
use std::sync::mpsc::channel;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut runtime = create_runtime()?;
    // let shot = runtime.snapshot();
    // fs::write("./snapshot_blob.bin", &shot).await?;
    // let main_module = resolve_url_or_path("./main.js")?;
    // let mod_id = runtime.load_main_module(&main_module, None).await?;

    // let mod_result = runtime.mod_evaluate(mod_id);


    let (tx, rx) = channel::<(String, String)>();
    let (tx2, rx2) = channel::<()>();
    let _ = thread::spawn(move || {
        let mut will_exit = false;
        let mut count: u128 = 0;
        let mut editor = Editor::<()>::new();

        loop {
            let readline = editor.readline(">> ");
            match readline {
                Ok(line) => {
                    will_exit = false;
                    if line.trim() == "" { continue; }
                    editor.add_history_entry(line.as_str());
                    let script_name = format!("<repl:{}>", count);
                    let script = format!("{}", line);
                    tx.send((script_name, script)).unwrap();
                    let _ = rx2.recv();
                    count += 1;
                }
                Err(ReadlineError::Interrupted) => {
                    if will_exit {
                        process::exit(0);
                    } else {
                        will_exit = true;
                        println!("");
                        println!("(To exit, press Ctrl+C again or Ctrl+D or type .exit)")
                    }
                }
                Err(e) => { panic!("REPL Error: {:?}", e); }
            }
        }
    });
    
    // setTimeout(()=>console.log("hello"),1000)
    println!("Codename \"Neptune\", JS Runtime.");
    println!("debug build, test only.");
    
    
    loop {
        if let Ok((id, script)) = rx.try_recv() {
            if let Err(e) = runtime.execute_script(&id, &script) {
                eprintln!("{} {}", colors::red_bold("EvalError:"), e);
            }
            tx2.send(())?;
        }
        runtime.run_event_loop(false).await?;
    }


    // Ok(())
}
