#!/bin/bash
# Copyright © 2017 Government of the United States of America, as represented by the Secretary of the Air Force.
# No copyright is claimed in the United States under Title 17, U. S. Code. All Other Rights Reserved.
# Copyright 2017 University of Cincinnati. All rights reserved. See LICENSE.md file at:
# https://github.com/afrl-rq/OpenUxAS
# Additional copyright may be held by others, as reflected in the commit history.

set -e

# from the README.md, 2017-05-11:


# references:
# * http://stackoverflow.com/questions/3466166/how-to-check-if-running-in-cygwin-mac-or-linux/17072017#17072017
# * https://serverfault.com/questions/501230/can-not-seem-to-get-expr-substr-to-work

# Confirm shell
[ "`ps -o comm= $$`" = bash ] || { echo "`ps -o comm= $$` is not bash"; exit 1; }

# Confirm non-root user
[ $USER = root ] && { echo "Do not run this script as root"; exit 1; }

# Preauthorize sudo
sudo -k && sudo -v || { echo "sudo not authenticated"; exit 1; }
while true; do sudo -n true; sleep 60; kill -s 0 $$ || exit; done 2>/dev/null &


if [ "$(uname)" == "Darwin" ]; then
    echo "The Mac/Darwin portion of this script is out of date."
    echo "Please update using the Fedora section as a guide."
    exit 1
    ## FIXME:
    echo "Install Prerequisites on Mac OS X"
    echo " "
    echo "Install XCode"
    echo "* Get yourself a developer account and grab the file from: https://developer.apple.com/xcode/"
    echo " (This cannot be downloaded automatically due to the need to agree to license &etc. terms.)"
    echo " (So, download from website manually and install the .dmg file.)"
    echo "Once you've done this..."
    echo "Press any key to continue..."
    # as of 2017-05-08, this is: ????.dmg
    # ref: https://superuser.com/questions/689315/run-safari-from-terminal-with-given-url-address-without-open-command
    # ref: https://www.macissues.com/2014/09/18/how-to-launch-and-quit-applications-in-os-x-using-the-terminal/
    /Applications/Safari.app/Contents/MacOS/Safari & sleep 1 && osascript -e 'tell application "Safari" to open location "https://developer.apple.com/xcode/"'
    #echo "* Install .dmg"
    read -rs -p " " -n 1 # reference: https://ss64.com/bash/read.html
    echo " "
    # Enable commandline tools: in terminal
    xcode-select --install
    # Install homebrew (must be administrator): in terminal
    sudo ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)"
    # Add homebrew to path: in terminal
    echo `export PATH="/usr/local/bin:$PATH"` >> ~/.bash_profile
    source ~/.bash_profile # bash
    brew tap caskroom/cask
    # Install git: in terminal
    brew install git
    # Install unique ID library: in terminal
    brew install ossp-uuid
    # Install Boost library and configure it in a fresh shell: in terminal
    brew install boost
    echo 'export BOOST_ROOT=/usr/local' >> ~/.bash_profile
    source ~/.bash_profile # bash
    # Install pip3: in terminal
    brew install python3
    curl -O https://bootstrap.pypa.io/get-pip.py
    sudo -H python3 get-pip.py
    # Install ninja build system: in terminal
    brew install cmake
    brew install pkg-config
    sudo -H pip3 install scikit-build
    sudo -H pip3 install ninja
    # Install meson build configuration: in terminal
    sudo -H pip3 install meson==0.42.1
    # Install python plotting capabilities (optional): in terminal
    sudo -H pip3 install matplotlib
    sudo -H pip3 install pandas
    # Install Oracle JDK
    brew cask install java
    # Install ant for command line build of java programs
    brew install ant
    echo "Dependencies installed!"
    
