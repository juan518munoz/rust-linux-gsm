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

## Known issues:

### L4D2
Fails to initialize because it attemps to download file as `anonymous` user:

First login manually inside the container:
```bash
docker exec -it --user linuxgsm l4d2server /bin/steamcmd
```

Then change `/volumes/l4d2server/config-lgsm/l4d2server/common.cgf`:
```cfg
##################################
######## Common Settings #########
##################################
# PLACE GLOBAL SETTINGS HERE
## These settings will apply to all instances.
steamuser="<YOUR_USERNAME>"
steampass="" # your steam password - can be left blank if using steam guard
```

> Note: It's highly recommended to use steamguard and leave steampass field blank, as this fields are stored in plain text format.


### Minecraft

Forge changed the way they start Minecraft in 1.18. Here is how you can still enjoy the power of LGSM and run a modded Minecraft Forge server.

1. Install LinuxGSM scripts and go through the process of creating a Minecraft server

2. Start the server, then stop it

3. Download the Forge Installer from: https://files.minecraftforge.net/

4. Copy the forge-1.18.x-x.x.x-installer.jar to /home/mcserver/serverfiles

5. cd /home/mcserver/serverfiles

6. java -jar forge-1.18.x-x.x.x-installer.jar -installServer

7. Edit /home/mcserver/lgsm/config-lgsm/mcserver/mcserver.cfg
If the file is empty, copy everything from _default.cfg to mcserver.cfg

There are two lines you need to change
Comment out the two original lines with a # like this:

```
preexecutable="java -Xmx${javaram}M -jar"
executable="./minecraft_server.jar"
```

Type in the following replacement two lines:

```
preexecutable=""
executable="./run.sh"
```

The preexecutable section should now be empty, just two double quotes
