#include "rust_nes_emulator_embedded.h"

void setup() {
    Serial.begin(9600);
    Serial.println("on setup()");

    EmbeddedEmulator_init();
    Serial.println("Emulator initialzied.");
}

void loop() {
    Serial.println("on loop()");
    delay(100);
}