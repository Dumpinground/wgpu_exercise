#!usr/bin/env nu
source example.nu
# run a example in wasm
def main [
  id: int # Input example id. Run tutorial_1 when id = 1.
  feature = 'default' # Set this to 'challenge' to run a challenge example.
  --release (-r) # Compile in Debug or Release Mode.
  --target = wasm32-unknown-unknown # Can be set to another target ep. wasm32-wasi.
  --webgl
] {
  let build = if not $release { 'debug' } else { 'release' }
  let example = ( get_example $id | get example.0 )

  let cmd_build = (
    [
      RUSTFLAGS=--cfg=web_sys_unstable_apis    
      cargo b --target $target --example $example -F $feature
      (if $release { '-r' })
      (if $webgl { '-F webgl' })
    ] 
      | filter { |x| $x != null } 
      | str join ' '
  )
  nu -c $cmd_build

  let wasm_path = $'target/($target)/($build)/examples/($example).wasm'
  wasm-bindgen --out-dir dist --web $wasm_path

  cp index.html dist/index.html
}
