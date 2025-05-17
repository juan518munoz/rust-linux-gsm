# Minecraft

Forge changed the way they start Minecraft in 1.18. Here is how you can still enjoy the power of LGSM and run a modded Minecraft Forge server.

1. Run de docker compose for this server at least once.

2. Download the Forge Installer from: https://files.minecraftforge.net/ into `/serverfiles`

3. Access the container console then run:

```bash
docker exec -it --user linuxgsm mcserver /bin/bash
cd serverfiles
java -jar forge-1.18.x-x.x.x-installer.jar -installServer
```

4. Edit /home/mcserver/lgsm/config-lgsm/mcserver/mcserver.cfg, type in the following two lines:

```cfg
preexecutable=""
executable="./run.sh"
```

5. You may now start the server with docker compose.
