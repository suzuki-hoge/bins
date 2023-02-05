# Git Stash ( gsh )

## gsh

alias to ***g***it ***s***tas***h***.

```
$ gsh -h
bins 0.1.0

USAGE:
    git-stash [FLAGS] [message]

FLAGS:
    -h, --help       Prints help information
    -l, --list       show stash list ( default )
    -p, --pop        pop
    -s, --save       stash
    -V, --version    Prints version information

ARGS:
    <message>    message for save
```

examples.

``` 
$ gsh -s
```

``` 
$ gsh -s 'foo bar'
```

``` 
$ gsh -p
```
