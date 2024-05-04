#include "MouseInfo.hh"
#include "Renderer.hh"
#include "rust_functions.h"
#include <SFML/Window/Event.hpp>
#include <SFML/Window/Mouse.hpp>
#include <iostream>

using namespace std;

const int SIZE = 20;

int main(int argc, char *argv[]) {

    Renderer renderer(SIZE);

    sf::Event event;

    while (renderer.window.isOpen()) {

        while (renderer.window.pollEvent(event))
            if (event.type == sf::Event::Closed) {
                cout << "Closing window" << endl;
                renderer.window.close();
            }

        sf::Vector2i mousePos = sf::Mouse::getPosition(renderer.window);

        mousePos.x = mousePos.x / SIZE;
        mousePos.y = mousePos.y / SIZE;

        bool leftPressed = sf::Mouse::isButtonPressed(sf::Mouse::Button::Left);

        MouseInfo mouse;
        mouse.posx = mousePos.x;
        mouse.posy = mousePos.y;
        mouse.clicked = leftPressed;
        mouse.selected_tag = 1;

        compute(renderer.render_buffer, renderer.getCols(), renderer.getRows(),
                mouse);
        cout << "end compute" << endl;
        renderer.render();
    }
}
