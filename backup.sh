#!/bin/zsh
mongodump --db company --out .
echo "MongoDB data backed up to $(pwd)/company"
