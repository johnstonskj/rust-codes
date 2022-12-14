#!/usr/bin/env sh

LAGENCY=$(echo $1 | tr A-Z a-z)
LNAME=$(echo $2 | tr A-Z a-z)
PACKAGE="codes-${LAGENCY}-${LNAME}"

UAGENCY=$(echo $1 | tr a-z A-Z)
UNAME=$(echo $2 | tr a-z A-Z)
PACKAGE_NAME="${UAGENCY} ${UNAME}"

echo "Creating new cargo package $PACKAGE"

cargo init --lib $PACKAGE

echo "Adding README"

cat <<EOT >> $PACKAGE/README.md
# Package ${PACKAGE}

This package contains an implementation of the
[${NAME}](...) specification.

[![crates.io](https://img.shields.io/crates/v/${PACKAGE}.svg)](https://crates.io/crates/${PACKAGE})
[![docs.rs](https://docs.rs/${PACKAGE}/badge.svg)](https://docs.rs/${PACKAGE})

For notes on the design of the API, see the repository 
[README](https://github.com/johnstonskj/rust-codes/blob/main/README.md).

## Example

TBD

## Features

## Changes

**Version 0.1.0**

## TODO

TBD
EOT

echo "Updating Cargo.toml"

cat <<EOT >> $PACKAGE/Cargo.toml
codes-agency = { version = "0.1", path = "../codes-agency" }
codes-common = { version = "0.1", path = "../codes-common" }
serde = { version = "1.0", features = ["derive"], optional = true }

[build-dependencies]
codes-common = { version = "0.1", path = "../codes-common" }
tera = "1.17"

[package.metadata.docs.rs]
targets = ["x86_64-unknown-linux-gnu"]
all-features = true

[features]
default = ["serde"]

EOT

echo "Copying template files"
cp templates/lib.rs $PACKAGE/src/lib.rs
cp templates/build.rs $PACKAGE/build.rs

mkdir $PACKAGE/templates
cp templates/lib._rs $PACKAGE/templates/lib._rs
 
echo "Creating data directory and refresh script"
mkdir $PACKAGE/data

cat <<EOT >> $PACKAGE/data/refresh.sh
#!/usr/bin/env sh

curl -o file_name "url"
EOT

chmod 755 $PACKAGE/data/refresh.sh
