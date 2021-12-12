# mini-deno
 Mini-JavaScript Runtime Powered By Deno

## Build
```powershell
$env:V8_SNAPSHOT = "./v8_context_snapshot.bin"
cargo run --bin build_snapshot
cargo build --bin neptune --release
```
