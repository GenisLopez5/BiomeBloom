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
    mouse.selected_tag = 0;

    cout << "[INFO (C++)]: Creant fields" << endl;
    int64_t *floatFields = // <--- Mega buffer
        (int64_t *)malloc(sizeof(int64_t) * NUM_OF_FIELDS * renderer.getCols() *
                          renderer.getRows());
    for (int i = 0; i < NUM_OF_FIELDS * renderer.getCols() * renderer.getRows();
         ++i) {
        int row = i / renderer.getCols();
        int col = i % renderer.getCols();
        floatFields[i] = abs(col - renderer.getCols() / 2) + abs(row - renderer.getRows() / 2);
    }
    // TODO: Inicialitzeu els valors amb un funció continua aquí

    cout << "[INFO (C++)]: Fields creats" << endl;
    sf::Event event;

    auto last_compute = chrono::steady_clock::now();
    while (renderer.window.isOpen()) {
        // Window events
        while (renderer.window.pollEvent(event))
            if (event.type == sf::Event::Closed) {
                cout << "Closing window" << endl;
                renderer.window.close();
            }

        // Mouse events
        if (sf::Mouse::isButtonPressed(sf::Mouse::Button::Left)) {
            sf::Vector2i mousePos = sf::Mouse::getPosition(renderer.window);

            mouse.posx = mousePos.x / SIZE;
            mouse.posy = mousePos.y / SIZE;

            if (mousePos.y < renderer.window.getSize().y - 75)
            {
                mouse.selected_tag = mouse.posx / (renderer.window.getSize().x / 9);
                gameUI.ChangedMaterial(mouse.selected_tag);
            }

            cout << "calling update mouse" << endl;

            update_mouse(mouse, renderer.render_buffer, renderer.getCols(),
                         renderer.getRows());
        }

        if (chrono::steady_clock::now() - last_compute >= chrono::seconds(1)) {
            sf::Vector2i mousePos = sf::Mouse::getPosition(renderer.window);

            mouse.posx = mousePos.x / SIZE;
            mouse.posy = mousePos.y / SIZE;

            cout << "first address is: " << floatFields << endl;
            compute(renderer.render_buffer, renderer.getCols(),
                    renderer.getRows(), mouse, floatFields);
            last_compute = chrono::steady_clock::now();
            cout << "end compute" << endl;
        }

        renderer.window.clear();
        renderer.render();
        renderer.renderCanvas(gameUI);
        renderer.window.display();
    }
}
