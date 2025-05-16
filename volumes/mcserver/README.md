# Minecraft

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
