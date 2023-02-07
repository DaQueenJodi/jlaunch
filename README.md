# Install:
`cargo install --path .`
# Usage:
`cargo --help`
(also make sure to download WineGE via `jlaunch download wine-ge` if you use it, it won't do it automatically (yet)
# Manual Configuration
if you want, you can add games as a json file in XDG_DATA_HOME (~/.local/share by default)
heres a useless example for an entry for `vim`:
```
{
  "name": "vim",
  "path": "/usr/bin/vim",
  "runner": "Native",
  "options": {
    "gamescope": false,
    "gamemode": false
  }
}
```
# TODO: (not in any specific order)
- [x] make it launch games from a config 
- [x] make it download winege
- [x] make it have a seperate prefix per game
- [x] add a cli
- [ ] make it automatically download runners when running games or at least warn the user
- [ ] add a tui
- [ ] make it download other deps like winetricks locally
- [ ] make it use winetricks to manage stuff declaratively
- [ ] make a full game installer system
- [ ] allow it to use proton if possible
