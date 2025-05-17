# Game Servers

## Adding a new Server

TODO

## Importing an existing server

1. Copy the server `volume` from the previous server to the new server.
2. Add the associated `docker-compose` entry to the new server.

## Executing commands from within a server

With the server running:

```bash
docker exec -it --user linuxgsm csgoserver ./csgoserver details
```

> You can access `bash` this way too.
