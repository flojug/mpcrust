Work in progress
==============

But
==============
Faire une chaîne Audiophile à base de Raspberry Pi.
- écouter morceaux enregistrés sur un disque dur local
- écouter radios

Avec un écran Touchscreen sans utiliser l'écran tactile (ne fonctionne plus).

Taille de l'écran : 50x15 pour avoir de gros caractères visibles de loin.

Contrôle avec une télécommande IR simplifiée (juste quelques boutons)


-> Tentatives pour utiliser ncmpc ou ncmpcpp (et plusieurs autres clients mpc) mais impossible de les contrôler par IR (trop complexe de gérer les différentes popups avec seulement quelques boutons de la télécommande).


Matériel
==============
Raspberry Pi 3B+
-----------------

Ecran Tactile Officiel 7" 800x480 Touchscreen
-----------------
Terminal 100x30

![Ecran+Raspberry](./images/IMG_1358.JPG)

RemotePi Board for Pi 3B and Pi 3B+
-----------------
https://www.msldigital.com/collections/all-products/products/remotepi-board-for-pi-3

Pour pouvoir allumer, éteindre, contrôler le Raspberry par IR

![Remoteboard](./images/IMG_1362.JPG)

DAC U-SABRE
-----------------
https://www.audiophonics.fr/fr/dac-sans-volume/audiophonics-u-sabre-usb-dac-24bit96khz-sa9023-es9023-otg-v22e-tcxo-edition-p-11056.html

![DAC](./images/IMG_1361.JPG)

Ampli FX-AUDIO
-----------------
https://www.audiophonics.fr/fr/amplificateur-full-digital-fda/fx-audio-d802c-pro-amplificateur-fda-bluetooth-42-nfc-class-d-sta326-2x80w-4-ohm-noir-p-12850.html

![Ampli](./images/IMG_1360.JPG)

Télécommande standard
-----------------
![Télécommande](./images/IMG_1363.JPG)

Un disque SSD 512 Go pour stocker la musique
-----------------

La chaine une fois assemblée :
-----------------
![Chaine1](./images/IMG_1368.JPG)
![Chaine2](./images/IMG_1383.JPG)
![Chaine3](./images/IMG_1386.JPG)


Installation
==============

Installer Raspbian
-----------------
Raspberry Pi OS (32-bit) Lite

Installer mpd

[Liste des packages installés](./install/packages.txt)


Configuration de l'écran, terminal 50x15
-----------------

    # cat /etc/default/console-setup
    ...
    FONTFACE="Terminus"
    FONTSIZE="16x32"
    ...

    # etc/init.d/console-setup.sh restart

Faire tourner l'écran si besoin
-----------------

    # cat /boot/config.txt
    ...
    lcd_rotate=2
    ...

Touches
-----------------
    .local/share/mpcrust/keys.json


Configuration IR
-----------------

    # cat /boot/config.txt
    ...
    dtoverlay=gpio-ir
    ...


    apt install ir-keytable
    apt install inputlirc lirc

Connaître le /dev/sys utilisé

    # ir-keytable

Initialiser un protocole

    # cat /sys/class/rc/rc0/protocols
    # echo nec > /sys/class/rc/rc0/protocols

    ir-keytable -p <protocole>
    ir-keytable -t

Générer le fichier /etc/rc_keymaps/one_for_all.toml

Ajouter dans /etc/rc.local

    ir-keytable -c -w /etc/rc_keymaps/one_for_all.toml --sysdev rc0


    cat /etc/default/inputlirc

    # Options to be passed to inputlirc.
    EVENTS="/dev/input/event0"
    OPTIONS="-g -m 0 -c"
    # EVENTS="/dev/input/event*"
    # OPTIONS=

Il faut désactiver lircd (?? à voir)

Associer les touches à des événements claviers
    # cat lircrc
    begin
         prog = irexec
         button = KEY_POWER
         config = /home/pi/atou.sh "a"
    end
    begin
         prog = irexec
         button = KEY_POWER2
         config = /home/pi/atou.sh "b"
    end
    begin
         prog = irexec
         button = KEY_TV_AV
         config = /home/pi/atou.sh "c"
    end
    ...



Désactiver WIFI et Bluetooth
-----------------
Pour ceux qui n'aiment pas baigner dans les ondes électromagnatiques

    # cat /boot/config.txt
    ...
    dtoverlay=disable-bt
    dtoverlay=disable-wifi
    ...

    # sudo systemctl disable hciuart.service
    # sudo systemctl disable bluealsa.service
    # sudo systemctl disable bluetooth.service

    # sudo apt-get purge bluez -y
    # sudo apt-get autoremove -y


