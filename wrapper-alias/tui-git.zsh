build_dir=$(cd "$(dirname "${0}")" && git rev-parse --show-toplevel)/target/release

alias gwb='$build_dir/git-hub-launcher'
alias gprf='$build_dir/git-hub-pull-request-review-launcher'
