#include "DAtom.hh"
// #include "Renderer.hh"
#include "rust_functions.h"
#include <iostream>
#include <string>

using namespace std;

int main() {
  int size_x = 5;
  int size_y = 5;

  // Renderer renderer;

  DAtom *render_buffer = new DAtom[size_x * size_y];

  compute(render_buffer, size_x, size_y);

  // compute(render_buffer, size_x * size_y);
  // renderer.render(render_buffer);

  cout << render_buffer[0].material << endl;
}
