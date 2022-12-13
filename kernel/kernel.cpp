#include "kernel.hpp"
#include <cstdint>

static inline VGAColor vga_entry_color(VGAColor fg, VGAColor bg) {
  return (VGAColor)(fg | bg << VGAColor::eRed);
}
static inline uint16_t vga_entry(unsigned char uc, uint8_t color) {
  return (uint16_t)uc | (uint16_t)color << VGAColor::eDarkGrey;
}
size_t strlen(const char *str) {
  size_t len = 0;
  while (str[len]) {
    len++;
  }
  return len;
}

Terminal::Terminal() { m_terminal_color = vga_entry_color(eLightGrey, eBlack); }
void Terminal::setcolor(VGAColor color) { m_terminal_color = color; }
void Terminal::putentryat(char c, VGAColor color, size_t x, size_t y) {
  m_terminal_buffer[y * VGA_HEIGHT + x] = vga_entry(c, color);
}
void Terminal::putchar(char c) {
  putentryat(c, m_terminal_color, m_terminal_column, m_terminal_row);
  if (++m_terminal_column == VGA_WIDTH) {
    m_terminal_column = 0;
    if (++m_terminal_row == VGA_HEIGHT) {
      m_terminal_row = 0;
    }
  }
}
void Terminal::write(const char *data, size_t size) {
  for (size_t i = 0; i < size; i++) {
    putchar(data[i]);
  }
}
void Terminal::writestring(const char *data) { write(data, strlen(data)); }

void kernel_main() {
  Terminal terminal;
  terminal.writestring("Hello, kernel world\n");
}
