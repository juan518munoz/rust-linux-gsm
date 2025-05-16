# Game Server Manager

TODOs:
- Add Makefile commands to start/monitor/stop game servers.
  - `make` commands should run `docker` or `docker-compose` actions.
- Update server-launcher to use this new `make` commands.
- Implement some sort of security meassure, restrict API calls only to allowed users.
  - Using a fixed key may not be secure as of now, as running the server through http makes content not secure.
- Move this to workspace, create a new _deathwatch_ member:
  - New binary should be run as a cron, monitoring running servers that are unused.
  - Servers left empty for more than X will be stopped.

