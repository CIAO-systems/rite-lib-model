#!/bin/env bash
REPORT_FILE=target/coverage/tarpaulin-report.html
cargo tarpaulin \
    --out Html \
    --output-dir target/coverage \
    --exclude-files "src/plugin.rs"


# Check for option --open
if [[ "$1" == "--open" ]]; then
    OPEN_COMMAND="xdg-open"

    if [[ "$OSTYPE" == "darwin"* ]]; then
        OPEN_COMMAND="open"
    fi

    eval $OPEN_COMMAND $REPORT_FILE
fi