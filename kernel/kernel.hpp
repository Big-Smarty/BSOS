#pragma once

#include <cstdint>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

static const size_t VGA_WIDTH = 80;
static const size_t VGA_HEIGHT = 25;

enum VGAColor : uint8_t {
  eBlack = 0,
  eBlue = 1,
  eGreen = 2,
  eCyan = 3,
  eRed = 4,
  eMagenta = 5,
  eBrown = 6,
  eLightGrey = 7,
  eDarkGrey = 8,
  eLightBlue = 9,
  eLightGreen = 10,
  eLightCyan = 11,
  eLightRed = 12,
  eLightMagenta = 13,
  eLightBrown = 14,
  eWhite = 15,
};

static inline VGAColor vga_entry_color(VGAColor fg, VGAColor bg);
static inline uint16_t vga_entry(unsigned char uc, uint8_t color);
size_t strlen(const char *str);

class Terminal {
public:
  Terminal();

  void setcolor(VGAColor color);
  void putentryat(char c, VGAColor color, size_t x, size_t y);
  void putchar(char c);
  void write(const char *data, size_t size);
  void writestring(const char *data);

private:
  size_t m_terminal_row{0};
  size_t m_terminal_column{0};
  VGAColor m_terminal_color{VGAColor::eBlack};
  uint16_t *m_terminal_buffer;
};

extern "C" void kernel_main();
