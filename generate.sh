#!/usr/bin/env bash
set -e

CHIPTOOL_REV=d5ec99b8016e9e0f292d69fd14dcf9f00ec4e057

top_dir="$(cd -P -- "$(dirname -- "$0")" && pwd -P)"
crate_dir="${top_dir%%/}/output"
crate_src_dir="${crate_dir%%/}/src"

cargo install --root .cargo-root --git https://github.com/embassy-rs/chiptool.git --rev $CHIPTOOL_REV
chiptool="${top_dir%%/}/.cargo-root/bin/chiptool"

echo "Generating sources"
for i in svds/*.svd; do
    device=`basename -- $i`
    device=${device%.*}
    crate_device_dir="${crate_src_dir%%/}/${device,,}"

    echo "  $device"

    mkdir -p $crate_device_dir
    cd $crate_device_dir
    $chiptool generate --svd "${top_dir%%/}/svds/$device.svd" --transform "${top_dir%%/}/transforms/$device.yaml"
done
cd $top_dir

echo "Copying crate files"
if version=`git describe 2>/dev/null`; [ $? -ne 0 ]; then
    version="0.0.0-nightly"
fi
sed "s/__VERSION__/$version/" src/Cargo.toml > "${crate_dir%%/}/Cargo.toml"
cp src/build.rs $crate_dir
cp src/common.rs $crate_src_dir
cp src/lib.rs $crate_src_dir
for i in svds/*.svd; do
    device=`basename -- $i`
    device=${device%.*}
    device=${device,,}
    echo "#[cfg_attr(feature = \"$device\", path = \"$device/lib.rs\")]" >> ${crate_src_dir%%/}/lib.rs
    echo "$device = [\"_device_selected\"]" >> ${crate_dir%%/}/Cargo.toml
done
echo "mod device;" >> ${crate_src_dir%%/}/lib.rs
echo "#[cfg(feature = \"_device_selected\")]" >> ${crate_src_dir%%/}/lib.rs
echo "pub use device::*;" >> ${crate_src_dir%%/}/lib.rs
touch ${crate_src_dir%%/}/device.rs
cp .gitignore $crate_dir

echo "Formatting output crate"
cargo fmt --manifest-path ${crate_dir%%/}/Cargo.toml
