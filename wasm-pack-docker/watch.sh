#!/bin/bash
 
if [ -z "$(which inotifywait)" ]; then
    echo "inotifywait not installed."
    echo "In most distros, it is available in the inotify-tools package."
    exit 1
fi
 
counter=0;
 
function execute() {
    echo "Compiling. $@" 
    eval "$@"
}
 
execute "$@"

inotifywait --recursive --monitor --format "%e %w%f" \
--event modify,move,create,delete ./src \
| while read changed; do
    counter=$((counter+1))
    echo "Detected change n. $counter" 
    echo $changed
    execute "$@"
done