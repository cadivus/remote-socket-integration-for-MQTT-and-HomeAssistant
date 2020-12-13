#include <RCSwitch.h>
#include <ArduinoJson.h>
#include "SilvercrestCodes.h"

RCSwitch rcSwitch = RCSwitch();
const int senderPin = 9;

const int protocol = 5;
const int pulseLength = 523;
const int wait = 450;

void setup() {
  rcSwitch.enableTransmit(senderPin);
  rcSwitch.setPulseLength(pulseLength);
  rcSwitch.setProtocol(protocol);
  Serial.begin(9600);
}


void loop() {
  char buff[255] = {0};
  int pos = 0;
  while (true) {
    if(Serial.available() > 0) {
      buff[pos] = Serial.read();
  
      if(buff[pos] == '\n'){
        break;
      }
      ++pos;
    }    
  }
  buff[pos] = '\0';
  if(pos == 0) {
    return;
  }

  keypress key = getKeyPress(buff);
  unsigned long code = SilvercrestCodes::get_code(key);
  rcSwitch.send(code, 24);
  delayMicroseconds(500);
  rcSwitch.send(code, 24);
}

keypress getKeyPress(char* input) {
  keypress result = {0};

  DynamicJsonDocument doc(1024);
  deserializeJson(doc, input);
  
  const char* state = doc["state"];
  const char* remote = doc["remote"];
  const char* device = doc["device"];

  result.remote = atoi(remote);
  result.device = atoi(device);
  result.valid = true;
  
  if(strcmp(state, "ON") == 0) {
    result.state = SilvercrestCodes::ON;
  } else if(strcmp(state, "OFF") == 0) {
    result.state = SilvercrestCodes::OFF;
  } else {
    result.valid = false;
    return result;
  }

  return result;
}
