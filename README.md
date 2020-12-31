# Remote-socket-integration-for-MQTT-and-HomeAssistant
This project is for integrating cheap remote sockets into MQTT and especially HomeAssistant.
After getting notified about a change by MQTT, this program sends the change via serialport. The socket is identified by the remote control an the key, it’s assigned to.
This repository contains a library for sending the actual remote codes to the sockets and an implementation of the remote hardware for Arduino.

The hardware needed is pretty simple and looks like this:<br><img src="/doc/images/Arduino_Mega_433Mhz_transmitter.svg"><br>

An Arduino Nano is enough for this, the Mega is not needed.


## Configuration

By placing a file named "config" next to the executable, you can configure the program.
The configuration is written in JSON and looks like this:
```JSON
{
    "serialport": "/dev/serial/by-id/der_arduino",
    "baudrate": 9600,
    "broker": "localhost",
    "brokerport": 1883,
    "brokeruser": "username",
    "brokerpassword": "password",
    "devices": [{"type":"switch","name":"Remoteswitch1","manufacturer":"Silvercrest","model":"60494","remote":0,"device":0},
    			{"type":"switch","name":"Remoteswitch2","manufacturer":"Silvercrest","model":"60494","remote":0,"device":1}]
}
```
"remote" is the remote control being emulated and "device" specifies the keys of the remote.
