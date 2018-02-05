#! /bin/bash

_SUDO=$1


# exit on non-zero return
set -e

LIBRARY_NAME="sqlitecpp"
LIBRARY_FOLDER_NAME="sqlitecpp"
SOURCE_ARCHIVE_FILE="1.3.1.tar.gz"
SOURCE_ARCHIVE_ADDRESS="https://github.com/SRombauts/SQLiteCpp/archive/"
SOURCE_FOLDER_NAME="SQLiteCpp-1.3.1"

ARCHIVE_COMMAND="tar xvf"

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

mkdir -p ./build
cd ./build
CXXFLAGS="-DSQLITE_OMIT_LOAD_EXTENSION" cmake  ..
make -j8; make
echo "Installing..."
$_SUDO cp ./libSQLiteCpp.a /usr/local/lib/
$_SUDO cp -rf ../include/SQLiteCpp /usr/local/include/

echo "Cleaning up..."
cd ${CWD}

# uncomment the following line to remove source code
#rm -rf ./${LIBRARY_FOLDER_NAME}

echo "Finished!"
