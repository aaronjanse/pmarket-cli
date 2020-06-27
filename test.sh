#!/usr/bin/env bash

set -e

cargo build
export PATH=$PATH:./target/debug

set -o xtrace



NOW=$(date +%s)
OPENS=$(($NOW+1))
CLOSES=$(($OPENS+1))

ADMIN_TOK=$(mktemp)
echo "niNh6Mnsktq3QX8x" > $ADMIN_TOK

pmarket-cli --token-file $ADMIN_TOK admin create-event \
  --title "USA 2020 Presedential Election" \
  --description "Who will be elected?" \
  --opens $OPENS --closes $CLOSES

pmarket-cli --token-file $ADMIN_TOK admin create-stock \
  --title "Trump" --event-id 1

pmarket-cli --token-file $ADMIN_TOK admin create-stock \
  --title "Biden" --event-id 1

JOHN_TOK=$(mktemp)
pmarket-cli signup --username john --password 123
pmarket-cli --token-file $JOHN_TOK signin --username john --password 123

ALICE_TOK=$(mktemp)
pmarket-cli signup --username alice --password 123
pmarket-cli --token-file $ALICE_TOK signin --username alice --password 123





