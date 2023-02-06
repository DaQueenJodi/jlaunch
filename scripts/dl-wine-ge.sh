#!/bin/bash
# deps: tar (with xz support) and wget


URL="${1}"
OUTDIR="${2}/wine-ge"
TEMPDIR="/tmp/wine-ge"
mkdir -p "${TEMPDIR}"
mkdir -p "${OUTDIR}"
URLDISP=$(echo "${URL}" | sed -E 's|.*/(.*.tar.xz)|\1|')
echo "Downloading ${URLDISP}..."
wget --quiet --output-document "${TEMPDIR}/wine-ge.tar.xz" "${URL}"
echo "done!"
echo "decompressing tarball..."
# ignore the first directory since its just a container directory
tar --extract --xz --strip 1\
	--directory "${OUTDIR}" \
	--file "${TEMPDIR}/wine-ge.tar.xz"
echo "done!"
