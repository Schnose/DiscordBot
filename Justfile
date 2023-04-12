# Run locally
dev:
  # Setup the database with docker
  docker-compose up -d
  sleep 2
  # Run migrations
  PGPASSWORD=postgres ./migrations/connect.example.sh < ./migrations/schemas_up.sql
  # Run the bot
  cargo shuttle run --port 9000

# Clean up
yeet:
  docker-compose down
  # Sometimes the port the bot runs on is occupied even after it stopped, so we
  # make sure to end the process if it's still running.
  killall schnose-discord-bot || true

# Deploy to shuttle.rs
prod:
  cargo shuttle deploy
