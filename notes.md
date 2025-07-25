# General idea
- allows to switch configs in 2 modes:
    - Safer - copy's the whole config (custom + cfg) to `tf/` (chance to recover original cfg)
    - Less space - move's the whole config to /tf folder (no chance to recover original cfg in case something goes wrong)

- in case a cfg doesn't have defined either `cfg/` or `custom/` only the defined folder gets switched (allow for cfg/custom dir mixing)

# Stored info
## Application config
- `tf_path` - steamapps/commmon/Team Fortress 2/tf/ path
- `switch_mode` - either safer or less space modes (default - *safe*)

## "Config" config
Localized in every config directory
- ~~`name`~~ - name of the config (Will be used as defined folder)
- `description` - description of the cfg (ex. whats in the cfg, is it just cfg or also custom etc, date)
- `author` - original author
- `alias` - quick switching alias 

# TOFO
- [ ] What to do in case of override errors



# Cli
```
tf2-cfgmgr list
tf2-cfgmgr init
tf2-cfgmgr switch <cfg_name>
```

```
tf2-cfgmgr --list
tf2-cfgmgr <cfg_name>
```

## TUI
**POSSIBLE TODO** Will be used in list mode
