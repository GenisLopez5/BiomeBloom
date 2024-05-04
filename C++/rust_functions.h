#include "DAtom.hh"
#include "MouseInfo.hh"

struct CFloatPVector {
    double **ptr;
    uint64_t size;
};

extern "C" void compute(DAtom *ptr, int64_t buffer_width, int64_t buffer_height,
                        MouseInfo mouse, CFloatPVector buffers);
extern "C" void update_mouse(MouseInfo mouse, DAtom *ptr, int64_t buffer_width,
                             int64_t buffer_height);
