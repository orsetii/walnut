#ifndef PRINT_H
#define PRINT_H

#include <stdarg.h>
#include <stddef.h>
#include <stdint.h>
#include <limits.h>
#include "util.h"
#include "serial.h"

void kprint(const char *str);
void kprintln(const char *str);
void kvprintf(const char *, va_list);
void kprintf(const char *, ...);

#endif