#! /bin/bash

echo "NOTICE: Running HelloWorld dual example from task01 executable."
echo "Press enter to start the demo."
echo "When running, press Enter to stop."
read
BIN="../../build/task01"
$BIN -cfgPath cfg_HelloWorld-1.xml |tee /dev/tty >hello-1.log &
$BIN -cfgPath cfg_HelloWorld-2.xml |tee /dev/tty >hello-2.log &
trap 'echo **** PRESS ENTER TO TERMINATE ****' INT
read && pkill -9 task01
