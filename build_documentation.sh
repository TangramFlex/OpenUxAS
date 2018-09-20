#!/bin/bash
# Copyright © 2017 Government of the United States of America, as represented by the Secretary of the Air Force.
# No copyright is claimed in the United States under Title 17, U. S. Code. All Other Rights Reserved.
# Copyright 2017 University of Cincinnati. All rights reserved. See LICENSE.md file at:
# https://github.com/afrl-rq/OpenUxAS
# Additional copyright may be held by others, as reflected in the commit history.

set -e

# from the README.md, 2017-05-08:

echo "Installing dependencies..."

# references:
# * http://stackoverflow.com/questions/3466166/how-to-check-if-running-in-cygwin-mac-or-linux/17072017#17072017
# * https://serverfault.com/questions/501230/can-not-seem-to-get-expr-substr-to-work

if [ "$(uname)" == "Darwin" ]; then
    # Install doxygen and related packages: in terminal
    brew install doxygen
    brew install graphviz
    brew cask install mactex
    # Install firefox, sed, evince (for pdf viewing)
    brew install firefox sed evince
elif [ "$(expr substr $(uname -s) 1 5)" == "Linux" ]; then
    if [ -n "$(which apt 2>/dev/null)" ]; then
    # Install doxygen and related packages: in terminal
    sudo apt -y install doxygen
    sudo apt -y install graphviz
    sudo apt -y install texlive-full
    sudo apt -y install texlive-latex-extra
    # Install sed, evince (for pdf viewing)
    sudo apt -y install sed evince
    fi
    if [ -n "$(which dnf 2>/dev/null)" ]; then
    sudo dnf -y install doxygen graphviz texlive texlive-latex texlive-tufte-latex texlive-tabu texlive-hardwrap texlive-multirow texlive-titlesec texlive-adjustbox texlive-import texlive-sectsty texlive-tocloft
    sudo dnf -y install zathura zathura-pdf-poppler
    fi
else
    echo "Unsupported platform! Script install only for Linux and Mac"
    exit 1
fi

echo "Generating User Manual..."
# run this at: ./OpenUxAS/doc/reference/UserManual
cd ./doc/reference/UserManual
pdflatex UxAS_UserManual.tex

echo "Creating HTML Doxygen reference documentation..."
# run this at: ./OpenUxAS/doc/doxygen
cd ../../doxygen
sh RunDoxygen.sh

echo "Creating Doxygen PDF reference manual (post-RunDoxygen.sh run)..."
# run this at: ./OpenUxAS/doc/doxygen
cd ./
HOLDSTR=`cat ./ExtraLineToFixLatex.txt`
# ref: http://stackoverflow.com/questions/13210880/replace-one-substring-for-another-string-in-shell-script
HOLDSTR2="${HOLDSTR//\\/\\\\}" # need \\ for every \ for sed; "replace every instance" by // instead of /
sed -i.orig "s/%===== C O N T E N T S =====/${HOLDSTR2}\n%===== C O N T E N T S =====/" ./latex/refman.tex

# run this at: ./OpenUxAS/doc/doxygen/latex
cd ./latex
pdflatex refman.tex

echo "...Congratulations! You're done with building the documentation!"

# --eof--
