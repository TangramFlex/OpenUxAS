# Source this script using a POSIX-compatible shell (bash, dash, sh, ...)
# to set PATH for UxAS development.
export PYTHONUSERBASE=`pwd`/toolroot
export PATH=$PYTHONUSERBASE/bin:$PATH
export PATH=$PATH:`pwd`/experimental
