#!/bin/bash

year=2025
day=${1}
printf -v dd %02d "${day}"
dir="${year}/${dd}"
name="y${year}d${dd}"
message="${year} day ${day}."

if [[ -a $dir ]]; then
    echo >&2 "Path ${dir} already exists."
    exit 1
fi

jj new -m "${message}" trunk || exit
cp -r template "${dir}" || exit
cargo init --name "${name}" "${dir}"
