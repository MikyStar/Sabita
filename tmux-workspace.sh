#!/bin/sh

########################################
###############  Config  ###############
########################################

# Working directory, use $HOME for absolute path
path="."

# Format : session:window.pane

# Session
session="Sabita"

# Windows
main="$session:0"

# Panes
editor="$main.0"
term00="$main.1"

# Commands
edit="ed"
lint="cargo clippy"

########################################
###############  Script  ###############
########################################

tmux has-session -t "$session" > /dev/null 2>&1

if [ ! $? != 0 ]; then # Already created
  tmux attach -t "$session" > /dev/null 2>&1
  exit 0
fi

###

cd "$path"

tmux new-session -d -s "$session"

###

tmux rename-window -t $main "main"

###

tmux split-window -t $main -h # YES, I know ... This is vertical

###

tmux resize-pane -t 0 -R 350

###

tmux send-keys -t $editor "$edit" Enter
tmux send-keys -t $term00 "$lint" Enter

###

tmux selectp -t 0

tmux attach -t $session
