#!/usr/bin/env bash

test -d src/global && rm -r src/global
test -d src/region && rm -r src/region

# global bindings
curl https://bitcraft-early-access.spacetimedb.com/v1/database/bitcraft-global/schema?version=9 | jq '{"V9": .}' > schema.json
spacetime generate --module-def schema.json --lang rs --out-dir src/global
rm schema.json

# region bindings
curl https://bitcraft-early-access.spacetimedb.com/v1/database/bitcraft-2/schema?version=9 | jq '{"V9": .}' > schema.json
spacetime generate --module-def schema.json --lang rs --out-dir src/region
rm schema.json

# strip message for version
ST_VER='1\.3\.0'
ST_REV=''
find src -name "*.rs" -type f -exec \
  perl -i -0pe "s|\n\n// This was generated using spacetimedb cli version $ST_VER \(commit $ST_REV\)\.||g" {} +

sed -i "s/^version = .*$/version = \"$(date -u +%Y.%-m.%-d)\"/" Cargo.toml
