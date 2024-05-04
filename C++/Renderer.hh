#include "DAtom.hh"
#include "SFML/Graphics.hpp"
#include <iostream>
#include <vector>

using namespace std;

class Renderer {
private:
  int render_buffer_size;
  vector<sf::Sprite> sprites;
  vector<sf::Texture> textures;

  void set_new_texture(const DAtom &d_atom, sf::Sprite &sprite);

public:
  sf::RenderWindow window;
  DAtom *render_buffer;

  Renderer(int render_buffer_size);

  void render();
};
