#!/bin/bash

set -e

# Build and copy build artifacts
./pax build --release --designer
#
cp -r .pax/build/release/web/assets ./public/
cp .pax/build/release/web/pax-* ./public/

aws --profile=pax s3 sync --exclude ".git/*" --acl=public-read ./public s3://staging.pax.dev/www
aws --profile=pax cloudfront create-invalidation --distribution-id=E29ZMWF6F0HQ61 --paths "/www/*"
