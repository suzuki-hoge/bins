# Git Checkout ( gco / gcof )

## gco
alias to ***g***it ***c***heck***o***ut.

```
$ gco -h
bins 0.1.0

USAGE:
    git-checkout [FLAGS] [OPTIONS] <target>

FLAGS:
    -b, --branch     create and checkout a new branch
    -f, --feature    create sub branch under feature branch
    -h, --help       Prints help information
    -o, --origin     checkout from remote origin
    -V, --version    Prints version information

OPTIONS:
    -p, --prefix <prefix>    add branch prefix

ARGS:
    <target>    branch name or file name
```

examples.

```
$ gco -p auth -fb issue-42    # git checkout -b auth/feature/issue-42
```

```
$ gco -fbo issue-42           # git checkout -b feature/issue-42 origin/feature/issue-42
```

## gcof

***g***it ***c***heck***o***ut with ***f***ilter selection.
```
$ gcof
```

```
> 
+-------------------------------------------------------------------------------------------------+
|   auth
| * feat/issue-42
|   hotfix/index
|                                                                                                 |
|                                                                                                 |
+-------------------------------------------------------------------------------------------------+
```
