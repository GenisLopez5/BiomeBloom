#include "DAtom.hh"
#include "SFML/Graphics.hpp"
#include <iostream>
#include <vector>

using namespace std;
using namespace sf;

class Renderer{
    private:
        int render_buffer_size;
        vector<Sprite> sprites;
        vector<Texture> textures;

        void set_new_texture(const DAtom& d_atom, Sprite& sprite);
    public:
        RenderWindow window;
        DAtom* render_buffer;

public:
  RenderWindow window;
  DAtom *render_buffer;

  Renderer(int render_buffer_size);

  void render();
};
