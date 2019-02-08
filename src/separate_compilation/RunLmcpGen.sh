#! /bin/bash -e

HERE=$PWD;

DIRECTORY="../../../LmcpGen"

if [ -d "${DIRECTORY}" ]; then

	# build lmcpgen
	cd ${DIRECTORY}
	echo " ** building lmcpgen **"
	ant -q jar
	echo " ** finished building lmcpgen **"
	cd ${HERE}

	# auto-create c++ libraries
	echo ""
	echo " ** processing mdms **"
	# LmcpGen writes generated assets to the path given by -dir, but does
	# not clear assets from previous runs. This seems wrong.
	rm -rf "$HERE/LMCP"
	java -Xmx2048m -jar ../../../LmcpGen/dist/LmcpGen.jar -cpp -dir "$HERE/LMCP"
	###java -Xmx2048m -jar ../../../LmcpGen/dist/LmcpGen.jar -mdmdir "$HERE/mdms" -cpp -dir "$HERE/LMCP"

	# LmcpGen writes meson.build; we must edit for use in disaggregation.
	# When disaggregation works, these edits should be done within LmcpGen.
	echo " ** editing LMCP/meson.build **"
	ed -v $HERE/LMCP/meson.build <<EOF
g/srcs_lmcp/s/srcs_lmcp/srcs_lmcp_task01/g
g/incs_lmcp/s/incs_lmcp/incs_lmcp_task01/g
g/cpp_args_lmcp/s/cpp_args_lmcp/cpp_args_lmcp_task01/g
g/lib_lmcp/s/lib_lmcp/lib_lmcp_task01/g
/'lmcp'/s/'lmcp'/'lmcp_task01'/
wq
EOF
	echo " ** finished processing mdms **"

else
	echo "ERROR: LmcpGen must be present!!!"
	exit 1
fi
