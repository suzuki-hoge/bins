build_dir=$(cd "$(dirname "${0}")" && git rev-parse --show-toplevel)/target/release

alias bf='$build_dir/filter'
alias bgh='$build_dir/git_hub_launcher'
