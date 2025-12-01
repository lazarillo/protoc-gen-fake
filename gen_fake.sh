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

# Execute protoc directly
exec protoc \
  --plugin=protoc-gen-fake=./target/debug/protoc-gen-fake \
  --fake_out="$OUT_PATH" \
  --fake_opt=output_path="$OUT_PATH" \
  "${OTHER_ARGS[@]}"