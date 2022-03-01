#!/usr/bin/env bash

set -e

TEMPLATES_DIR="$(dirname "$0")/templates"

# CMD="$(dirname "$0")/openapitools/openapi-generator-cli"
# export OPENAPI_GENERATOR_VERSION=6.0.0-SNAPSHOT
CMD="npx @openapitools/openapi-generator-cli"

echo "Generating SDKs..."

function generate() {

    OPENAPI_FILENAME="$1"
    shift
    PACKAGE_NAME="$1"
    shift
    PACKAGE_VERSION="$1"
    shift

    OUTPUT_PATH="packages/${PACKAGE_NAME}"

    echo "Generator version"
    ${CMD} version

    echo "Validating OpenAPI ${OPENAPI_FILENAME}"
    ${CMD} validate -i "${OPENAPI_FILENAME}"

    echo "Generating based on ${OPENAPI_FILENAME}"

    rm -Rf "$OUTPUT_PATH"

    ${CMD} generate -g rust -t "$TEMPLATES_DIR"  -i \
        "${OPENAPI_FILENAME}" -o "$OUTPUT_PATH" \
        --package-name="${PACKAGE_NAME}" \
        --additional-properties="packageVersion=${PACKAGE_VERSION},supportMultipleResponses=false,useSingleRequestParameter=false,licenseName=Apache-2.0,licenseUrl=https://www.apache.org/licenses/LICENSE-2.0.txt" \
        --ignore-file-override="$(dirname "$0")/../.openapi-generator-ignore"

    rm -Rf "$OUTPUT_PATH/.openapi-generator"
}

# generate ".openapi/connector-mgmt.yaml" "rhoas-connector-management-sdk" "0.0.1-alpha.1"
generate ".openapi/kafka-admin-rest.yaml" "rhoas-kafka-instance-sdk" "0.0.1-alpha.1"
generate ".openapi/kas-fleet-manager.yaml" "rhoas-kafka-management-sdk" "0.0.1-alpha.1"
# blocked by: https://github.com/OpenAPITools/openapi-generator/pull/11754
# generate ".openapi/registry-instance.json" "rhoas-registry-instance-sdk" "0.0.1-alpha.1"
generate ".openapi/srs-fleet-manager.json" "rhoas-registry-management-sdk" "0.0.1-alpha.1"
