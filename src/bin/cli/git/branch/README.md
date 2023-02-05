# Git Branch ( gbh / gbdf )

## gbh
alias to ***g***it ***b***ranc***h***.

```
$ gbh -h
bins 0.1.0

USAGE:
    git-branch [FLAGS]

FLAGS:
    -a, --all        list both remote-tracking and local branches
    -h, --help       Prints help information
    -V, --version    Prints version information
```

examples.

``` 
$ gbh

  auth                                                                                          |
* feat/issue-42                                                                                 |
  hotfix/index                                                                                  |
```

## gbdf
***g***it ***b***ranc***h*** ***d***elete with ***f***ilter selection.

```
$ gbdf
```

```
> 
+-------------------------------------------------------------------------------------------------+
|   auth                                                                                          |
| * feat/issue-42                                                                                 |
|   hotfix/index                                                                                  |
|                                                                                                 |
|                                                                                                 |
+-------------------------------------------------------------------------------------------------+
```
