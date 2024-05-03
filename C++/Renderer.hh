#include "Atom.hh"
#include "include/SFML/Graphics.hpp"
#include <vector>
#include <iostream>

using namespace std;
using namespace sf;

class Renderer{
    private:
        int render_buffer_size;
        vector<Sprite> sprites;
    public:
        RenderWindow window;
        Atom* render_buffer;

        Renderer(int render_buffer_size);

        void render();
};