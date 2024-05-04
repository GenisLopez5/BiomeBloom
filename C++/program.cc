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

    cout << "[INFO (C++)]: Creant fields" << endl;
    int64_t* floatFields =  // <--- Mega buffer
        (int64_t*)malloc(sizeof(int64_t) * NUM_OF_FIELDS*renderer.getCols()*renderer.getRows());
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

            mousePos.x = mousePos.x / SIZE;
            mousePos.y = mousePos.y / SIZE;

            MouseInfo mouse;
            mouse.posx = mousePos.x;
            mouse.posy = mousePos.y;

            mouse.selected_tag = 1;
            cout << "calling update mouse" << endl;

            update_mouse(mouse, renderer.render_buffer, renderer.getCols(),
                         renderer.getRows());
        }

        if (chrono::steady_clock::now() - last_compute >= chrono::seconds(1)) {
            sf::Vector2i mousePos = sf::Mouse::getPosition(renderer.window);

            mousePos.x = mousePos.x / SIZE;
            mousePos.y = mousePos.y / SIZE;

            MouseInfo mouse;
            mouse.posx = mousePos.x;
            mouse.posy = mousePos.y;

            mouse.selected_tag = 1;
            cout << "first address is: " << floatFields << endl;
            compute(renderer.render_buffer, renderer.getCols(),
                    renderer.getRows(), mouse, floatFields);
            last_compute = chrono::steady_clock::now();
            cout << "end compute" << endl;
        }

        renderer.render();
        renderer.renderCanvas(gameUI);
    }
}
