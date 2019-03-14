#! /bin/bash

HERE=$(cd `dirname $0`; pwd)
URL=$HERE/doc/doxygen/html/index.html
if [ "$(which xdg-open 2>/dev/null)" ]; then
	xdg-open $URL &
else
	open $URL &
fi >/dev/null 2>&1
