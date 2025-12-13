#!/bin/sh

########################################################
## Dioxus CLI (`dx`) 0.6.3 is used in this case.      ##
## Install it using `cargo install dioxus-cli@0.6.3`. ##
########################################################

RUST_BACKTRACE=1 dx serve --port 3003 --platform web