Monter la partition de musique
-----------------

    # mount /dev/sda1 /var/lib/mpd/music

Pare-feu basique
-----------------

    # cat /etc/init.d/packetfilter
    #!/bin/bash

    ### BEGIN INIT INFO
    # Provides:          packetfilter
    # Required-Start:    $remote_fs $syslog
    # Required-Stop:     $remote_fs $syslog
    # Default-Start:     2 3 4 5
    # Default-Stop:      0 1 6
    # Short-Description: DÃ©marre les rÃ¨gles iptables
    # Description:       Charge la configuration du pare-feu iptables
    ### END INIT INFO

    # Script de controle du filtrage ip.

    PATH=/bin:/sbin:/usr/sbin:/usr/bin

    case "$1" in
        start)
        echo -n "Turning on packet filtering:"

        # Par defaut
        iptables -F
        iptables -X
        iptables -t nat -F
        iptables -t nat -X
        iptables -P INPUT ACCEPT
        iptables -P OUTPUT ACCEPT
        iptables -P FORWARD DROP

        # Connections etablies
        iptables -A INPUT -m state --state ESTABLISHED,RELATED -j ACCEPT

        # Ssh depuis l'exterieur
        iptables -A INPUT -p tcp -i eth0 --dport 22 -j ACCEPT

        # ICMP
        iptables -A INPUT -p icmp -i eth0 -j ACCEPT

        # On bloque le reste pour eth0
        iptables -A INPUT -i eth0 -j DROP

        echo "."
        ;;
        stop)
        echo -n "Turning off packet filtering:"
        iptables -F
        iptables -X
        iptables -t nat -F
        iptables -t nat -X
        iptables -P INPUT ACCEPT
        iptables -P OUTPUT ACCEPT
        iptables -P FORWARD DROP
        echo "."
        ;;
        restart)
        $0 stop
        sleep 2
        $0 start
        ;;
        *)
        echo "Usage: /etc/init.d/packetfilter {start|stop|restart}"
        exit 1
        ;;
    esac

    # /etc/init.d/packetfilter start


splash
-----------------

### fbset

    mode "800x480"
      geometry 800 480 800 480 32
      timings 0 0 0 0 0 0 0
      rgba 8/16,8/8,8/0,8/24
    endmode

https://yingtongli.me/blog/2016/12/21/splash.html

mais ne pas faire : Disable the login prompt by running systemctl disable getty@tty1 as root.


https://raspberrypi.stackexchange.com/questions/59310/remove-boot-messages-all-text-in-jessie
https://retropie.org.uk/docs/FAQ/#how-do-i-hide-the-boot-text
https://retropie.org.uk/forum/topic/14299/tutorial-remove-boot-text-on-the-raspberry-pi-for-noobs

NON :
http://redsymbol.net/linux-kernel-boot-parameters//2.6.25/
Enlever couleurs écran
dwc_otg.lpm_enable=0 console=serial0,115200 console=tty1 root=PARTUUID=27515498-02 rootfstype=ext4 elevator=deadline fsck.repair=yes rootwait logo.nologo consoleblank=0 loglevel=1 quiet  vt.default_red=0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0 vt.default_grn=0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0 vt.default_blu=0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0


    echo -n 0,0,0,0,170,170,170,170,85,85,85,85,255,255,255,255 > /sys/module/vt/parameters/default_blu
    echo -n 0,0,170,85,0,0,170,170,85,85,255,255,85,85,255,255 > /sys/module/vt/parameters/default_grn
    echo -n 0,170,0,170,0,170,0,170,85,255,85,255,85,255,85,255 > /sys/module/vt/parameters/default_red

    [Unit]
    Description=Restore system coloes
    Before=getty@tty1.service

    [Service]
    Type=oneshot
    ExecStart=/home/pi/restore-colors

    [Install]
    WantedBy=getty.target


    cat > /home/pi/restore-colors
    echo -n 0,0,0,0,170,170,170,170,85,85,85,85,255,255,255,255 > /sys/module/vt/parameters/default_blu
    echo -n 0,0,170,85,0,0,170,170,85,85,255,255,85,85,255,255 > /sys/module/vt/parameters/default_grn
    echo -n 0,170,0,170,0,170,0,170,85,255,85,255,85,255,85,255 > /sys/module/vt/parameters/default_red
    FIN NON


Radios
==============
http://dir.xiph.org/yp.xml

    telnet ice2.ikoula.net-radio.fr 80
    GET frequence3.flac HTTP1/1
    Hostname: ice2.ikoula.net-radio.fr


Utilisation
==============

Configuration des touches
-----------------



