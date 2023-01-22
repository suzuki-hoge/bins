# Development Launcher ( bd )
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

### Ctrl + c ( input area only )
Mark for ***c***hange directory.

### Ctrl + e ( input area only )
Mark for launch ***e***ditor.

### Ctrl + g ( input area only )
Mark for launch ***G***itHub launcher ( `bgh` ).

### Ctrl + u ( input area only )
Mark for launch Command launcher's ***u***p ( `bb up` ).

### Ctrl + d ( input area only )
**D**elete project.

It disappears on pane 1.

### Ctrl + s ( pane 2 only )
**S**ave tags.

### Tab
Jump cursor alternately between input area and pane 2.

### Enter ( input area only )
Run all mark.

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
### ~/.bins-preset-actions.yaml
If missing, fail launch.

Check [Migration](#).
