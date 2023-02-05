# Git Log ( glg )

## glg

alias to ***g***it ***l***o***g***.

```
$ glg -h

bins 0.1.0

USAGE:
    git-log [FLAGS] [count]

FLAGS:
    -h, --help       Prints help information
    -l, --long       show long format
    -V, --version    Prints version information

ARGS:
    <count>     [default: 5]
```

examples.

```
$ glg
2023/01/31
  suzuki-hoge - development-starter: start.
  suzuki-hoge - github-launcher: open specified url.

2023/01/29
  suzuki-hoge - github-launcher: fix base branch and open urls.

2023/01/28
  suzuki-hoge - github-launcher: add urls.
  suzuki-hoge - create wrapped bins.
```

```
$ glg -l 3
87f4838 [ 2023/01/31 09:55:14 ] suzuki-hoge development-starter: start.
7679fba [ 2023/01/31 08:52:47 ] suzuki-hoge github-launcher: open specified url.
77123e3 [ 2023/01/29 22:19:06 ] suzuki-hoge github-launcher: fix base branch and open urls.
```
