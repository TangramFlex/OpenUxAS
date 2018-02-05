#! /bin/bash

_SUDO=$1


# exit on non-zero return
set -e

LIBRARY_NAME="minizip"
LIBRARY_FOLDER_NAME="minizip"
SOURCE_ARCHIVE_FILE="1.2.tar.gz"
SOURCE_ARCHIVE_ADDRESS="https://github.com/nmoinvaz/minizip/archive/"
SOURCE_FOLDER_NAME="minizip-1.2"

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
cmake -DUSE_AES=OFF ..
make -j8; make
echo "Installing..."
$_SUDO make install

echo "Cleaning up..."
cd ${CWD}

# uncomment the following line to remove source code
#rm -rf ./${LIBRARY_FOLDER_NAME}

echo "Finished!"