elif [ "$(expr substr $(uname -s) 1 5)" == "Linux" ]; then
    if [ -n "$(which apt 2>/dev/null)" ]; then
    echo "Installing Prerequisite Tools on Ubuntu Linux"
    # run an 'apt update' check without sudo
    # ref: https://askubuntu.com/questions/391983/software-updates-from-terminal-without-sudo
    ##aptdcon --refresh
    NUMBER_UPGRADEABLE=`apt-get -s upgrade | grep "upgraded," | cut -d' ' -f1`
    if [ $NUMBER_UPGRADEABLE -gt 0 ]
    then
        echo "Some packages require updating, running apt update-upgrade as sudo now..."
        sudo apt -y update
        sudo apt -y upgrade
        echo "Done with apt update-upgrade!"
    fi

    # Install pkg-config for finding link arguments
    sudo apt -y install pkg-config
    # Install git
    sudo apt -y install git
    sudo apt -y install gitk
    # Install ed
    sudo apt -y install ed
    # Install opengl development headers
    sudo apt -y install libglu1-mesa-dev
    # Install unique ID creation library
    sudo apt -y install uuid-dev
    # Install Boost libraries (**optional but recommended**; see external dependencies section)
    sudo apt -y install libboost-filesystem-dev libboost-regex-dev libboost-system-dev
    # Install minizip
    sudo apt -y install libminizip-dev
    # Install pip3
    sudo apt -y install python3-pip
    ##sudo -H pip3 install --upgrade pip
    # Install tkinter
    sudo apt -y install python3-tk
    # Install ant for command line build of java programs
    sudo apt -y install ant
    # We probably have xterm; be certain.
    sudo apt -y install xterm
    # Install ninja build system
    sudo -H pip3 install ninja
    # Install meson build configuration
    sudo -H pip3 install meson==0.42.1
    # Install python plotting capabilities (optional)
    sudo -H pip3 install matplotlib
    sudo -H pip3 install pandas
    # Install Oracle JDK
    echo oracle-java8-installer shared/accepted-oracle-license-v1-1 select true | sudo /usr/bin/debconf-set-selections
    sudo add-apt-repository -y ppa:webupd8team/java
    sudo apt -y update
    sudo apt -y install oracle-java8-installer
    sudo apt -y install oracle-java8-set-default

    # Extend paths for pip --user option
    source ./path.sh

    # Pinning meson is understandable for now; the API is in flux.
    [ -n "$PYTHONUSERBASE" ] || { echo "PYTHONUSERBASE is not set"; exit 1; }
    mkdir -p $PYTHONUSERBASE
    pip3 install --user meson==0.42.1
    fi  # have apt; must be Ubuntu

    # IMPORTANT: This Fedora installation is the model for all other platforms.
    if [ -n "$(which dnf 2>/dev/null)" ]; then
    echo "Installing Prerequisite packages on Fedora Linux"
    # These should be the same packages (perhaps with different names) as above
    sudo dnf -y install pkgconf git gitk ed mesa-libGLU-devel uuid-devel \
        boost-devel python3-pip python3-tkinter ant xterm redhat-rpm-config \
        gcc-c++ python3-devel ninja-build python3-matplotlib python3-pandas \
        java-1.8.0-openjdk-devel minizip-devel

    # Extend paths for pip --user option
    source ./path.sh

    # Pinning meson is understandable for now; the API is in flux.
    [ -n "$PYTHONUSERBASE" ] || { echo "PYTHONUSERBASE is not set"; exit 1; }
    mkdir -p $PYTHONUSERBASE
    pip3 install --user meson==0.42.1
    fi  # have dnf; must be Fedora

    echo "Dependencies installed!"

else
    echo "Unsupported platform! Only Ubuntu Linux and Mac OSX supported"
    exit 1
fi

echo "Configuring UxAS"
#check to see if already in OpenUxAS
current_directory=${PWD##*/}
git_directory=$PWD'/.git'
if [ $current_directory != "OpenUxAS" ] || [ ! -d $git_directory ]; then
    echo "Checking out OpenUxAS ..."
    git clone -b develop --single-branch https://github.com/afrl-rq/OpenUxAS.git
fi

# ensure one directory above OpenUxAS
if [ $current_directory == "OpenUxAS" ] && [ -d $git_directory ]; then
   cd ..
fi

echo "Checking out LmcpGen ..."
rm -rf LmcpGen
git clone https://github.com/afrl-rq/LmcpGen.git
cd LmcpGen
ant -q jar
cd ..
echo "Checking out OpenAMASE ..."
rm -rf OpenAMASE
git clone https://github.com/afrl-rq/OpenAMASE.git
cd OpenAMASE/OpenAMASE
ant -q jar
cd ../..
    
echo "Configuring UxAS plotting utilities ..."
cd OpenUxAS/src/Utilities/localcoords
sudo python3 setup.py install
cd ../../..

echo "Preparing UxAS build ..."
rm -rf build_release build_debug
python3 prepare
sh RunLmcpGen.sh
( cd src/separate_compilation && sh RunLmcpGen.sh )
meson build_release --buildtype=release
meson build_debug --buildtype=debug

echo "Performing initial UxAS build ..."
ninja -C build_debug
ninja -C build_release

# Alias the debug build
ln -s build_debug build

cat <<'EOF'

================================================================

DONE!

Subsequent builds are done using:

  $ ninja -C build_debug

and 

  $ ninja -C build_release
EOF

# --eof--
