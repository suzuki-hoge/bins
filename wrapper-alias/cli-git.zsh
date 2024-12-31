build_dir=$(cd "$(dirname "${0}")" && git rev-parse --show-toplevel)/target/release

alias gad='git add .'
alias gadf='gstf git add'

alias gbh='git branch'
alias gbdf='git branch | bf | xargs git branch -D'

alias gcl='git reset . && git checkout . && git clean -d -f'

function gcm {
  git commit --message "$1"
}
alias gcma='git commit --amend --no-edit'

alias gco='$build_dir/git-checkout'
alias gcof='git branch | bf | cut -c3- | xargs git checkout'

alias glg='$build_dir/git-log'

alias gpl='$build_dir/git-pull'

alias gpr='$build_dir/git-pull-request'

alias gprc='$build_dir/git-pull-request-commit'

alias gps='$build_dir/git-push'

alias grb='$build_dir/git-rebase'
alias grba='git rebase --abort'
alias grbc='git rebase --continue'

alias gsh='$build_dir/git-stash'

alias gst='git status --short --branch'
alias gstf='git status --short | bf | cut -c4- | xargs'
