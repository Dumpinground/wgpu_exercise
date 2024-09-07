def main [
  --verbose (-v)
] {
  let cmd = (
    [
      miniserve dist --index index.html
      --header 'x-content-type-options:nosniff'
      --header 'Cache-control:no-cache'
      (if ($verbose) { '-v' })
    ]
      | filter { |x| $x != null }
      | str join ' '
  )

  nu -c $cmd
}

# miniserve dist --index index.html --header 'x-content-type-options: nosniff' --header 'Cache-control: no-cache'
