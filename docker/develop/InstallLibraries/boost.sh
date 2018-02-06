#! /bin/bash

_SUDO=$1


# exit on non-zero return
set -e

LIBRARY_NAME="boost"
LIBRARY_FOLDER_NAME="boost"
SOURCE_ARCHIVE_FILE="boost_1_64_0.tar.bz2"
SOURCE_ARCHIVE_ADDRESS="https://sourceforge.net/projects/boost/files/boost/1.64.0/"
SOURCE_FOLDER_NAME="boost_1_64_0"

ARCHIVE_COMMAND="tar xjvf "

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
#unzip -o ${SOURCE_ARCHIVE_FILE}
${ARCHIVE_COMMAND} ${SOURCE_ARCHIVE_FILE}

# change to the source directory
cd ${SOURCE_FOLDER_NAME}

echo "Building..."

./bootstrap.sh
./b2 --prefix=/usr/local --with-date_time --with-filesystem --with-regex --with-system link=static -d0 install

echo "Cleaning up..."
cd ${CWD}

# uncomment the following line to remove source code
#$_SUDO rm -rf ./${LIBRARY_FOLDER_NAME}

echo "Finished!"
