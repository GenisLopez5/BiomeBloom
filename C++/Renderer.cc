#include "Renderer.hh"

Renderer::Renderer(int render_buffer_size)
    : textures(1), window(sf::VideoMode(800, 600), "BiomeBloom") {
  // SETUP texture[0] DEFAULT TEXTURE
  textures[0].loadFromFile("../data/default.png");

  // SETUP SPRITES VECTOR (ALL START BEING DEFAULT)
  sf::Sprite default_sprite(textures[0]);
  vector<sf::Sprite> sprites(render_buffer_size, default_sprite);
  this->sprites = sprites;

  // SETUP RENDER BUFFER
  this->render_buffer_size = render_buffer_size;
  render_buffer = new DAtom[render_buffer_size];
}

void Renderer::set_new_texture(const DAtom &d_atom, sf::Sprite &sprite) {
  switch (d_atom.material) {
  default:
    sprite.setTexture(textures[0]);
    break;
  }
}

void Renderer::render() {
  window.clear();

  for (int i = 0; i < render_buffer_size; ++i) {
    if (render_buffer[i].obsolete)
      set_new_texture(render_buffer[i], sprites[i]);

    window.draw(sprites[i]);
  }

  cout << " hello " << endl;
  window.display();
  cout << " bye " << endl;
}
