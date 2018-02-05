#! /bin/bash

_SUDO=$1


# exit on non-zero return
set -e

LIBRARY_NAME="cppzmq"
LIBRARY_FOLDER_NAME="cppzmq"
SOURCE_ARCHIVE_FILE="v4.2.2.tar.gz"
SOURCE_ARCHIVE_ADDRESS="https://github.com/zeromq/cppzmq/archive/"
SOURCE_FOLDER_NAME="cppzmq-4.2.2"

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

$_SUDO cp ./zmq.hpp /usr/local/include/zmq.hpp

echo "Cleaning up..."
cd ${CWD}

# uncomment the following line to remove source code
#rm -rf ./${LIBRARY_FOLDER_NAME}

echo "Finished!"
