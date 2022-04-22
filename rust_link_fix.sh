#!/bin/sh

LINKER="cc"

link_args=
for arg in "$@"; do
    case "${arg}" in
    -l*) link_args="${link_args} ${arg}" ;;
    esac
done

# copy link args to end
exec "${LINKER}" "$@" ${link_args}
