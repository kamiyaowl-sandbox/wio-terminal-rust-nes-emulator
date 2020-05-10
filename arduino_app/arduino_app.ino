#include <TFT_eSPI.h> // Hardware-specific library
#include <SPI.h>

static TFT_eSPI tft = TFT_eSPI();       // Invoke custom library

#include "rust_nes_emulator_embedded.h"
static uint8_t fb[EMBEDDED_EMULATOR_VISIBLE_SCREEN_HEIGHT][EMBEDDED_EMULATOR_VISIBLE_SCREEN_WIDTH][EMBEDDED_EMULATOR_NUM_OF_COLOR];

void setup() {
    // for debug
    Serial.begin(250000);

    // Setup the LCD
    tft.init();
    tft.begin();
    tft.setRotation(3);
    tft.fillScreen(TFT_BLACK);
    tft.setTextColor(TFT_WHITE);
    tft.setTextSize(2);
    tft.fillScreen(TFT_BLACK);
    tft.setCursor(0, 0);

    // Setup the Emulator
    EmbeddedEmulator_init();
    if (!EmbeddedEmulator_load()) {
        tft.setTextColor(TFT_RED);
        tft.print("Emulator load Error");        
        while(1) {}
    }
}

void loop() {
    // emulation
    EmbeddedEmulator_update_screen(&fb);

    // screen update
    tft.startWrite();
    for(uint8_t j = 0 ; j < EMBEDDED_EMULATOR_VISIBLE_SCREEN_HEIGHT ; ++j) {
        for(uint8_t i = 0 ; i < EMBEDDED_EMULATOR_VISIBLE_SCREEN_WIDTH ; ++i) {
            const uint8_t r = fb[j][i][0];
            const uint8_t g = fb[j][i][1];
            const uint8_t b = fb[j][i][2];
            const uint16_t color = (r << 11) | (g << 5) | b; // { r[15:11], g[10:5], b[4:0] }
            tft.drawPixel(i, j, color);
        }
    }
    tft.endWrite();
}

