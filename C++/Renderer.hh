#include "DAtom.hh"
#include "SFML/Graphics.hpp"
#include <vector>

using namespace std;

class Renderer {
  private:
    int rows;
    int cols;
    vector<sf::Sprite> sprites;
    vector<sf::Texture> textures;

    void set_new_texture(const DAtom &d_atom, sf::Sprite &sprite);

  public:
    sf::RenderWindow window;
    DAtom *render_buffer;

    Renderer(int atom_size);

    void render();
    int getRows() const;
    int getCols() const;
};
