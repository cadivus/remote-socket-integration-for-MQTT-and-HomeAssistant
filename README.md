# Remote-socket-integration-for-MQTT-and-HomeAssistant
This project is for integrating cheap remote sockets into MQTT and especially HomeAssistant.
After getting notified about a change by MQTT, this program sends the change via serialport. The socket is identified by the remote control an the key, it’s assigned to.
This repository contains a library for sending the actual remote codes to the sockets and an implementation of the remote hardware for Arduino.

The hardware needed is pretty simple and looks like this:<br><img src="/doc/images/Arduino_Mega_433Mhz_transmitter.svg"><br>

An Arduino Nano is enough for this, the Mega is not needed.
