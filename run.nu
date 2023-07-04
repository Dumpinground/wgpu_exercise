#!/usr/bin/env nu

# run a example
def main [
  id: int # Input example id. Run tutorial_1 when id = 1.
  feature = 'default' # Define an enabled feature
] {
  let file = ( ls examples | where name =~ ('tutorial_' + ( $id | into string) )
    | get name.0
    | parse '{dir}\{example}'
  )
  $file
  cargo r --example ( $file | get example.0 ) -F $feature
}
