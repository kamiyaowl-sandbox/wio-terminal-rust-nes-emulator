// #include <TFT_eSPI.h>
// #include <SPI.h>

// static TFT_eSPI tft = TFT_eSPI();

#include "rust_nes_emulator_embedded.h"

void setup() {
    // for debug
    Serial.begin(9600);
    while (!Serial) {}
    Serial.println("*rust-nes-emulator for Wio Terminal*");

    // Setup the LCD
    // tft.init();
    // tft.begin();
    // tft.setRotation(3);
    // tft.fillScreen(TFT_BLACK);
    // tft.setTextColor(TFT_WHITE);
    // tft.setTextSize(2);
    // tft.fillScreen(TFT_BLACK);
    // tft.setCursor(0, 0);

    // Setup the Emulator
    Serial.print("initialize...");
    EmbeddedEmulator_init();
    Serial.println("done");

    Serial.print("rom loading...");
    if (!EmbeddedEmulator_load()) {       
        Serial.println("Error");
        // tft.setTextColor(TFT_RED);
        // tft.print("Emulator load Error");        
    }
    Serial.println("done");
}

void loop() {
    uint8_t fb_line[EMBEDDED_EMULATOR_VISIBLE_SCREEN_WIDTH][EMBEDDED_EMULATOR_NUM_OF_COLOR];
    
    // emulation
    const uint8_t currentY = (uint8_t)EmbeddedEmulator_get_next_line_ptr(); // 次のLine位置
    EmbeddedEmulator_update_line(&fb_line);

    // screen update
    // tft.startWrite();
    if (currentY < EMBEDDED_EMULATOR_VISIBLE_SCREEN_HEIGHT) { // blank期間は書かなくて良い
        for(uint32_t i = 0 ; i < EMBEDDED_EMULATOR_VISIBLE_SCREEN_WIDTH ; ++i) {
            const uint8_t r = fb_line[i][0];
            const uint8_t g = fb_line[i][1];
            const uint8_t b = fb_line[i][2];
            if (r > 0x80 || g > 0x80 || b > 0x80) {
                Serial.print("#");
            } else {
                Serial.print(" ");
            }
            // const uint16_t color = (r << 11) | (g << 5) | b; // { r[15:11], g[10:5], b[4:0] }
            // tft.drawPixel(i, j, color);
        }
    }
    Serial.println("");
    // tft.endWrite();
}

