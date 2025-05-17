# Game Servers

## Adding a new Server

TODO

> NOTE: THIS DOESN'T CONTEMPLATE NEEDED CHANGES TO BACKEND/FRONTEND

## Importing an existing server

1. Copy the server `volume` from the previous server to the new server.
2. Add the associated `docker-compose` entry to the new server.
3. Modify the `.env` file to add the needed environment variables.

> NOTE: THIS DOESN'T CONTEMPLATE NEEDED CHANGES TO BACKEND/FRONTEND

## Executing commands from within a server

With the server running:

```bash
docker exec -it --user linuxgsm csgoserver ./csgoserver details
```

> You can access `bash` this way too.
