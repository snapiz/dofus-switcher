#!/bin/bash

for PID in `ps -ef| awk '/dofus-switcher\/switcher.sh/ {print $2}'`; do
    if [[ $PID != $$ ]]; then
        kill -9 $PID
    fi
done

keys_press=()
default_names=(
    Eni-doco Bilicow
)


function update_names() {
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

    for name in "${default_names[@]}"; do
        if [[ $data == *"$name"* ]] && [[ " ${visible_windows[*]} " =~ " ${name} " ]]; then
            names+=($name)
        fi
    done
}

update_names

xinput test-xi2 --root 3 | grep -A2 --line-buffered RawKey | while read -r line; do
    if [[ $line != *"detail"* ]]; then
        continue
    fi

    key=$( echo $line | sed "s/[^0-9]*//g")

    if [[ " ${keys_press[*]} " == *"$key"* ]]; then
        keys_press=("${keys_press[@]/$key}")
    else
        keys_press+=($key)
        continue
    fi

    current=$(xprop -id $(xprop -root _NET_ACTIVE_WINDOW | cut -d ' ' -f 5) WM_NAME | awk -F '"' '{print $2}')

    if [[ $current != *"Dofus"* ]]; then
        continue
    fi

    current=(${current//;/ })
    current=${current[0]}

    if [[ " ${keys_press[*]} " == *"64"* ]]; then
        continue
    fi

    len=${#names[@]}
    last=$((len - 1))

    case $key in

    23)
        for i in "${!names[@]}"; do
            if [[ "${names[$i]}" != "${current}" ]]; then
                continue
            fi

            next=$((i + 1))

            if [[ next -gt last  ]]; then
                next=0
            fi

            wmctrl -a ${names[$next]} - Dofus
        done
        ;;

    67 | 68 | 69 | 70 | 71 | 72 | 73 | 74)
        next=$((key - 67))

        if [[ -v names[next] ]]; then
            wmctrl -a ${names[$next]} - Dofus
        fi
        ;;
    49)
        for i in "${!names[@]}"; do
            wmctrl -a ${names[$i]} - Dofus
            sleep 0.15
            xdotool click 1
        done
        ;;
    75)
        for i in "${!names[@]}"; do
            wmctrl -a ${names[$i]} - Dofus
            sleep 0.15
            xdotool click 3
        done
        ;;
    76)
        for i in "${!names[@]}"; do
            wmctrl -a ${names[$i]} - Dofus
            sleep 0.15
            xdotool click --repeat 2 1
            sleep 0.15
        done
        ;;
    95)
        for i in "${!names[@]}"; do
            if [ "$i" -eq 0 ]; then
                continue
            fi
            wmctrl -a ${names[$i]} - Dofus
            sleep 0.15
            xdotool click 1
        done
        ;;
    110)
        wmctrl -a ${names[0]} - Dofus
        sleep 0.15
        xdotool key space
        sleep 0.15
        for i in "${!names[@]}"; do
            if [ "$i" -eq 0 ]; then
                continue
            fi
            xdotool type "/invite ${names[$i]}"
            sleep 0.15
            xdotool key Return
            sleep 0.20
        done
        wmctrl -a ${names[1]} - Dofus
        ;;
    112)
        data=$(wmctrl -l)
        readarray -t windows <<<"$data"

        for window in "${windows[@]}"; do
            window_data=($window)
            id=${window_data[0]}
            state=`xprop -id $id | grep "_NET_WM_STATE(ATOM)"`

            if [[ $window == *"Dofus"* ]] && [[ $state == *"_NET_WM_STATE_HIDDEN"* ]]; then
                wmctrl -R ${window_data[3]} - Dofus
            fi
        done

        update_names
        ;;
    117)
        xdotool windowminimize $(xdotool getactivewindow)
        update_names
        ;;
    esac
done
