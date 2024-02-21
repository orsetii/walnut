#ifndef UTIL_H
#define UTIL_H

int toupper(int c);

#define CAST(type, x) (((union {typeof(x) src; type dst;})(x)).dst)   // gcc


#endif