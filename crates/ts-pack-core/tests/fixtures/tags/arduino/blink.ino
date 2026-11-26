#include <Arduino.h>

#define LED_PIN 13
#define BLINK_TWICE(ms) blinkTimes(2, ms)

const int kButtonPin = 2;
int blinkCount = 0;
extern int bootCount;

typedef unsigned long millis_t;
using Callback = void (*)(int);

enum class Mode { Off, Steady, Blinking = 4 };

union Reading {
  int raw;
  float volts;
};

struct Pin {
  uint8_t number;
  bool inverted;

  Pin() : number(0), inverted(false) {}
};

namespace led {

class Output {
 public:
  virtual void write(bool on) = 0;
  virtual ~Output() = default;
};

class DigitalOutput : public Output {
 public:
  explicit DigitalOutput(uint8_t pin);
  void write(bool on) override { digitalWrite(pin_, on ? HIGH : LOW); }
  const Pin& pin() const { return config_; }
  void toggle();

 private:
  uint8_t pin_;
  Pin config_;
};

DigitalOutput::DigitalOutput(uint8_t pin) : pin_(pin) {}

void DigitalOutput::toggle() {
  write(digitalRead(pin_) == LOW);
}

}  // namespace led

namespace sensors::analog {
float toVolts(int raw) { return raw * (5.0 / 1023.0); }
}  // namespace sensors::analog

template <typename T>
T clampValue(T value, T low, T high) {
  return value < low ? low : (value > high ? high : value);
}

led::DigitalOutput* makeLed(uint8_t pin) {
  return new led::DigitalOutput(pin);
}

void blinkTimes(int times, int ms) {
  for (int i = 0; i < times; i++) {
    digitalWrite(LED_PIN, HIGH);
    delay(ms);
    digitalWrite(LED_PIN, LOW);
    delay(ms);
  }
}

void blinkTimes(int times) {
  blinkTimes(times, 250);
}

led::DigitalOutput *statusLed;

void setup() {
  pinMode(LED_PIN, OUTPUT);
  pinMode(kButtonPin, INPUT_PULLUP);
  statusLed = makeLed(LED_PIN);
}

void loop() {
  int raw = analogRead(A0);
  float volts = sensors::analog::toVolts(raw);
  int ms = clampValue<int>(static_cast<int>(volts * 100), 50, 1000);
  statusLed->toggle();
  BLINK_TWICE(ms);
  blinkCount++;
}
