#!/usr/bin/env sh

AUTHORITY="193.23.244.244"
DATA_DIR=`dirname $0`

curl --compressed -o "$DATA_DIR/consensus" "http://$AUTHORITY/tor/status-vote/current/consensus.z"