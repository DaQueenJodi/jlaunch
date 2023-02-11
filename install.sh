#!/bin/sh
APP=jlaunch
SCRIPT_LOC=${XDG_DATA_HOME:-~/.local/share}/$APP/
cargo install --path .
cp -r Scripts $SCRIPT_LOC

