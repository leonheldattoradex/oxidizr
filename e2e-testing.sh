#!/bin/bash

COMMAND="cargo run --"

options=(
    "--name-only"
    "--tls-creds"
    "--tls-root-ca"
    "--tls-cert"
    "--tls-prv-key"
    "--ecu-keys"
    "--ecu-keyid"
    "--ecu-pub-key"
    "--ecu-prv-key"
)

run_test() {
    local option=$1

    echo "Testing option: $option"

    output=$($COMMAND $option 2>&1)
    exit_status=$?

    if [ $exit_status -eq 0 ]; then
        echo "Success: $option"
    else
        echo "Failure: $option"
        echo "$output"
    fi

    echo "-----------------------------"
}

for option in "${options[@]}"; do
    run_test "$option"
done

echo "All tests completed!"