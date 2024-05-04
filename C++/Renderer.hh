#include "Canvas.hh"
#include "DAtom.hh"
#include "SFML/Graphics.hpp"
#include <vector>

#pragma once

using namespace std;

class Renderer {
  private:
    int rows;
    int cols;
    vector<sf::Shader> shaders;
    vector<sf::Sprite> sprites;
    vector<sf::Texture> textures;

    void set_new_texture(DAtom &d_atom, sf::Sprite &sprite);

  public:
    sf::RenderWindow window;
    DAtom *render_buffer;

    Renderer(int atom_size);

    void render();
    void renderCanvas(Canvas &canvas);
    int getRows() const;
    int getCols() const;
};
