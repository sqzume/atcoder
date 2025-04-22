#!/usr/bin/env fish

cd $argv[1]
cargo compete submit $argv[2]
cd ..

