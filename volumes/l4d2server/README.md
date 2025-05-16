# L4D2
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

