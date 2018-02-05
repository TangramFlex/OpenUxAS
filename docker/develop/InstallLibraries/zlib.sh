#! /bin/bash

_SUDO=$1


# exit on non-zero return
set -e

LIBRARY_NAME="zlib"
LIBRARY_FOLDER_NAME="zlib"
SOURCE_ARCHIVE_FILE="v1.2.8.tar.gz"
SOURCE_ARCHIVE_ADDRESS="https://github.com/madler/zlib/archive/"
SOURCE_FOLDER_NAME="zlib-1.2.8"

echo "Making Dirs"
CWD=$(pwd)
mkdir -p ./${LIBRARY_FOLDER_NAME}
cd ./${LIBRARY_FOLDER_NAME}

if [ -f ${SOURCE_ARCHIVE_FILE} ]
then
	echo "*** "${LIBRARY_NAME}":: Archive File ("${SOURCE_ARCHIVE_FILE}") Exists, Skipping Source Fetch! ***"
else
	echo "Fetching Source"
	wget ${SOURCE_ARCHIVE_ADDRESS}${SOURCE_ARCHIVE_FILE}
fi

echo "Unpacking..."
tar xvf ${SOURCE_ARCHIVE_FILE}

# change to the source directory
cd ${SOURCE_FOLDER_NAME}

echo "Building..."

mkdir -p ./build
cd ./build
cmake ..
make -j8; make
echo "Installing..."
$_SUDO make install
$_SUDO rm /usr/local/lib/libz.so*

echo "Cleaning up..."
cd ${CWD}

# uncomment the following line to remove source code
#rm -rf ./${LIBRARY_FOLDER_NAME}

echo "Finished!"
