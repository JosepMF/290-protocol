#!/bin/bash

# init the test server

cd ..

sudo cargo run --bin server-attacker -- --port 3000 --command ls
