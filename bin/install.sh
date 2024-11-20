#!/bin/bash

ROOT=$(dirname $0)

cargo b -r --manifest-path $ROOT/../Cargo.toml
cp $ROOT/../target/x86_64-pc-windows-gnu/release/webphishing.dll $HOME/.local/share/Steam/steamapps/common/WEBFISHING/winmm.dll
