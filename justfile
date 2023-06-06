# Shows all available recipes
default:
  @just --list

# Setup the database container
db-up:
  # PostgreSQL starting up...
  docker-compose up -d

# Delete the database container
db-down:
  # Destroying PostgreSQL container...
  docker-compose down

# Connect to the local database
connect:
  PGPASSWORD=postgres ./migrations/connect.sh

# Run database migrations to create tables (copy `./migrations/connect.example.sh` to `./migrations/connect.sh`)
migrate-up:
  PGPASSWORD=postgres ./migrations/connect.sh < ./migrations/up.sql
  PGPASSWORD=postgres ./migrations/connect.sh < ./migrations/data.sql

# Run database migrations to delete tables (copy `./migrations/connect.example.sh` to `./migrations/connect.sh`)
migrate-down:
  PGPASSWORD=postgres ./migrations/connect.sh < ./migrations/down.sql

# Create a project for the bot on shuttle.rs
setup:
  cargo shuttle project start --name schnose-discord-bot --idle-minutes 0

# Run the bot using `cargo-shuttle`
run:
  cargo shuttle run --name schnose-discord-bot

# Deploy the bot to shuttle.rs
deploy:
  cargo shuttle deploy --name schnose-discord-bot

# vim: et ts=2 sw=2 sts=2 ai si
