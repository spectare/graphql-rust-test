#!/bin/bash

REPORT=$(find target/debug -maxdepth 1 -name 'graphql_rust_test-*' -a ! -name '*.d')

for file in $REPORT; do
    mkdir -p "target/cov/$(basename $file)"
    sudo kcov --include-pattern=rori_core --exclude-pattern=/.cargo,/usr/lib --verify "target/cov/$(basename $file)" "$file"
done

wget -O - -q "https://codecov.io/bash" > .codecov
chmod +x .codecov
./.codecov -t $CODECOV_TOKEN
echo "Uploaded code coverage"
