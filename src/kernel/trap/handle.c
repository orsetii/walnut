#include "trap/handle.h"
#include "print.h"

size_t m_trap(size_t epc, size_t tval, size_t cause, size_t hart_id, size_t status)
{
    // kprintf("epc=0x%x tval=%d cause=%d hart_id=%x status=%d\n", epc, tval, cause, hart_id, status);

    if (hart_id != 0) {
        while (1) { }
    }

    if (cause >> 63) {
        m_handle_interrupt();
        return epc;
    } else {
        return m_handle_exception(epc);
    }
}

size_t m_handle_exception(size_t epc)
{
    // kprintf("IS EXCEPTION");
    return epc + 4;
}

void m_handle_interrupt()
{
    // kprintf("IS INTERRUPT");
}

size_t s_trap(size_t epc, size_t tval, size_t cause, size_t hart_id, size_t status)
{
    // kprintf("epc=0x%x tval=%d cause=%d hart_id=%x status=%d\n", epc, tval, cause, hart_id, status);

    if (hart_id != 0) {
        while (1) { }
    }

    if (cause >> 63) {
        m_handle_interrupt();
        return epc;
    } else {
        return m_handle_exception(epc);
    }
}

size_t s_handle_exception(size_t epc)
{
    // kprintf("IS EXCEPTION");
    return epc + 4;
}

void s_handle_interrupt()
{
    // kprintf("IS INTERRUPT");
}
