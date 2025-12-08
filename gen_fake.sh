#!/bin/bash

# Default values
OUT_PATH=""
OTHER_ARGS=()

# Parse arguments
while [[ $# -gt 0 ]]; do
  case $1 in
    --out_path=*)
      OUT_PATH="${1#*=}"
      shift # past argument=value
      ;;
    --out_path)
      OUT_PATH="$2"
      shift # past argument
      shift # past value
      ;;
    *)
      OTHER_ARGS+=("$1")
      shift # past argument
      ;;
  esac
done

if [ -z "$OUT_PATH" ]; then
  echo "Error: --out_path is required"
  exit 1
fi

# Determine the directory for --fake_out
FAKE_OUT_DIR="."
if [[ "$OUT_PATH" == */ ]]; then
  # If OUT_PATH explicitly ends with a slash, it's a directory
  FAKE_OUT_DIR="$OUT_PATH"
elif [[ "$OUT_PATH" == */* ]]; then
  # If OUT_PATH contains a slash but doesn't end with one, extract the directory part
  FAKE_OUT_DIR="$(dirname "$OUT_PATH")"
fi

# Execute protoc directly
exec protoc \
  --plugin=protoc-gen-fake=./target/release/protoc-gen-fake \
  --proto_path=proto \
  --fake_out="$FAKE_OUT_DIR" \
  --fake_opt=output_path="$OUT_PATH" \
  "${OTHER_ARGS[@]}"