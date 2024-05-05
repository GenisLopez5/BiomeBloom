#include "DAtom.hh"
#include "MouseInfo.hh"
#include <cstdint>

extern "C" void compute(DAtom *ptr, int64_t buffer_width, int64_t buffer_height,
                        MouseInfo mouse, int64_t *buffers);
extern "C" void update_mouse(MouseInfo mouse, DAtom *ptr, int64_t buffer_width,
                             int64_t buffer_height);
extern "C" void reset_buffer();