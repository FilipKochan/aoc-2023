$day = $args[0]
mkdir $day
cd $day
cargo init --name "aoc$day" 2>&1 | %{ "$_" }
"target\n" | Out-File ".gitignore"
cargo add --path "../base/rust_base/" 2>&1 | %{ "$_" }
code .