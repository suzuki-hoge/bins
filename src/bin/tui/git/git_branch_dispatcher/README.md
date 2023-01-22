# Git Branch Dispatcher ( bgb )

Dispatch action to ***G***it ***b***ranch.

## Usage: Launch selector

```
$ bgb
```

```
> 
+--------------------------------------------------+----------------------------------------------+
|   develop                                        | 2023/01/18 suzuki-hoge fix api parameters.   |
|   feat/auth                                      | 2023/01/18 GitHub Merge pull request #301    |
| * feat/payment                                   |                                              |
|   hotfix/style                                   |                                              |
|                                                  |                                              |
+--------------------------------------------------+----------------------------------------------+
```

### Input search works on

- Pane 1 text

### Keys

| key     | description                       | constraint |
|---------|-----------------------------------|------------|
| C-c     | ***C***heckout branch, then exit. |            |
| C-d     | ***D***elete branch.              |            |

## Struct

```
- Branch
  |-- Name --> [ raw ] --> Pane 1 --> [ fix ] --> Run and exit
  `-- Logs --> [ raw ] --> Pane 2
```

## File IO

| file          | description                                       |
|---------------|---------------------------------------------------|
| ./.git/config | If missing, fail launch.                          |
