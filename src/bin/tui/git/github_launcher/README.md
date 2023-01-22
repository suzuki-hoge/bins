# GitHub Launcher ( bgh )
Open ***G***it***H***ub page(s) on browser.

## Usage: Launch selector
```
$ bgh
```

```
> 
+-------------------------------------------------------------------------------------------------+
|   Pulls: all                                                                                    |
|   Pulls: own                                                                                    |
|   Pulls: review                                                                                 |
|   Issues: all                                                                                   |
| * Tree: develop                                                                                 |
|   Tree: fix-style                                                                               |
|   Commits: develop                                                                              |
|   Commits: fix-style                                                                            |
|   PR: fix-style -> develop                                                                      |
|   Wiki: top                                                                                     |
|   File ( develop ): src/pages/items/index.tsx                                                   |
+-------------------------------------------------------------------------------------------------+
```

### Input search works on
- Pane 1 text

### Ctrl + Space
Fix current line.

The selected line disappears, and you can continue to select.

### Ctrl + f
***F***inish selection, then open selected line(s).

### Enter
Fix current line, then open selected line(s).

( `Enter` equals `Ctrl + Space` & `Ctrl + f` )

## Struct
```
- Page
  |-- Label --> [ raw ] --> Pane 1 --> [ fix ] --+
  `-- URL                                        +--> Open and exit
```

## File IO
### .git/config
Read current directory's config file.

## Pages
### Pulls
- https://github.com/{owner}/{repo}/pulls
- https://github.com/{owner}/{repo}/pulls/@me
- https://github.com/{owner}/{repo}/pulls?q=is:open+is:pr+-reviewed-by:@me+reviewed-by:@me

### Issues
- https://github.com/{owner}/{repo}/issues

### Tree
- https://github.com/{owner}/{repo}/tree/{branch}
  - current branch
  - base branch
  - local branches

### Commits
- https://github.com/{owner}/{repo}/commits/{branch}
  - current branch
  - base branch
  - local branches

### PR
- https://github.com/{owner}/{repo}/compare/{base-branch}...{current-branch}

### Wiki
- https://github.com/{owner}/{repo}/wiki

### File
- https://github.com/{owner}/{repo}/blob/{branch}/{path}
  - current branch
  - base branch
  - local branches
