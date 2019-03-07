#! /bin/bash

echo "NOTICE: Running HelloWorld dual example from task01 executable."
echo "When running, press Return to stop."
echo "Press enter to continue"
read
BIN="../../build/task01"
$BIN -cfgPath cfg_HelloWorld-1.xml &
$BIN -cfgPath cfg_HelloWorld-2.xml &
read && pkill -9 task01
