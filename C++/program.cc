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

int main(int argc, char *argv[]) {

    Renderer renderer(SIZE);

    GameUI gameUI(renderer.window.getSize().x, renderer.window.getSize().y);

    CFloatPVector floatFields;
    floatFields.ptr = new int64_t *[2];
    floatFields.size = 2;

    floatFields.ptr[0] = new int64_t[renderer.getRows() * renderer.getCols()];
    floatFields.ptr[1] = new int64_t[renderer.getRows() * renderer.getCols()];

    for (int i = 0; i < renderer.getRows() * renderer.getCols(); ++i) {
        floatFields.ptr[0][i] = 42;
        floatFields.ptr[1][i] = 50;
    }

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
            cout << "float should be: " << **floatFields.ptr << endl;
            compute(renderer.render_buffer, renderer.getCols(),
                    renderer.getRows(), mouse, floatFields);
            last_compute = chrono::steady_clock::now();
            cout << "end compute" << endl;
        }

        renderer.render();
        renderer.renderCanvas(gameUI);
    }
}
