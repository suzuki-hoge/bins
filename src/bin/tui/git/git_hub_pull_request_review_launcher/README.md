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

### Keys

| key | description                  | constraint |
|-----|------------------------------|------------|
| C-o | ***O***pen browser and exit. |            |
| C-c | ***C***heckout and exit.     |            |

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

| file                    | description                                                                                                                                                             |
|-------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| ~/.bins-credential.yaml | Read GitHub [Personal Access Token](https://github.com/settings/tokens).<br>key = `github_personal_access_token`<br>value = `lghp_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx` |
| ./.git/config           | If missing, fail launch.                                                                                                                                                |
