#include "DAtom.hh"
#include "MouseInfo.hh"
extern "C" void compute(DAtom *ptr, int buffer_width, int buffer_height);
extern "C" void update_mouse(DAtom *ptr, MouseInfo mouse);