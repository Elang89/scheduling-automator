#!/bin/bash

source env.sh
rm -rf docker-compose.production.yml
rm -rf docker-compose.development.yml
rm -rf docker-compose.testing.yml

envsubst < "templates/template.production.yml" > "../docker-compose.production.yml"
envsubst < "templates/template.development.yml" > "../docker-compose.development.yml"
envsubst < "templates/template.testing.yml" > "../docker-compose.testing.yml"
