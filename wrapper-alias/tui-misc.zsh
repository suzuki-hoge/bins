build_dir=$(cd "$(dirname "${0}")" && git rev-parse --show-toplevel)/target/release

alias bf='$build_dir/filter'
alias bcl='$build_dir/calendar'
alias bd='eval `$build_dir/development_starter`'
