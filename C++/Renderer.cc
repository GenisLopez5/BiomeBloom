#include "Renderer.hh"
#include "Types.hh"
#include <SFML/Graphics/Glsl.hpp>
#include <iostream>

Renderer::Renderer(int atom_size)
    : textures(2), window(sf::VideoMode(600, 800), "BiomeBloom"), shaders(2) {

    rows = window.getSize().y / atom_size;
    cols = window.getSize().x / atom_size;

    // SETUP texture[0] DEFAULT TEXTURE
    textures[0].loadFromFile("../data/default.png");
    textures[1].loadFromFile("../data/ant_texture.png");
    shaders[0].loadFromFile("../data/mud_shader.glsl",
                            sf::Shader::Type::Fragment);
    shaders[1].loadFromFile("../data/grass.glsl", sf::Shader::Type::Fragment);

    // SETUP SPRITES VECTOR (ALL START BEING DEFAULT)
    sf::Vector2f pos = {0, 0};

    sprites.resize(rows * cols);
    for (int i = 0; i < sprites.size(); ++i) {
        pos.x = (i % cols) * atom_size;
        pos.y = int(i / cols) * atom_size;
        sprites[i] = sf::Sprite(textures[0]);
        sprites[i].setPosition(pos);
        sprites[i].scale(atom_size / (double)textures[0].getSize().x,
                         atom_size / (double)textures[0].getSize().y);
    }

    // SETUP RENDER BUFFER
    render_buffer = new DAtom[rows * cols];
}

void Renderer::set_new_texture(DAtom &d_atom, sf::Sprite &sprite) {
    switch (d_atom.material) {
    case Types::ANT:
        sprite.setTexture(textures[1]);
        break;
    case Types::DEFAULT:
        sprite.setTexture(textures[0]);
        break;
    }

    d_atom.obsolete = false;
}

void Renderer::render() {

    for (int i = 0; i < rows * cols; ++i) {
        if (render_buffer[i].obsolete) {
            set_new_texture(render_buffer[i], sprites[i]);
        }

        if (render_buffer[i].material == Types::ANT) {
            window.draw(sprites[i], &shaders[0]);
        } else if (render_buffer[i].material == Types::DEFAULT) {
            window.draw(sprites[i], &shaders[1]);
        }
    }
}

void Renderer::renderCanvas(Canvas &canvas) {

    for (const auto &sprite : canvas.SpriteMap) {
        /* cout << "sprite pos: " << sprite.second.sprite.getPosition().y <<
         * endl; */
        /* cout << "renderg sprite: " << sprite.first << endl; */
        window.draw(sprite.second.sprite);
    }

    /* cout << "sprites drown" << endl; */

    /* for (const auto &text : canvas.TextMap) {
        window.draw(text.second);
    } */

    /* cout << "text_drown drown" << endl; */
}

int Renderer::getRows() const { return rows; };
int Renderer::getCols() const { return cols; };
