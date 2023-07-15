#!/usr/bin/env nu
source example.nu
# run a example
def main [
  id: int # Input example id. Run tutorial_1 when id = 1.
  --release (-r) # Compile in Debug or Release Mode.
  feature = 'default' # Define an enabled feature
] {
  let example = ( get_example $id | get example.0 )
  # cargo r --example $example -F $feature
  let cmd_run = ( [cargo r --example $example -F $feature ( if $release { '-r' } )] | filter { |x| $x != null } |str join ' ' )
  nu -c $cmd_run
}
