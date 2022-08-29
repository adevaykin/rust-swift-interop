#ifndef INTEROP_H
#define INTEROP_H

#include <stdint.h>
#include <stdbool.h>

void* interop_init();
void interop_destroy(void*);

bool do_job(void* instance, void (*callback)(uint32_t msg));

#endif
