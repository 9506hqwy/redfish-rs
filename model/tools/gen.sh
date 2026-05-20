#!/bin/bash

set -euo pipefail

DSP8010_VERSION='2026.1'
SWORDFISH_VERSION='1.2.8'
TOOLS_VERSION='0.3.0'

DSP8010_FILE="DSP8010_${DSP8010_VERSION}.zip"
# Cloudflare Bot Management prevents download file using curl.
# So, locate file at top directory before executing this script.
#DSP8010_URL="https://www.dmtf.org/sites/default/files/standards/documents/${DSP8010_FILE}"

SWORDFISH_FILE="Swordfish_v${SWORDFISH_VERSION}.zip"
# Cloudflare Bot Management prevents download file using curl.
# So, locate file at top directory before executing this script.
#SWORDFISH_URL="https://www.snia.org/sites/default/files/technical-work/swordfish/release/v${SWORDFISH_VERSION}/zip/${SWORDFISH_FILE}"

TOOLS_FILE="x86_64-unknown-linux-gnu.tar.gz"
TOOLS_URL="https://github.com/9506hqwy/openapi-spec-rs/releases/download/${TOOLS_VERSION}/${TOOLS_FILE}"

OUTPUT_DIR="$(dirname $0)/../src"
OUTPUT_DIR=$(cd "${OUTPUT_DIR}"; pwd)

WORK_DIR=$(mktemp -d)
trap 'rm -rf ${WORK_DIR}' EXIT

pushd "${WORK_DIR}"

# redfish
mkdir ./redfish
mkdir -p ./spec/schemas/v1
#curl -sSLO "${DSP8010_URL}"
cp "${OLDPWD}/${DSP8010_FILE}" .
unzip "${DSP8010_FILE}" -d ./redfish
mv ./redfish/*/openapi/* ./spec/schemas/v1

# sowrdfish
mkdir -p ./swordfish/schemas
mkdir -p ./spec/schemas/swordfish/v1
#curl -sSLO "${SWORDFISH_URL}"
cp "${OLDPWD}/${SWORDFISH_FILE}" .
unzip "${SWORDFISH_FILE}" -d ./swordfish
unzip "swordfish/Swordfish_v${SWORDFISH_VERSION}_Schema.zip" -d ./swordfish/schemas
mv ./swordfish/schemas/yaml/* ./spec/schemas/swordfish/v1

# Replace URI.
# Because the StorageSystemCollection's schema is difference in swordfish 1.2.7.
#  - Draft:   http://redfish.dmtf.org/schemas/swordfish/v1/StorageSystemCollection.yaml
#  - Release: http://redfish.dmtf.org/schemas/v1/StorageSystemCollection.yaml
# In swordfish 1.2.6, draft schema is same.
sed -i ./spec/schemas/swordfish/v1/openapi.yaml \
    -e 's|schemas/v1/StorageSystemCollection.yaml|schemas/swordfish/v1/StorageSystemCollection.yaml|'
# Because the Sensor_SensorLowerCriticalExcerpt was added v1.13.
# In redfish 2026.1, the Sensor_SensorLowerCriticalExcerpt indicates v1.9.3.
sed -i ./spec/schemas/v1/Sensor.yaml \
    -e 's|schemas/v1/Sensor.v1_9_3.yaml#/components/schemas/Sensor_v1_9_3_SensorLowerCriticalExcerpt|schemas/v1/Sensor.v1_13_0.yaml#/components/schemas/Sensor_v1_13_0_SensorLowerCriticalExcerpt|'

# tool
curl -sSLO "${TOOLS_URL}"
tar -zxf "${TOOLS_FILE}"
chmod +x openapi-spec-model

# Generating model.
rm -rf "${OUTPUT_DIR:?}"/*
./openapi-spec-model ./spec "${OUTPUT_DIR}"
pushd "${OUTPUT_DIR}"
cargo fmt --all
