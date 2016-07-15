#!/bin/sh
set -x

${CC:=cc}
$CC -T $(dirname $0)/link.ld "$@"
