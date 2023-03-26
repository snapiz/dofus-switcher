#!/bin/bash

data=$(wmctrl -l)
readarray -t windows <<<"$data"
names=()
visible_windows=()

for window in "${windows[@]}"; do
    window_data=($window)
    id=${window_data[0]}
    state=`xprop -id $id | grep "_NET_WM_STATE(ATOM)"`

    if [[ $window == *"Dofus"* ]] && [[ $state != *"_NET_WM_STATE_HIDDEN"* ]]; then
        visible_windows+=(${window_data[3]})
    fi
done

echo "${visible_windows[@]}"