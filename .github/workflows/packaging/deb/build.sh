#!/bin/bash
# create a deb package from rust sources
#
############ LIST OF MANAGED VARIABLES REQUIRED FOR DEB PACKAGE ############
name=cma-server
# version=x.y.z - reading from first arg $1
descriptionShort="CMA Server | Handling data on fly"
descriptionExtended="CMA Server | Handling data on fly.
- Designed to collect any kind of data from connected devices via any kind of communication protocols
- Make some declarative computation on data coming from connected devices
- Share collected / computed data to the clients
- Stores collected / computed data to the disk / database"
changeDetails="
- + TCP Server - Shares data with clients over tcp connection
	- Auth - simple password authentication
	- Points - request configurations of all configured points
	- Subscibe - subscriptiopn on the list of points to be received on changed
- + API Client
	- performs all received SQL's to the configured database
	- buffering received SQL's
	- trying resend failed requests
- + CMA Client
	- communication with similar application via TCP
- + Profinet Client
	- communication with Profined (Siemens) devices
	- read DB's
	- write commands
- + Task
	- Declarative configurable computation
	- Event based execution
	- Timer based execution
"
copyrightNotice="Copyright 2024 anton lobanov"
maintainer="anton lobanov <lobanov.anton@gmail.com>"
licenseName="GNU GENERAL PUBLIC LICENSE v3.0"
licenseFile="LICENSE"

############ LIST OF MANAGED VARIABLES OPTIONAL FOR DEB PACKAGE ############
#
# list of requared working directories
# 	<installPath> <permissions>
workingDirs=(
	"/home/scada/cma-server/ 777"
)
#
# preinst, postinst, prerm and postrm scripts:
preinst="./.github/workflows/packaging/deb/preinst"
postinst="./.github/workflows/packaging/deb/postinst"
prerm="./.github/workflows/packaging/deb/prerm"
postrm="./.github/workflows/packaging/deb/postrm"
# list of assets in the format:
# 	<sourcePath> <installPath> <permissions>
assets=(
	"./target/release/cma-server /usr/bin/ 755"
	"./.github/workflows/packaging/deb/service/cma-server.service /etc/systemd/system/ 644"
	"./libsnap7.so /home/scada/cma-server/lib/ 755"
	"./config.yaml /home/scada/cma-server/ 777"
)
outputDir=target/
# 'any', 'all' or one of the supported architecture (e.g., 'amd64', 'arm64', 'i386', 'armhf')
# you can choose one of the provided by `dpkg-architecture -L` or leave the field blank for automatic detection
arch=
# comma separated list of the package dependecies in the following format:
# "<package_name> [(<<|>>|<=|>=|= <version>)], ..."
# e.g. "foo (>=2.34), bar"
depends=""

############ READING VERSION FROM ARGUMENT ############
RED='\033[0;31m'
YELLOW='\033[1;93m'
NC='\033[0m' # No Color
version=$1
if [[ "$version" =~ [0-9]+\.[0-9]+\.[0-9]+ ]]; then 
	echo "Version: $version"
else
	echo -e "${RED}ERROR${NC}: Version not supplied.\nDebian package build script required proper version of your softvare in the format: x.y.z, passed as argument"
fi

# check required variables
echo "Checking reqired variables ..."
missedVarMsg="non-empty value required"
echo "${!name@}=${name:?$missedVarMsg}"
echo "${!version@}=${version:?$missedVarMsg}"
echo "${!descriptionShort@}=${descriptionShort:?$missedVarMsg}"
echo "${!descriptionExtended@}=${descriptionExtended:?$missedVarMsg}"
echo "${!changeDetails@}=${changeDetails:?$missedVarMsg}"
echo "${!copyrightNotice@}=${copyrightNotice:?$missedVarMsg}"
echo "${!maintainer@}=${maintainer:?$missedVarMsg}"
echo "${!licenseName@}=${licenseName:?$missedVarMsg}"
echo "${!licenseFile@}=${licenseFile:?$missedVarMsg}"

echo "Start packaging ..."

############ INITIALIZE THE PACKAGE SOURCE STRUCTURE AND COPY RESOURCES ############

arch=${arch:=$(dpkg --print-architecture)}
debFileName="${name}_${version}_${arch}"
packageRoot=$(readlink -m "/tmp/debian/${debFileName}")

if [[ -d $packageRoot ]]; then
	echo "Freeing the directory for temporary build files ..."
	rm -rf $packageRoot
