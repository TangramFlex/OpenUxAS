The `./runUxAS_HelloWorld-dual.sh` script runs HelloWorld in two separate
OpenUxAS instances. The instances exchange messages via a Zyre bridge.

This demo assumes the following; these may not match your platform:

  1) the presence of an active firewall which is not configured to
     pass Zyre/0MQ traffic,
  2) a systemd-based distro (see the script), and
  3) a network device named "enp0s20f0u4u1" (see `cfg_HelloWorld-?.xml`).

The script disables the firewall, runs the demo and then restarts the
firewall. Note the instructions printed when the script starts.
