#include "mem.h"

void zero_bytes(void *ptr, size_t num_bytes)
{
memset(ptr, 0, num_bytes);
}

void memset(void *ptr, u8 val, size_t len)
{
  for (size_t i = 0; i < len; i++)
  {
    *((char*)ptr++) = val;
  }
}