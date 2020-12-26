# ArduinoRemoteModule

The Arduino or any other serial device gets JSON-formatted data like this:
```JSON
{name: "<name of remote socket>", state: "<ON or OFF>", remote: "<remote number>", device: "<device number>"}
```
The name of the remote socket is not needed, but the other information.
After getting the information, this Arduino-sketch looks up the remote codes using the SilvercrestCodes library and sends it using RCSwitch (https://github.com/sui77/rc-switch). All counter start at zero.


SilvercrestCodes

This library implements the rolling code of Silvercrest 60494 remote switches.
<br>
<br><img src="/../doc/images/Silvercrest_60494.png"><br>
<br>
By passing the remote, the switch and the state, this library returns the next code to send. All counters start at zero.
