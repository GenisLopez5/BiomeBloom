#pragma once

#include <SFML/Graphics.hpp>
#include <iostream>
#include <list>

using namespace sf;
using namespace std;

enum Button_Mode{
    toggle,
    oneclick
};

template <typename T>
class Button{
    private:
        static list<Button> buttons;

        Button_Mode mode;
        bool isPressed;
        int custom;
        T* me_p;

        void (*func_to_exec)(T* me, bool value, int custom);
        
        void mouse_clicked()
        {
            if (mode == toggle)
            {
                isPressed = !isPressed;
                func_to_exec(me_p, custom, isPressed);
            }
            else if (mode == oneclick)
            {
                func_to_exec(me_p, custom, true);
            }
        }

    public:
        FloatRect bounds;

        Button(Button_Mode mode, FloatRect bounds, int custom, void (*func_to_exec)(T* me, bool value, int custom), T* me_p)
        {
            this->me_p = me_p;
            this->custom = custom;
            this->func_to_exec = func_to_exec;
            this->bounds = bounds;
            this->mode = mode;
            Button::buttons.push_back(*this);
        }

        static void MouseClicked(Vector2i mousePos)
        {
            for (Button& button : buttons)
            {
                if (button.bounds.contains(mousePos.x, mousePos.y))
                {
                    button.mouse_clicked();
                }
            }
        }
};