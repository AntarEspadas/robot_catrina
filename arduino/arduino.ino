#include <Servo.h>

#define SERVO_COUNT 9
#define LED_PIN 12

Servo servos[SERVO_COUNT];

void setup() {
  Serial.begin(9600);

  for (int i = 0; i < SERVO_COUNT; i++) {
    servos[i].attach(i + 3);
    if (i % 2 != 0)
      servos[i].write(0);
    else
      servos[i].write(180);
  }

  pinMode(LED_PIN, OUTPUT);
}

void loop() {
  if (Serial.available() > 0) {
    byte values[2];

    Serial.readBytes(values, 2);

    int i = values[0] - 3;

    if (i > 0 && i < SERVO_COUNT) {
      servos[i].write(values[1]);
      // Ok Message
      Serial.write(1);
      // Error Message

    }
    else if (i >= SERVO_COUNT) {
      digitalWrite(i, values[1]);
      // Ok Message
      Serial.write(1);
    }
    else {
      // Error Message
      Serial.write(0);
      return;   
    }

  }
}
