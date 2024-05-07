#!/usr/bin/env bash

if test -f './token.txt'; then
    DISCORD_TOKEN=$(cat token.txt) cargo run
else
    echo 'put the bot token in ./token.txt'
fi
