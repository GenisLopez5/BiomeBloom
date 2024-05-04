#include "Canvas.hh"

#pragma once

using namespace sf;

class GameUI : public Canvas {
  private:
    Text _PaletteText;
    Sprite _PaletteImg;
    Sprite _PaletteSelectedImg;

  public:
    GameUI(int size_x, int size_y) : Canvas(size_x, size_y) {

        addText("_PaletteImg", "Selected material: ", "../data/defaultFont.otf", 30,
                BottomMiddle, sf::Vector2f(0.5, 0));

        addSprite("_PaletteImg", "../data/palette.png", BottomMiddle,
                sf::Vector2f(0.5, 1));

        addSprite("_PaletteSelectedImg", "../data/PaletteSelected.png",
                sf::Vector2f(BottomMiddle.x - 75 * 4, BottomMiddle.y),
                sf::Vector2f(0, 1));
    }
};