# Source this script using a POSIX-compatible shell (bash, dash, sh, ...)
# to set PATH for UxAS development.
export PYTHONUSERBASE=`pwd`/toolroot
[[ "$PATH" = *$PYTHONUSERBASE/bin:* ]] || export PATH=$PYTHONUSERBASE/bin:$PATH
[[ "$PATH" = *:`pwd`/experimental* ]] || export PATH=$PATH:`pwd`/experimental
