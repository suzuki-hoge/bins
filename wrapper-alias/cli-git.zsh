build_dir=$(cd "$(dirname "${0}")" && git rev-parse --show-toplevel)/target/release

alias gad='$build_dir/git-add'
alias gadf='gstf git add'

alias gbdf='git branch | bf | xargs git branch -D'
alias gbh='$build_dir/git-branch'

alias gcl='$build_dir/git-clean'

alias gcm='$build_dir/git-commit'

alias gcma='$build_dir/git-commit-amend'

alias gco='$build_dir/git-checkout'
alias gcof='git branch | bf | cut -c3- | xargs git checkout'

alias glg='$build_dir/git-log'

alias gpl='$build_dir/git-pull'

alias gpr='$build_dir/git-pull-request'

alias gprc='$build_dir/git-pull-request-commit'

alias gps='$build_dir/git-push'

alias grb='$build_dir/git-rebase'

alias grbc='$build_dir/git-rebase-continue'

alias gsh='$build_dir/git-stash'

alias gst='$build_dir/git-status'
alias gstf='git status --short | bf | cut -c4- | xargs'
