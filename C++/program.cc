#include "MouseInfo.hh"
#include "Renderer.hh"
#include "rust_functions.h"
#include <SFML/Window/Event.hpp>
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
        MouseInfo mouse = (MouseInfo){0, 0, false};
        compute(renderer.render_buffer, renderer.getRows(), renderer.getCols(),
                mouse);
        renderer.render();
    }
}
