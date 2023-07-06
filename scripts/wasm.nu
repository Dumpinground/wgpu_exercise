#!usr/bin/env nu
source example.nu
# run a example in wasm
def main [
  id: int # Input example id. Run tutorial_1 when id = 1.
  --release (-r) # Compile in Debug or Release Mode.
  feature = 'default'
  --target = wasm32-unknown-unknown
] {
  let build = if not $release { 'debug' } else { 'release' }
  let example = ( get_example $id | get example.0 )
  RUSTFLAGS=--cfg=web_sys_unstable_apis cargo b ( if $release { '-r' } ) --target $target --example $example -F $feature

  let wasm_path = $'target/($target)/($build)/examples/($example).wasm'
  wasm-bindgen --out-dir dist --web $wasm_path
}
