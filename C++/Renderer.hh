#include "Canvas.hh"
#include "DAtom.hh"
#include "SFML/Graphics.hpp"
#include "Types.hh"
#include <map>
#include <vector>

#pragma once

using namespace std;

class Renderer {
  private:
    int rows;
    int cols;
    vector<sf::Sprite> sprites;
    sf::Texture antTexture;
    sf::Texture default_texture;

  public:
    map<int64_t, sf::Shader> typetoshader;

    sf::RenderWindow window;
    DAtom *render_buffer;

    Renderer(int atom_size);

    void render(int64_t *float_fields);
    void renderCanvas(Canvas &canvas);
    int getRows() const;
    int getCols() const;
};
