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

        _PaletteText = addText("_PaletteImg", "Selected material: ", "../data/defaultFont.otf", 30,
                BottomMiddle, sf::Vector2f(0.5, 0));

        _PaletteImg = addSprite("x_PaletteImg", "../data/palette.png", BottomMiddle,
                sf::Vector2f(0.5, 1));

        _PaletteSelectedImg = addSprite("y_PaletteSelectedImg", "../data/PaletteSelected.png",
                sf::Vector2f(BottomMiddle.x - 75 * 4, BottomMiddle.y),
                sf::Vector2f(0, 1));

        addSprite("sprite0", "../data/sprite0", sf::Vector2f(BottomMiddle.x - 75*4, BottomMiddle.y), sf::Vector2f(0,1));
        addSprite("sprite1", "../data/sprite1", sf::Vector2f(BottomMiddle.x - 75*3, BottomMiddle.y), sf::Vector2f(0,1));
        addSprite("sprite2", "../data/sprite2", sf::Vector2f(BottomMiddle.x - 75*2, BottomMiddle.y), sf::Vector2f(0,1));
        addSprite("sprite3", "../data/sprite3", sf::Vector2f(BottomMiddle.x - 75*1, BottomMiddle.y), sf::Vector2f(0,1));
        addSprite("sprite4", "../data/sprite4", sf::Vector2f(BottomMiddle.x - 75*0, BottomMiddle.y), sf::Vector2f(0,1));
        addSprite("sprite5", "../data/sprite5", sf::Vector2f(BottomMiddle.x + 75*1, BottomMiddle.y), sf::Vector2f(0,1));
        addSprite("sprite6", "../data/sprite6", sf::Vector2f(BottomMiddle.x + 75*2, BottomMiddle.y), sf::Vector2f(0,1));
        addSprite("sprite7", "../data/sprite7", sf::Vector2f(BottomMiddle.x + 75*3, BottomMiddle.y), sf::Vector2f(0,1));
        addSprite("sprite8", "../data/sprite8", sf::Vector2f(BottomMiddle.x + 75*4, BottomMiddle.y), sf::Vector2f(0,1));
    }

    void ChangedMaterial(int label_id)
    {
        _PaletteSelectedImg.setPosition(75*label_id, _PaletteSelectedImg.getPosition().y);
    }
};
