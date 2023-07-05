#!/usr/bin/env nu

# get a example
def get_example [
  id: int # Input example id.
] {
  let file = (ls examples | where name =~ ('tutorial_' + ( $id | into string ) )
    | get name.0
    | parse '{dir}\{example}'
  )
  $file | get example.0
}
