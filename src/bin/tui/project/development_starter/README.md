# Development Starter ( bd )

Start ***d***evelopment.

## Usage: Launch selector

```
$ bd
```

```
> 
+------------------------------+------------------------------------------------------------------+
|   bins                       | ts, react, next, yarn                                            |
|   langbox-haskell            |                                                                  |
| * foo-frontend               |                                                                  |
|   foo-backend                |                                                                  |
|   corporate-site             |                                                                  |
|   dotfiles                   |                                                                  |
|                              |                                                                  |
+------------------------------+------------------------------------------------------------------+
```

### Input search works on

- Pane 1 text
- Pane 2 text

### Keys

| key     | description                                              | constraint      |
|---------|----------------------------------------------------------|-----------------|
| C-c     | Mark for ***c***hange directory.                         | input area only |
| C-e     | Mark for launch ***e***ditor.                            | input area only |
| C-g     | Mark for launch ***G***itHub launcher ( `bgh` ).         | input area only |
| C-u     | Mark for launch Command launcher's ***u***p ( `bb up` ). | input area only |
| C-d     | **D**elete project.<br>It disappears on pane 1.          | input area only |
| Enter   | Run all marked line.                                     | input area only |
| C-s     | **S**ave tags.                                           | pane 2 only     |
| Tab     | Jump cursor alternately between input area and pane 2.   ||

## Struct

```
- Project
  |-- Directory                                   +--> Run and exit
  |-- Name            --+--> [ mix ] --> Pane 1 --+
  |-- Mark            --+
  |-- DirectoryExists --+
  |-- GitConfigExists --+
  |-- UpCommandExists --+
  `-- Tags            -----> [ raw ] --> Pane 2
```

## File IO

| file                   | description                                       |
|------------------------|---------------------------------------------------|
| ~/.bins-project-mapper | If missing, fail launch. ( Check [Migration](#) ) |
