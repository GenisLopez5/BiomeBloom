#include "Canvas.hh"
#include <SFML/Graphics/Sprite.hpp>
#include <SFML/System/Vector2.hpp>

#pragma once

using namespace sf;

class GameUI : public Canvas {
  private:
    string _PaletteText = "_PaletteText";
    string _PaletteImg = "_PaletteImg";
    string _PaletteSelectedImg = "z_PaletteSelectedImg";

  public:
    map<string, int> nametotag;
    GameUI(int size_x, int size_y) : Canvas(size_x, size_y) {
        nametotag = {{"sprite0", 0}, {"sprite3", 3}, {"sprite6", 6},
                     {"sprite1", 1}, {"sprite4", 4}, {"sprite7", 7},
                     {"sprite8", 8}, {"sprite2", 2}, {"sprite5", 5}};

        addText(_PaletteText, "Selected material: ", "../data/defaultFont.otf",
                30, BottomMiddle, sf::Vector2f(0.5, 0));

        addSprite(_PaletteImg, "../data/palette.png", BottomMiddle,
                  sf::Vector2f(0.5, 1), 1, false);

        addSprite(_PaletteSelectedImg, "../data/PaletteSelected.png",
                  sf::Vector2f(BottomMiddle.x - 75 * 2, BottomMiddle.y),
                  sf::Vector2f(0.5, 1), 1, false);

        addSprite("sprite0", "../data/sprite0.png",
                  sf::Vector2f(BottomMiddle.x - 75 * 2, BottomMiddle.y),
                  sf::Vector2f(0.5, 1));
        addSprite("sprite1", "../data/sprite1.png",
                  sf::Vector2f(BottomMiddle.x - 75 * 1, BottomMiddle.y),
                  sf::Vector2f(0.5, 1));
        addSprite("sprite2", "../data/sprite2.png",
                  sf::Vector2f(BottomMiddle.x - 75 * 0, BottomMiddle.y),
                  sf::Vector2f(0.5, 1));
        addSprite("sprite3", "../data/sprite3.png",
                  sf::Vector2f(BottomMiddle.x + 75 * 1, BottomMiddle.y),
                  sf::Vector2f(0.5, 1));
        addSprite("sprite4", "../data/sprite4.png",
                  sf::Vector2f(BottomMiddle.x + 75 * 2, BottomMiddle.y),
                  sf::Vector2f(0.5, 1));
    }

    int manageInput(Vector2i pos) {
        for (auto &sprite : SpriteMap) {
            if (intersectElement(sprite.first, pos)) {
                cout << "intersectig with: " << sprite.first << endl;
                ChangedMaterial(nametotag[sprite.first]);
                return nametotag[sprite.first];
            }
        }
        return -1;
    }

    void ChangedMaterial(int label_id) {
        SpriteMap[_PaletteSelectedImg].sprite.setPosition(
            BottomMiddle.x - 75 * 2 + 75 * label_id,
            SpriteMap[_PaletteSelectedImg].sprite.getPosition().y);
    }
};
