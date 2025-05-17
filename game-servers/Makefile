ENV_FILE="../.env"

# make start SERVICE=mc-game-server
start:
	docker compose --env-file $(ENV_FILE) up $(SERVICE) -d

# make stop SERVER=mcserver
stop:
	docker stop $(SERVER)
