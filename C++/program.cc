#include "MouseInfo.hh"
#include "Renderer.hh"
#include "rust_functions.h"

using namespace std;

int main(int argc, char *argv[]) {
  int size_x = 5;
  int size_y = 6;

  Renderer renderer(size_x * size_y);

  while (renderer.window.isOpen()) {
    cout << "rendering frame" << endl;
    MouseInfo mouse = (MouseInfo){0, 0, false};
    compute(renderer.render_buffer, size_x, size_y, mouse);
    cout << "ending coumpute" << endl;
    renderer.render();
  }
}
