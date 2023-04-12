dev:
  # Setup the database with docker
  docker-compose up -d
  # Run migrations
  PGPASSWORD=postgres ./migrations/connect.example.sh < ./migrations/schemas_up.sql
  # Run the bot
  cargo shuttle run --port 9000

prod:
  cargo shuttle deploy
