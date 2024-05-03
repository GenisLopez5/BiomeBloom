#include "Atom.hh"
#include "Renderer.hh"
#include "include/SFML/Graphics.hpp"
#include <iostream>

using namespace std;

int main(int argc, char* argv[])
{
    int size_x = atoi(argv[1]);
    int size_y = atoi(argv[2]);

    Renderer renderer(size_x * size_y);

    while (renderer.window.isOpen())
    {
        compute(renderer.render_buffer); 
        renderer.render();
    }
}