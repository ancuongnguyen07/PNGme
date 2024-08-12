#!/bin/bash

# Use the Makefile directory as the executing directory
# So './' is used instead of '../'
PNGMe=./target/debug/pngme
KEY="Roh9FEiuw+UiVPunRLM+Qrxcz+0bxeKQ/MS4ri1aUT8="
OG_FILE=images/dog.png
ENC_FILE=tmp/dog_secret.png

echo "TEST SCRIPT STARTS ==========================================="

# Encode the encrypted/secret message
$PNGMe encode -i $OG_FILE -o $ENC_FILE -c abcd -m "Secret!!!" -k $KEY >tmp/output.txt
# -n: Suppresses automatic printing of the pattern space (useful for controlling what gets output).
# s/^Nonce:\(.*\)/\1/: This is the substitution command:
# - ^Nonce: matches the beginning of the line followed by Nonce:.
# - \(.*\) captures everything after Nonce: and stores it in a group.
# - \1 refers to the first captured group (which is the <value>).
# p: Prints the result only if the substitution was successful.
NONCE=$(sed -n 's/^Nonce:\(.*\)/\1/p' tmp/output.txt)

# Decode the decrypted message
$PNGMe decode -i $ENC_FILE -c abcd -k $KEY -n $NONCE
echo PASSED

echo "TEST SCRIPT ENDS ==========================================="
