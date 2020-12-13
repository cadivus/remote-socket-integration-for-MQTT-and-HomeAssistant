/* 
 * File:   SilvercrestCodes.h
 * Author: cadivus
 *
 * Created on November 10, 2020, 4:10 PM
 */

#ifndef SILVERCRESTCODES_H
#define SILVERCRESTCODES_H

typedef struct {
  bool valid;
  int remote;
  int device;
  int state;
}keypress;

namespace SilvercrestCodes {
  static int ON = 0;
  static int OFF = 1;
  
  static int rolling[3] = {0};
  static int length_of_code[3] = {24, 24, 24};
  
  static const int remote_count = 3;
  static const int rolling_count = 4;
  static const int device_count = 5;
  static const int state_count = 2;
  static unsigned long codes[remote_count][rolling_count][device_count][state_count] = {
      {
        //https://github.com/peterand/BATsender/blob/master/BAT_Lidl_codes.txt
        {
          {/*A1_ON: */0xa460ac, /*A1_OFF: */0xae057c},
          {/*B1_ON: */0xada405, /*B1_OFF: */0xa15895},
          {/*C1_ON: */0xa9e98e, /*C1_OFF: */0xa460ae},
          {/*D1_ON: */0xa0cbd7, /*D1_OFF: */0xa23137},
          {/*M1_ON: */0xa9e982, /*M1_OFF: */0xa460a2}
        },
        {
          {/*A2_ON: */0xa77e6c, /*A2_OFF: */0xaf4f2c},
          {/*B2_ON: */0xa23135, /*B2_OFF: */0xa0cbd5},
          {/*C2_ON: */0xac2abe, /*C2_OFF: */0xa77e6e},
          {/*D2_ON: */0xa51c47, /*D2_OFF: */0xa386f7},
          {/*M2_ON: */0xaf4f22, /*M2_OFF: */0xabb252}
        },
        {
          {/*A3_ON: */0xa6971c, /*A3_OFF: */0xa9e98c},
          {/*B3_ON: */0xa386f5, /*B3_OFF: */0xa51c45},
          {/*C3_ON: */0xae057e, /*C3_OFF: */0xa6971e},
          {/*D3_ON: */0xa8fdc7, /*D3_OFF: */0xaad3e7},
          {/*M3_ON: */0xae0572, /*M3_OFF: */0xa69712}
        },
        {
          {/*A4_ON: */0xabb25c, /*A4_OFF: */0xac2abc},
          {/*B4_ON: */0xaad3e5, /*B4_OFF: */0xa8fdc5},
          {/*C4_ON: */0xaf4f2e, /*C4_OFF: */0xabb25e},
          {/*D4_ON: */0xa15897, /*D4_OFF: */0xada407},
          {/*M4_ON: */0xac2ab2, /*M4_OFF: */0xa77e62}
        }
      },
      {
        //https://github.com/pimatic/pimatic/issues/386
        {
          {/*A1_ON: */0x7F0E5C, /*A1_OFF: */0x78B9BC},
          {/*B1_ON: */0x7E4435, /*B1_OFF: */0x73C095},
          {/*C1_ON: */0x71172E, /*C1_OFF: */0x79AF1E},
          {/*D1_ON: */0x727547, /*D1_OFF: */0x708DF7},
          {/*M1_ON: */0x711722, /*M1_OFF: */0x79AF12}
        },
        {
          {/*A2_ON: */0x76D8AC, /*A2_OFF: */0x7A9C7C},
          {/*B2_ON: */0x708DF5, /*B2_OFF: */0x7553D5},
          {/*C2_ON: */0x773A8E, /*C2_OFF: */0x7F0E5E},
          {/*D2_ON: */0x7BF6C7, /*D2_OFF: */0x7C6BE7},
          {/*M2_ON: */0x7A9C72, /*M2_OFF: */0x7DE262}
        },
        {
          {/*A3_ON: */0x7DE26C, /*A3_OFF: */0x71172C},
          {/*B3_ON: */0x7C6BE5, /*B3_OFF: */0x727545},
          {/*C3_ON: */0x78B9BE, /*C3_OFF: */0x76D8AE},
          {/*D3_ON: */0x73C097, /*D3_OFF: */0x742107},
          {/*M3_ON: */0x78B9B2, /*M3_OFF: */0x76D8A2}
        },
        {
          {/*A4_ON: */0x79AF1C, /*A4_OFF: */0x773A8C},
          {/*B4_ON: */0x742105, /*B4_OFF: */0x7BF6C5},
          {/*C4_ON: */0x7A9C7E, /*C4_OFF: */0x7DE26E},
          {/*D4_ON: */0x7553D7, /*D4_OFF: */0x7E4437},
          {/*M4_ON: */0x773A82, /*M4_OFF: */0x7F0E52}
        }
      },
      {
        //https://forum.pimatic.org/topic/3337/433-mhz-funksteckdosen-lidl-silvercrest-rcr-dp3-3711-a-brennenstuhl-mit-homeduino
        {
          {/*A1_ON: */0x0A67B0, /*A1_OFF: */0x0BC910},
          {/*B1_ON: */0x0FF334, /*B1_OFF: */0x051294},
          {/*C1_ON: */0x070B7C, /*C1_OFF: */0x04482C},
          {/*D1_ON: */0x0BC912, /*D1_OFF: */0x0A67B2},
          {/*M1_ON: */0x0385AA, /*M1_OFF: */0x01EF4A}
        },
        {
          {/*A2_ON: */0x070B70, /*A2_OFF: */0x044820},
          {/*B2_ON: */0x01EF44, /*B2_OFF: */0x0385A4},
          {/*C2_ON: */0x0A67BC, /*C2_OFF: */0x0BC91C},
          {/*D2_ON: */0x044822, /*D2_OFF: */0x070B72},
          {/*M2_ON: */0x05129A, /*M2_OFF: */0x0FF33A}
        },
        {
          {/*A3_ON: */0x01EF40, /*A3_OFF: */0x0385A0},
          {/*B3_ON: */0x070B74, /*B3_OFF: */0x044824},
          {/*C3_ON: */0x0FF33C, /*C3_OFF: */0x05129C},
          {/*D3_ON: */0x0385A2, /*D3_OFF: */0x01EF42},
          {/*M3_ON: */0x0BC91A, /*M3_OFF: */0x0A67BA}
        },
        {
          {/*A4_ON: */0x0FF330, /*A4_OFF: */0x051290},
          {/*B4_ON: */0x0A67B4, /*B4_OFF: */0x0BC914},
          {/*C4_ON: */0x01EF4C, /*C4_OFF: */0x0385AC},
          {/*D4_ON: */0x051292, /*D4_OFF: */0x0FF332},
          {/*M4_ON: */0x04482A, /*M4_OFF: */0x070B7A}
        }
      }
    };
  
  unsigned long get_code(int remote, int key, int state);
  unsigned long get_code(keypress key);
  keypress find_code(unsigned long code);
  int get_code_length(int remote);
};

#endif /* SILVERCRESTCODES_H */
