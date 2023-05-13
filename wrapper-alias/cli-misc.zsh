build_dir=$(cd "$(dirname "${0}")" && git rev-parse --show-toplevel)/target/release

alias b-find='$build_dir/finder'
