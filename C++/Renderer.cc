#include "Renderer.hh"
#include "Types.hh"
#include <iostream>

Renderer::Renderer(int atom_size)
    : textures(2), window(sf::VideoMode(600, 800), "BiomeBloom") {

    rows = window.getSize().x / atom_size;
    cols = window.getSize().y / atom_size;

    // SETUP texture[0] DEFAULT TEXTURE
    textures[0].loadFromFile("../data/default.png");
    textures[1].loadFromFile("../data/ant_texture.png");

    // SETUP SPRITES VECTOR (ALL START BEING DEFAULT)
    sf::IntRect rect(0, 0, window.getSize().x / rows,
                     window.getSize().y / cols);

    sprites.resize(rows * cols);
    for (int i = 0; i < sprites.size(); ++i) {
        rect.left = (i % rows) * rect.width;
        rect.top = (i / rows) * rect.height;
        sprites[i] = sf::Sprite(textures[0]);
        sprites[i].setPosition(sf::Vector2f(rect.top, rect.left));
        sprites[i].scale({0.1f, 0.1f});
        cout << "rect: " << rect.top << " " << rect.left << " " << rect.width
             << " " << rect.height << endl;
    }

    // SETUP RENDER BUFFER
    render_buffer = new DAtom[rows * cols];
}

void Renderer::set_new_texture(const DAtom &d_atom, sf::Sprite &sprite) {
    switch (d_atom.material) {
    case Types::ANT:
        sprite.setTexture(textures[1]);
        break;
    case Types::DEFAULT:
        sprite.setTexture(textures[0]);
        break;
    }
}

void Renderer::render() {
    window.clear();

    for (int i = 0; i < rows * cols; ++i) {
        if (render_buffer[i].obsolete) {
            cout << "texture_obsolete!" << endl;
            set_new_texture(render_buffer[i], sprites[i]);
        }

        window.draw(sprites[i]);
    }

    window.display();
}

int Renderer::getRows() const { return rows; };
int Renderer::getCols() const { return cols; };
