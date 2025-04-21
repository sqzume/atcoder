#!/usr/bin/env fish

cargo compete new $argv
cargo member include $argv
cd $argv/

