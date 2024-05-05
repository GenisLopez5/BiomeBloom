#include "Renderer.hh"
#include "Types.hh"
#include <SFML/Graphics/Glsl.hpp>
#include <SFML/Graphics/RenderStates.hpp>
#include <chrono>
#include <iostream>

Renderer::Renderer(int atom_size)
    : window(sf::VideoMode(600, 800), "BiomeBloom") {
    default_texture.loadFromFile("../data/default.png");

    rows = window.getSize().y / atom_size;
    cols = window.getSize().x / atom_size;

    // SETUP texture[0] DEFAULT TEXTURE

    typetoshader[Types::TERRA].loadFromFile("../data/shaders/terra.glsl",
                                            sf::Shader::Type::Fragment);
    typetoshader[Types::HERBA].loadFromFile("../data/shaders/herba.glsl",
                                            sf::Shader::Type::Fragment);
    typetoshader[Types::FOC].loadFromFile("../data/shaders/foc.glsl",
                                          sf::Shader::Type::Fragment);
    typetoshader[Types::AIGUA].loadFromFile("../data/shaders/aigua.glsl",
                                            sf::Shader::Type::Fragment);
    typetoshader[Types::FORMIGA].loadFromFile("../data/shaders/formiga.glsl",
                                              sf::Shader::Type::Fragment);

    // SETUP SPRITES VECTOR (ALL START BEING DEFAULT)
    sf::Vector2f pos = {0, 0};

    sprites.resize(rows * cols);
    for (int i = 0; i < sprites.size(); ++i) {
        pos.x = (i % cols) * atom_size;
        pos.y = int(i / cols) * atom_size;
        sprites[i] = sf::Sprite(default_texture);
        sprites[i].setPosition(pos);
        sprites[i].scale(atom_size / (double)default_texture.getSize().x,
                         atom_size / (double)default_texture.getSize().y);
    }

    // SETUP RENDER BUFFER
    render_buffer = new DAtom[rows * cols];
}

void Renderer::render(int64_t *float_fields) {

    for (int i = 0; i < rows * cols; ++i) {

        float time =
            (float)std::chrono::duration_cast<std::chrono::milliseconds>(
                std::chrono::steady_clock::now().time_since_epoch())
                .count();

        sf::RenderStates rendStates(&typetoshader[render_buffer[i].material]);
        typetoshader[Types::AIGUA].setUniform("time", time);
        typetoshader[Types::FOC].setUniform("time", time);

        typetoshader[render_buffer[i].material].setUniform(
            "height", (float)float_fields[i] / 20.0f);

        window.draw(sprites[i], rendStates);
    }
}

void Renderer::renderCanvas(Canvas &canvas) {

    for (const auto &sprite : canvas.SpriteMap) {
        window.draw(sprite.second.sprite);
    }
}

int Renderer::getRows() const { return rows; };
int Renderer::getCols() const { return cols; };
