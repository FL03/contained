#!/bin/bash

# flags for the script:
# --no-link: don't create a symlink to the result in the current directory
# -b or --build: build the package and create a symlink to the result in the current directory (default)

[ "$1" = "--no-link" ] && no_link=true && shift
[ "$1" = "-b" ] && build_flag=true && shift
[ "$1" = "--build" ] && build_flag=true && shift

if [ -n "$build_flag" ]; then
  echo "Building the package and creating a symlink to the result in the current directory..."
else
  echo "Building the package without creating a symlink to the result in the current directory..."
fi

if [ -n "$build_flag" ]; then
  nix --extra-experimental-features 'nix-command flakes' build --allow-dirty "$@"
else
  nix --extra-experimental-features 'nix-command flakes' "$@"
fi


