#!/usr/bin/env nu
source example.nu
# run a example
def main [
  id: int # Input example id. Run tutorial_1 when id = 1.
  feature = 'default' # Define an enabled feature
] {
  let example = ( get_example $id )
  cargo r --example $example -F $feature
}
