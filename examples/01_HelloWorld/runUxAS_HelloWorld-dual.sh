#! /bin/bash

nokill () {
	echo
	echo
	echo
	echo "*** PRESS ENTER TO TERMINATE ***"
	echo
	echo
}
firewall_off () {
	if which ufw >/dev/null 2>&1; then
		sudo ufw disable
	elif which systemctl 2>&1 >/dev/null; then
		sudo systemctl stop firewalld
	else
		echo "Unknown firewall"
		exit 1
	fi
	echo "Firewall stopped"
}
firewall_on () {
	if which ufw >/dev/null 2>&1; then
		sudo ufw enable
	elif which systemctl 2>&1 >/dev/null; then
		sudo systemctl restart firewalld
	else
		echo "Unknown firewall"
		exit 1
	fi
	echo "Firewall started"
}
echo "NOTICE: Running HelloWorld dual example from task01 executable."
echo "You'll be prompted by sudo for your password to temporarily"
echo "disable the firewall."
echo
echo "Press enter to start the demo."
echo "When running, press Enter to stop."
read
firewall_off
sleep 1
BIN="../../build/task01"
$BIN -cfgPath cfg_HelloWorld-1.xml 2>&1 |tee /dev/tty >hello-1.log &
$BIN -cfgPath cfg_HelloWorld-2.xml 2>&1 |tee /dev/tty >hello-2.log &
trap nokill INT
read && { pkill -9 task01; firewall_on; }
