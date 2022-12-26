#!/bin/bash

mkdir -p ./src/generated/proto

PROTOC=`command -v protoc`
if [[ "$PROTOC" == "" ]]; then
  echo "Required "protoc" to be installed. Please visit https://github.com/protocolbuffers/protobuf/releases."
  exit -1
fi

echo "Compiling protobuf definitions"
protoc \
  --plugin=./node_modules/.bin/protoc-gen-ts_proto \
  --ts_proto_out=./src/generated/proto \
  --ts_proto_opt=esModuleInterop=true \
  --ts_proto_opt=outputClientImpl=grpc-web \
  -I ../server/proto \
  ../server/proto/auth.v1.proto \
  ../server/proto/shop.v1.proto

# do not terminate
while true; do sleep 86400; done