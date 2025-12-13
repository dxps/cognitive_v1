#!/bin/sh

BASEDIR=$(dirname $0)

sqlx migrate run --source $BASEDIR/db_migrations