fi

echo "Creating ${packageRoot} directory for temporary build files ..."
mkdir -p "$packageRoot"
echo "Creating ${packageRoot}/DEBIAN directory ..."
mkdir -p "${packageRoot}/DEBIAN"

createDir() {
	targetDir=$1; permissions=$2;
	installPath=$(readlink -m "${packageRoot}/${targetDir}")
	echo "Creating dir '${installPath}' ..."
	mkdir -p $installPath
	if [[ -d $installPath ]]; then
		echo "Applying permissions ${permissions} to dir ${installPath} ..."
		chmod -R "$permissions" "$installPath"
	else
		echo "${RED}Can't apply permissions ${permissions} to '${installPath}' ..."
	fi
}
copyAsset() {
	sourcePath=$1; targetDir=$2; permissions=$3
	assetPath=$(readlink -m "$sourcePath")
	if [[ ! -d $assetPath && ! -f $assetPath ]]; then
		echo "Asset ${assetPath} not found."
		exit 1
	fi
	installPath=$(readlink -m "${packageRoot}/${targetDir}")
	echo "Copying ${assetPath} to ${installPath} ..."
	mkdir -p $installPath && cp -r "$assetPath" "$installPath"
	if [[ -d $assetPath ]]; then
		echo "Applying permissions ${permissions} to dir ${installPath} ..."
		chmod -R "$permissions" "$installPath"
	elif [[ -f $assetPath ]]; then
		echo "Applying permissions ${permissions} to file ${installPath} ..."
		chmod "$permissions" "${installPath}/$(basename ${assetPath})"
	else
		echo "${RED}Unknown asset type, can't apply permissions ${permissions} to file${NC} ${installPath} ..."
	fi
}
for dir in "${workingDirs[@]}"; do
	read -ra dirOptions <<< $dir
	createDir ${dirOptions[0]} ${dirOptions[1]}
done
for asset in "${assets[@]}"; do
	read -ra assetOptions <<< $asset
	copyAsset ${assetOptions[0]} ${assetOptions[1]} ${assetOptions[2]}
done
copyAsset ${preinst} "DEBIAN" "755"
copyAsset ${postinst} "DEBIAN" "755"
copyAsset ${prerm} "DEBIAN" "755"
copyAsset ${postrm} "DEBIAN" "755"

############ CREATE A DEB CONTROL FILE ############

echo "Creating ${packageRoot}/DEBIAN/control file ..."
cat > "${packageRoot}/DEBIAN/control" <<- CONTROL
	Section: misc
	Priority: optional
	Version: $version
	Maintainer: $maintainer
	Package: $name
	Architecture: $arch
	Depends: $depends
	Description: $descriptionShort
	$(echo "$descriptionExtended" | sed "s/^/ /")
CONTROL

############ CREATE CHANGELOG AND COPYRIGHT FILES ############

docDir="${packageRoot}/usr/share/doc/${name}"
mkdir -p "$docDir"

echo "Generating changelog file ..."
changelogFile="${docDir}/changelog"
cat > "$changelogFile" <<- CHANGELOG
	$name ($version) unstable; urgency=medium

	$(echo "$changeDetails" | sed "s/^/  * /")

	$(echo " -- $maintainer  $(date -R)")


CHANGELOG
gzip -n --best "$changelogFile"
rm -f "$changelogFile"

echo "Generating copyright file ..."
copyrightFile="${docDir}/copyright"
cat > "$copyrightFile" <<- COPYRIGHT
	Format: https://www.debian.org/doc/packaging-manuals/copyright-format/1.0/
	Upstream-Name: $name
	Copyright: $copyrightNotice
	License: $licenseName
	$(cat "$licenseFile" | sed "s/^/ /")
COPYRIGHT

############ CREATE MD5 SUM FILES ############

cd $packageRoot
md5sum $(find . -type f -printf "%P\n" | grep -v "^DEBIAN/") > DEBIAN/md5sums
cd - > /dev/null

############ BUILD A DEB PACKAGE ############
echo "Building deb package ..."
# -Zxz - to to change the compression method from zstd to xz (zstd - supported since debian 12)
dpkg-deb -Zxz --build "${packageRoot}" "$outputDir" > /dev/null || exit 1 
echo "Deleting temporary created ${packageRoot} directory"
rm -rf "${packageRoot}"
echo "Debian package created and saved in $(readlink -m "${outputDir}/${debFileName}.deb")"