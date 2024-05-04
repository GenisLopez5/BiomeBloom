#include "MouseInfo.hh"
#include "Renderer.hh"
#include "rust_functions.h"
#include <SFML/Window/Event.hpp>
#include <SFML/Window/Mouse.hpp>
#include <chrono>
#include <iostream>

using namespace std;

const int SIZE = 20;

int main(int argc, char *argv[]) {

    Renderer renderer(SIZE);

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
        sf::Vector2i mousePos = sf::Mouse::getPosition(renderer.window);

        mousePos.x = mousePos.x / SIZE;
        mousePos.y = mousePos.y / SIZE;

        MouseInfo mouse;
        mouse.posx = mousePos.x;
        mouse.posy = mousePos.y;
        mouse.clicked = false;
        mouse.selected_tag = 1;

        if (sf::Mouse::isButtonPressed(sf::Mouse::Button::Left)) {
            mouse.clicked = true;
        }

        if (chrono::steady_clock::now() - last_compute >= chrono::seconds(1)) {
            compute(renderer.render_buffer, renderer.getCols(),
                    renderer.getRows(), mouse);
            last_compute = chrono::steady_clock::now();
            cout << "end compute" << endl;
        }
        renderer.render();
    }
}
