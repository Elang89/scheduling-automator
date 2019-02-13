#!/bin/bash

DB="$1"

psql -h localhost -U postgres -d ${DB} -f seed.sql