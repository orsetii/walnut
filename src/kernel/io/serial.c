#include "serial.h"

void initialize_uart()
{
    // Set word length
    mmio_writeb(NS16550_MMIO_ADDR, 3, 0b11);

    // Enable FIFO
    mmio_writeb(NS16550_MMIO_ADDR, 2, 1);

    // Enable receiver buffer interrupts
    mmio_writeb(NS16550_MMIO_ADDR, 1, 1);
}

void uart_put(char c)
{
    mmio_writeb(NS16550_MMIO_ADDR, 0, c);
}

char uart_get()
{
    return uart_has_data() ? mmio_readb(NS16550_MMIO_ADDR, 0)
                           : 0;
}

char uart_get_blocking()
{
    while (!uart_has_data())
    {
    }
    return mmio_readb(NS16550_MMIO_ADDR, 0);
}

void uart_print(const char *str)
{
    while (*str)
    {
        uart_put((int)*str);
        str++;
    }
}
void uart_println(const char *str)
{
    while (*str)
    {
        uart_put((int)*str);
        str++;
    }
    uart_put('\n');
}

int uart_has_data()
{
    return (mmio_readb(NS16550_MMIO_ADDR, 0) & 1) == 1;
}

// Currently not used/needed
void set_divisor()
{
    // If we cared about the divisor, the code below would set the divisor
    // from a global clock rate of 22.729 MHz (22,729,000 cycles per second)
    // to a signaling rate of 2400 (BAUD). We usually have much faster signalling
    // rates nowadays, but this demonstrates what the divisor actually does.
    // The formula given in the NS16500A specification for calculating the divisor
    // is:
    // divisor = ceil( (clock_hz) / (baud_sps x 16) )
    // So, we substitute our values and get:
    // divisor = ceil( 22_729_000 / (2400 x 16) )
    // divisor = ceil( 22_729_000 / 38_400 )
    // divisor = ceil( 591.901 ) = 592

    // The divisor register is two bytes (16 bits), so we need to split the value
    // 592 into two bytes. Typically, we would calculate this based on measuring
    // the clock rate, but again, for our purposes [qemu], this doesn't really do
    // anything.
    const u64 divisor = 592;
    const u64 divisor_least = divisor & 0xff;
    const u64 divisor_most = divisor >> 8;

    // Notice that the divisor register DLL (divisor latch least) and DLM (divisor
    // latch most) have the same base address as the receiver/transmitter and the
    // interrupt enable register. To change what the base address points to, we
    // open the "divisor latch" by writing 1 into the Divisor Latch Access Bit
    // (DLAB), which is bit index 7 of the Line Control Register (LCR) which
    // is at base_address + 3.
    mmio_writeb(NS16550_MMIO_ADDR, 3, 0b1000011);

    // Now, base addresses 0 and 1 point to DLL and DLM, respectively.
    // Put the lower 8 bits of the divisor into DLL
    mmio_writeb(NS16550_MMIO_ADDR, 0, divisor_least);
    mmio_writeb(NS16550_MMIO_ADDR, 1, divisor_most);

    // Close the Divisor Latch
    mmio_writeb(NS16550_MMIO_ADDR, 3, 0b11);
}