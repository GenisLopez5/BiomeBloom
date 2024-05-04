#include "MouseInfo.hh"
#include "Renderer.hh"
#include "rust_functions.h"
#include <SFML/Window/Event.hpp>
#include <SFML/Window/Mouse.hpp>
#include <ios>
#include <iostream>

using namespace std;

const int SIZE = 20;

int main(int argc, char *argv[]) {

    Renderer renderer(SIZE);

    sf::Event event;

    while (renderer.window.isOpen()) {

        renderer.window.pollEvent(event);
        if (event.type == sf::Event::Closed) {
            renderer.window.close();
        }

        sf::Vector2i mousePos = sf::Mouse::getPosition(renderer.window);
        mousePos.x = mousePos.x / SIZE;
        mousePos.y = mousePos.y / SIZE;
        cout << "mouse_x: " << mousePos.x << " mouse_y:" << mousePos.y << endl;
        bool leftPressed = sf::Mouse::isButtonPressed(sf::Mouse::Button::Left);
        MouseInfo mouse = (MouseInfo){mousePos.x, mousePos.y, leftPressed};

        compute(renderer.render_buffer, renderer.getCols(), renderer.getRows(),
                mouse);
        renderer.render();
    }
}
