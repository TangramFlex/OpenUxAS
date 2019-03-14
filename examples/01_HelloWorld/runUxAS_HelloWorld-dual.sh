#! /bin/bash

echo "NOTICE: Running HelloWorld dual example from task01 executable."
echo "You'll be prompted by sudo for your password to temporarily"
echo "disable the firewall."
echo
echo "Press enter to start the demo."
echo "When running, press Enter to stop."
read
sudo systemctl stop firewalld
echo firewall stopped
sleep 1
BIN="../../build/task01"
$BIN -cfgPath cfg_HelloWorld-1.xml 2>&1 |tee /dev/tty >hello-1.log &
$BIN -cfgPath cfg_HelloWorld-2.xml 2>&1 |tee /dev/tty >hello-2.log &
trap 'echo **** PRESS ENTER TO TERMINATE ****' INT
read && { pkill -9 task01; sudo systemctl restart firewalld; }
