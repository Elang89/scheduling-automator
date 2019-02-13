#!/bin/bash

VALUE=$(psql -U postgres -qtAX -c "SELECT 1 FROM pg_database WHERE datname='${PSQL_DB}'" )

if ! [[ "${VALUE}" = '1' ]]
then
    psql -U postgres << EOF
        CREATE DATABASE ${PSQL_DB};
        CREATE USER ${PSQL_USER} WITH ENCRYPTED PASSWORD '${PSQL_PASSWORD}';
        GRANT ALL PRIVILEGES ON DATABASE ${PSQL_DB} TO ${PSQL_USER};
        \c ${PSQL_DB};
        CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
EOF
else
    echo "Database and user have already been created"
fi
