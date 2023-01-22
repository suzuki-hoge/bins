# GitHub Pull Request Launcher ( bpr )
Open browser or checkout GitHub ***p***ull ***r***equest.

## Usage: Launch selector
```
$ bpr
```

```
> 
+--------------------------------------------------+----------------------------------------------+
|   #298 : Fix application page style        alice | Lorem ipsum dolor sit amet, consectetur      |
| * #275 : Send mail on cancel.              bob   | adipiscing elit, sed do eiusmod tempor       |
|                                                  | incididunt ut labore et dolore magna         |
|                                                  | aliqua. Ut enim ad minim veniam, quis        |
|                                                  |                                              |
+--------------------------------------------------+----------------------------------------------+
```

### Input search works on
- Pane 1 text
- Pane 2 text

### Pane 1: Ctrl + o
***O***pen browser and exit.

### Pane 1: Ctrl + c
***C***heckout and exit.

## Struct
```
- PullRequest
  |-- Number      --+
  |-- Title       --+--> [ mix ] --> Pane 1 --> [ fix ] --+
  |-- Author      --+                                     |
  |-- Branch                                              +--> Checkout and exit
  |-- Url                                                 +--> Open and exit
  `-- Description -----> [ raw ] --> Pane 2
```

## File IO
### .git/config
Read current directory's config file.

### ~/.bins-github-credential.yaml
Read GitHub [Personal Access Token](https://github.com/settings/tokens).

```yaml
name: suzuki-hoge
pat: ghp_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```
