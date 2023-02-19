build_dir=$(cd "$(dirname "${0}")" && git rev-parse --show-toplevel)/target/release

alias gwb='$build_dir/git_hub_launcher'
alias gprf='$build_dir/git_hub_pull_request_review_launcher'
