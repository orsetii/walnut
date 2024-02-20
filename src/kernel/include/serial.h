#ifndef SERIAL_H
#define SERIAL_H

#include <stddef.h>
#include <stdarg.h>
#include "types.h"
#include "io.h"

#define NS16550_MMIO_ADDR 0x10000000U

void initialize_uart();
int uart_has_data();
void set_divisor();
void uart_print(const char *str);
void uart_println(const char *str);
void uart_put(char c);
char uart_get();
char uart_get_blocking();

#endif