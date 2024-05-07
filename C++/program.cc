#include "GameUI.hh"
#include "MouseInfo.hh"
#include "Renderer.hh"
#include "rust_functions.h"
#include <SFML/Window/Event.hpp>
#include <SFML/Window/Mouse.hpp>
#include <chrono>
#include <cstdint>
#include <iostream>

using namespace std;

const int SIZE = 20;
const int NUM_OF_FIELDS = 3;

int main(int argc, char *argv[]) {

    Renderer renderer(SIZE);

    GameUI gameUI(renderer.window.getSize().x, renderer.window.getSize().y);

    MouseInfo mouse;
    mouse.selected_tag = 1;

    cout << "[INFO (C++)]: Creant fields" << endl;
    int64_t *floatFields = // <--- Mega buffer
        (int64_t *)malloc(sizeof(int64_t) * NUM_OF_FIELDS * renderer.getCols() *
                          renderer.getRows());
    for (int i = 0; i < NUM_OF_FIELDS * renderer.getCols() * renderer.getRows();
         ++i) {
        int row = i / renderer.getCols();
        int col = i % renderer.getCols();
        floatFields[i] = abs(col - renderer.getCols() / 2) +
                         abs(row - renderer.getRows() / 2);
    }
    // TODO: Inicialitzeu els valors amb un funció continua aquí

    cout << "[INFO (C++)]: Fields creats" << endl;
    sf::Event event;

    auto last_compute = chrono::steady_clock::now();
    bool paused = false;
    int miliseconds = 500;
    while (renderer.window.isOpen()) {
        // Window events
        while (renderer.window.pollEvent(event))
            if (event.type == sf::Event::Closed) {
                cout << "Closing window" << endl;
                renderer.window.close();
            }

        if (sf::Keyboard::isKeyPressed(sf::Keyboard::Space))
            paused = true;
        if (sf::Keyboard::isKeyPressed(sf::Keyboard::Enter))
            paused = false;
        if (sf::Keyboard::isKeyPressed(sf::Keyboard::R))
            reset_buffer(renderer.render_buffer,
                         renderer.getCols() * renderer.getRows());
        if (sf::Keyboard::isKeyPressed(sf::Keyboard::Up))
            miliseconds -= miliseconds > 10 ? 5 : 0;
        if (sf::Keyboard::isKeyPressed(sf::Keyboard::Down))
            miliseconds += 5;

        // Mouse events
        if (sf::Mouse::isButtonPressed(sf::Mouse::Button::Left)) {
            sf::Vector2i mousePos = sf::Mouse::getPosition(renderer.window);

            mouse.posx = mousePos.x / SIZE;
            mouse.posy = mousePos.y / SIZE;

            int tag = gameUI.manageInput(mousePos);
            if (tag >= 0) {
                mouse.selected_tag = tag;
            }

            cout << "calling update mouse: mouse tag" << mouse.selected_tag
                 << endl;

            update_mouse(mouse, renderer.render_buffer, renderer.getCols(),
                         renderer.getRows());
        }

        if (chrono::steady_clock::now() - last_compute >=
                chrono::milliseconds(miliseconds) and
            not paused) {
            sf::Vector2i mousePos = sf::Mouse::getPosition(renderer.window);

            mouse.posx = mousePos.x / SIZE;
            mouse.posy = mousePos.y / SIZE;

            cout << "[INFO (C++)]: first address is: " << floatFields << endl;
            compute(renderer.render_buffer, renderer.getCols(),
                    renderer.getRows(), mouse, floatFields);
            last_compute = chrono::steady_clock::now();
            cout << "end compute" << endl;
        }

        renderer.window.clear();
        renderer.render(floatFields);
        renderer.renderCanvas(gameUI);
        renderer.window.display();
    }
}
