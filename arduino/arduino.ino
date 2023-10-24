#include <Servo.h>

Servo servos[6];

void setup() {
  Serial.begin(9600);

  int servo_pins[] = {3, 5, 6, 9, 10, 11};

  for (int i = 0; i < 6; i++) {
    int pin = servo_pins[i];
    //servos[i] = new Servo();
    servos[i].attach(pin);
    servos[i].write(90);
  }
}

void loop() {
  if (Serial.available() > 0) {
    byte values[2];

    Serial.readBytes(values, 2);

    int i = index(values[0]);
    
    servos[i].write(values[1]);

    // Ok Message
    Serial.write(1);
  }
}

int index(int pin) {
  switch (pin) {
    case 3:
      return 0;
    case 5:
      return 1;
    case 6:
      return 2;
    case 9:
      return 3;
    case 10:
      return 4;
    case 11:
      return 5;
    default:
      return -1;
  }
}
