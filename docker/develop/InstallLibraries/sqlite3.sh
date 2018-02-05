#! /bin/bash

_SUDO=$1


# exit on non-zero return
set -e

LIBRARY_NAME="sqlite"
LIBRARY_FOLDER_NAME="sqlite"
SOURCE_ARCHIVE_FILE="sqlite-autoconf-3210000.tar.gz"
SOURCE_ARCHIVE_ADDRESS="https://sqlite.org/2017/"
SOURCE_FOLDER_NAME="sqlite-autoconf-3210000"

ARCHIVE_COMMAND="tar xzvf"

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

chmod +x configure
./configure --prefix=/usr/local \
            --disable-shared \
            --disable-dynamic-extensions \
            CPPFLAGS=-DSQLITE_ENABLE_COLUMN_METADATA
make -j8; make

echo "Installing..."
$_SUDO make install

echo "Cleaning up..."
cd ${CWD}

# uncomment the following line to remove source code
#rm -rf ./${LIBRARY_FOLDER_NAME}

echo "Finished!"
