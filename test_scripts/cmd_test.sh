#!/bin/bash

# Use the Makefile directory as the executing directory
# So './' is used instead of '../'
RED='\033[0;31m'
GREEN='\033[0;32m'

PNGMe=./target/debug/pngme
KEY="Roh9FEiuw+UiVPunRLM+Qrxcz+0bxeKQ/MS4ri1aUT8="
OG_FILE=images/dog.png
ENC_FILE=tmp/dog_secret.png
CHUNK_TYPE=abcd
MESSAGE="Secret!"
PASSPHRASE="MyPassphrase!"

fail_counter=0
success_counter=0

LINE_BREAKER="--------------------------------------------------------------"

assert_message() {
    local plaintext=$1
    if [ "$plaintext" == $MESSAGE ]; then
        echo PASSED
        success_counter=$((success_counter + 1))
    else
        echo FAILED
        fail_counter=$((fail_counter + 1))
    fi
}

capture_nonce() {
    # -n: Suppresses automatic printing of the pattern space (useful for controlling what gets output).
    # s/^Nonce:\(.*\)/\1/: This is the substitution command:
    # - ^Nonce: matches the beginning of the line followed by Nonce:.
    # - \(.*\) captures everything after Nonce: and stores it in a group.
    # - \1 refers to the first captured group (which is the <value>).
    # p: Prints the result only if the substitution was successful.
    sed -n 's/^Nonce:\(.*\)/\1/p'
}

capture_secret_mess() {
    sed -n 's/^Message:\(.*\)/\1/p'
}

# Remove the Carriage Return '\r' at the end of string
remove_end_cr() {
    tr -d '\r'
}

echo "TEST SCRIPT STARTS ==========================================="

echo "TEST encrypt/decrypt with the given key"
# Encode the encrypted/secret message
NONCE=$($PNGMe encode -i $OG_FILE -o $ENC_FILE -c $CHUNK_TYPE -m $MESSAGE -k $KEY | capture_nonce)
# Decode the decrypted message
PLAINTEXT=$($PNGMe decode -i $ENC_FILE -c $CHUNK_TYPE -k $KEY -n "$NONCE" | capture_secret_mess)
assert_message "$PLAINTEXT"
echo $LINE_BREAKER

echo "TEST encrypt/decrypt with the given passphrase"
NONCE=$($PNGMe encode -i $OG_FILE -o $ENC_FILE -c $CHUNK_TYPE -m $MESSAGE -p $PASSPHRASE | capture_nonce)
PLAINTEXT=$($PNGMe decode -i $ENC_FILE -c $CHUNK_TYPE -p $PASSPHRASE -n "$NONCE" | capture_secret_mess)
assert_message "$PLAINTEXT"
echo $LINE_BREAKER

echo "TEST encrypt/decrypt with the typed passphrase"
# As the output from TTY session usually include a Carriage Return '\r', we need to trim it to get the expected output
NONCE=$(./test_scripts/run_expect_tty.exp "$PNGMe encode -i $OG_FILE -o $ENC_FILE -c $CHUNK_TYPE -m $MESSAGE" "$PASSPHRASE" | capture_nonce | remove_end_cr)
PLAINTEXT=$(./test_scripts/run_expect_tty.exp "$PNGMe decode -i $ENC_FILE -c $CHUNK_TYPE -n $NONCE" "$PASSPHRASE" | capture_secret_mess | remove_end_cr)
assert_message "$PLAINTEXT"
echo $LINE_BREAKER

echo "TEST SCRIPT ENDS ==========================================="

echo "SUMMARY:"
echo -e "${GREEN}PASSED: $success_counter test case(s)"
echo -e "${RED}FAILED: $fail_counter test case(s)"

if [ "$fail_counter" -gt 0 ]; then
    # indicate error
    exit 1
else
    # indicate success
    exit 0
fi