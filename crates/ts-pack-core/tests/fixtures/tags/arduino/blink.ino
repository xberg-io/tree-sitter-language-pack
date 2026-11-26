int ledPin = 13;

void setup() {
  pinMode(ledPin, OUTPUT);
}

void loop() {
  blink(500);
}

void blink(int ms) {
  digitalWrite(ledPin, HIGH);
  delay(ms);
}
