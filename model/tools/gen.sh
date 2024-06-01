#!/bin/bash

set -eu

DSP8010_VERSION='2023.3'
SWORDFISH_VERSION='1.2.5a'
TOOLS_VERSION='0.2.0'

DSP8010_FILE="DSP8010_${DSP8010_VERSION}.zip"
DSP8010_URL="https://www.dmtf.org/sites/default/files/standards/documents/${DSP8010_FILE}"

SWORDFISH_FILE="Swordfish_v${SWORDFISH_VERSION}.zip"
SWORDFISH_URL="https://www.snia.org/sites/default/files/technical-work/swordfish/release/v${SWORDFISH_VERSION}/zip/${SWORDFISH_FILE}"

TOOLS_FILE="x86_64-unknown-linux-gnu.tar.gz"
TOOLS_URL="https://github.com/9506hqwy/openapi-spec-rs/releases/download/${TOOLS_VERSION}/${TOOLS_FILE}"

OUTPUT_DIR="$(dirname $0)/../src"
OUTPUT_DIR=$(cd $OUTPUT_DIR; pwd)

WORK_DIR=`mktemp -d`
trap 'rm -rf ${WORK_DIR}' EXIT

pushd "${WORK_DIR}"

# redfish
mkdir ./redfish
mkdir -p ./spec/schemas/v1
curl -sSLO "${DSP8010_URL}"
unzip "${DSP8010_FILE}" -d ./redfish
mv ./redfish/openapi/* ./spec/schemas/v1

# sowrdfish
mkdir -p ./swordfish/schemas
mkdir -p ./spec/schemas/swordfish/v1
curl -sSLO "${SWORDFISH_URL}"
unzip "${SWORDFISH_FILE}" -d ./swordfish
unzip "swordfish/Swordfish_v${SWORDFISH_VERSION}_Schema.zip" -d ./swordfish/schemas
mv ./swordfish/schemas/yaml/* ./spec/schemas/swordfish/v1

# tool
curl -sSLO "${TOOLS_URL}"
tar -zxf "${TOOLS_FILE}"
chmod +x openapi-spec-model

# Generating model.
rm -rf "${OUTPUT_DIR}"/*
./openapi-spec-model ./spec "${OUTPUT_DIR}"
pushd "${OUTPUT_DIR}"
cargo fmt --all
