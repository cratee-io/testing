#!/bin/bash

workingDir=$(dirname ${BASH_SOURCE[0]})

cd ${workingDir}

remote="https://hub.fastgit.org/dtolnay/inventory"
# ensure this tag match the actually inventory in use
tag="0.1.10"
pkgDir=inventory
outDir=inventory-impl-for-testonly

rm -rf $pkgDir $outDir
git clone -b $tag $remote $pkgDir
mv $pkgDir/impl $outDir
rm -rf $pkgDir $outDir/LICENSE-*

sed -i "s!inventory::!testing::inventory::!g" $outDir/src/lib.rs
sed -i 's!name = "inventory-impl"!name = "inventory-impl-for-testonly"!g' $outDir/Cargo.toml
