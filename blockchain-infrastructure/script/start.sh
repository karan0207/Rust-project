#!/bin/bash

# Start Tendermint
tendermint init
tendermint node --proxy_app=tcp://127.0.0.1:26658
