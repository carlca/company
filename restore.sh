#!/bin/zsh
mongorestore --db company --dir ./company
echo "MongoDB data restored from $(pwd)/company"
