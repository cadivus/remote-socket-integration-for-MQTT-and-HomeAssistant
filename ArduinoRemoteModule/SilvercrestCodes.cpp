/* 
 * File:   SilvercrestCodes.cpp
 * Author: cadivus
 * 
 * Created on November 10, 2020, 4:11 PM
 */

#include "SilvercrestCodes.h"

namespace SilvercrestCodes {
  unsigned long get_code(int remote, int key, int state) {
    int old_rollin = rolling[remote];
    rolling[remote] = (rolling[remote] + 1) % 4;
    
    return codes[remote][old_rollin][key][state];
  }
  
  unsigned long get_code(keypress key) {
    int remote = key.remote;
    int device = key.device;
    int state = key.state;
      
    return get_code(remote, device, state);
  }
  
  keypress find_code(unsigned long code) {
    keypress result = {0};
    result.valid = false;
    
    for(int remote = 0; remote < remote_count; ++remote) {
      for(int rolling = 0; rolling < rolling_count; ++rolling) {
        for(int device = 0; device < device_count; ++device) {
          for(int state = 0; state < state_count; ++state) {
            if(code == codes[remote][rolling][device][state]) {
              result.remote = remote;
              result.device = device;
              result.state = state;
              result.valid = true;
            }
          }
        }
      }
    }
    return result;
  }
  
  int get_code_length(int remote) {
    return length_of_code[remote];
  }
}
