#include <Servo.h>

Servo servos[9];

void setup() {
  Serial.begin(9600);

  for (int i = 0; i < 9; i++) {
    servos[i].attach(i + 3);
    servos[i].write(90);
  }
}

void loop() {
  if (Serial.available() > 0) {
    byte values[2];

    Serial.readBytes(values, 2);

    int i = values[0] - 3;
    
    servos[i].write(values[1]);

    // Ok Message
    Serial.write(1);
  }
}
