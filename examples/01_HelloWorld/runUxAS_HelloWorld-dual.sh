#! /bin/bash

echo "NOTICE: Running HelloWorld dual example from task01 executable."
echo "Use \`pkill task01\` in a separate window to stop."
echo "Press enter to continue"
read
BIN="../../build/task01"
$BIN -cfgPath cfg_HelloWorld-1.xml &
$BIN -cfgPath cfg_HelloWorld-2.xml &

