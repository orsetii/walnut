#include "print.h"

#define to_hex_digit(n) ('0' + (n) + ((n) < 10 ? 0 : 'a' - '0' - 10))

void kprint(const char *str)
{

#ifdef UART_ENABLED
    uart_print(str);
#endif
}
void kprintln(const char *str)
{

#ifdef UART_ENABLED
    uart_println(str);
#endif
}

// Limited version of vprintf() which only supports the following specifiers:
//
// - d/i: Signed decimal integer
// - u: Unsigned decimal integer
// - o: Unsigned octal
// - x: Unsigned hexadecimal integer
// - X: Unsigned hexadecimal integer (uppercase)
// - c: Character
// - s: String of characters
// - p: Pointer address
// - %: Literal '%'
//
// None of the sub-specifiers are supported for the sake of simplicity.
// The `n` specifier is not supported since that is a major source of
// security vulnerabilities. None of the floating-point specifiers are
// supported since floating point operations don't make sense in kernel
// space
//
// Anyway, this subset should suffice for printf debugging
void kvprintf(const char *format, va_list arg)
{
    while (*format)
    {
        if (*format == '%')
        {
            ++format;
            if (!*format)
                return;
            switch (*format)
            {
            case 'd':
            case 'i':
            {
                int n = va_arg(arg, int);
                if (n == INT_MIN)
                {
                    kprint("-2147483648");
                    break;
                }
                if (n < 0)
                {
                    uart_put('-');
                    n = ~n + 1;
                }
                char lsh = '0' + n % 10;
                n /= 10;
                char buf[9];
                char *p_buf = buf;
                while (n)
                {
                    *p_buf++ = '0' + n % 10;
                    n /= 10;
                }
                while (p_buf != buf)
                    uart_put(*--p_buf);
                uart_put(lsh);
            }
            break;
            case 'u':
            {
                unsigned n = va_arg(arg, unsigned);
                char lsh = '0' + n % 10;
                n /= 10;
                char buf[9];
                char *p_buf = buf;
                while (n)
                {
                    *p_buf++ = '0' + n % 10;
                    n /= 10;
                }
                while (p_buf != buf)
                    uart_put(*--p_buf);
                uart_put(lsh);
            }
            break;
            case 'o':
            {
                unsigned n = va_arg(arg, unsigned);
                char lsh = '0' + n % 8;
                n /= 8;
                char buf[10];
                char *p_buf = buf;
                while (n)
                {
                    *p_buf++ = '0' + n % 8;
                    n /= 8;
                }
                while (p_buf != buf)
                    uart_put(*--p_buf);
                uart_put(lsh);
            }
            break;
            case 'x':
            {
                unsigned n = va_arg(arg, unsigned);
                char lsh = to_hex_digit(n % 16);
                n /= 16;
                char buf[7];
                char *p_buf = buf;
                while (n)
                {
                    *p_buf++ = to_hex_digit(n % 16);
                    n /= 16;
                }
                while (p_buf != buf)
                    uart_put(*--p_buf);
                uart_put(lsh);
            }
            break;
            case 'X':
            {
                unsigned n = va_arg(arg, unsigned);
                char lsh = to_hex_digit(n % 16);
                n /= 16;
                char buf[7];
                char *p_buf = buf;
                while (n)
                {
                    *p_buf++ = to_hex_digit(n % 16);
                    n /= 16;
                }
                while (p_buf != buf)
                    uart_put(toupper(*--p_buf));
                uart_put(toupper(lsh));
            }
            break;
            case 'c':
                uart_put(va_arg(arg, int));
                break;
            case 's':
                kprint(va_arg(arg, char *));
                break;
            case 'p':
            {
                kprint("0x");
                size_t ptr = va_arg(arg, size_t);
                char lsh = to_hex_digit(ptr % 16);
                ptr /= 16;
                char buf[15];
                char *p_buf = buf;
                while (ptr)
                {
                    *p_buf++ = to_hex_digit(ptr % 16);
                    ptr /= 16;
                }
                while (p_buf != buf)
                    uart_put(*--p_buf);
                uart_put(lsh);
            }
            break;
            case '%':
                uart_put('%');
                break;
            default:
                uart_put('%');
                uart_put(*format);
            }
        }
        else
            uart_put(*format);
        ++format;
    }
}

void kprintf(const char *format, ...)
{
    va_list arg;
    va_start(arg, format);
    kvprintf(format, arg);
    va_end(arg);
}