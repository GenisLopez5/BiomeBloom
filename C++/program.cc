#include "Atom.hh"
#include "Renderer.hh"
#include <iostream>

using namespace std;

int main(int argc, char* argv[])
{
    int size_x = atoi(argv[1]);
    int size_y = atoi(argv[2]);

    Renderer renderer;

    Atom* render_buffer = new Atom[size_x * size_y];

    compute(render_buffer); 
    renderer.render(render_buffer);
}