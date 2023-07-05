#!usr/bin/env nu
source example.nu
# run a example in wasm
def main [
  id: int # Input example id. Run tutorial_1 when id = 1.
  feature = 'default'
] {
  let example = ( get_example $id )
  RUSTFLAGS=--cfg=web_sys_unstable_apis cargo b --target wasm32-unknown-unknown --example $example -F $feature
}
