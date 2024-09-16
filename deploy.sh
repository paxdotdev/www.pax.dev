#!/bin/bash

set -e

# Build and copy build artifacts
./pax build --release --designer
#
cp -r .pax/build/release/web/assets ./public/assets
cp .pax/build/release/web/pax-* ./public/

aws --profile=pax s3 sync --exclude ".git/*" --acl=public-read ./public s3://www.pax.dev/ 
aws --profile=pax cloudfront create-invalidation --distribution-id=EYKZZ3KH242XU --paths "/*"
