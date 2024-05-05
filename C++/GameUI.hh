#include "Canvas.hh"
#include <SFML/Graphics/Sprite.hpp>
#include <SFML/System/Vector2.hpp>
#include "Button.hh"
#include <list>

#pragma once

using namespace sf;

class GameUI : public Canvas {
  private:
    string _PaletteText = "_PaletteText";
    string _PaletteImg = "_PaletteImg";
    string _PaletteSelectedImg = "_PaletteSelectedImg";
    int64_t* selected_id;

    list<Button<GameUI> > buttons;

  public:
    map<string, int> nametotag;

    static void clicked_mat(GameUI* me, bool value, int custom) 
    {
        me->selected_item(custom); 
    }

    GameUI(int size_x, int size_y, int64_t* selected_id) : Canvas(size_x, size_y) {

        this->selected_id = selected_id;

        nametotag = {{"sprite0", 0}, {"sprite3", 3}, {"sprite6", 6},
                     {"sprite1", 1}, {"sprite4", 4}, {"sprite7", 7},
                     {"sprite8", 8}, {"sprite2", 2}, {"sprite5", 5}};

        addText(_PaletteText, "Selected material: ", "../data/defaultFont.otf",
                30, BottomMiddle, sf::Vector2f(0.5, 0));

        addSprite(_PaletteImg, "../data/palette.png", BottomMiddle,
                  sf::Vector2f(0.5, 1));

        addSprite(_PaletteSelectedImg, "../data/PaletteSelected.png",
                  sf::Vector2f(BottomMiddle.x - 75 * 4, BottomMiddle.y),
                  sf::Vector2f(0, 1));

        addSprite("sprite0", "../data/sprite0.png",
                  sf::Vector2f(BottomMiddle.x - 75 * 2, BottomMiddle.y),
                  sf::Vector2f(0, 1));

        buttons.push_back(Button<GameUI>(Button_Mode::oneclick, SpriteMap["sprite0"].sprite.getGlobalBounds(), 0, &clicked_mat, this));

        addSprite("sprite1", "../data/sprite1.png",
                  sf::Vector2f(BottomMiddle.x - 75 * 1, BottomMiddle.y),
                  sf::Vector2f(0, 1));

        buttons.push_back(Button<GameUI>(Button_Mode::oneclick, SpriteMap["sprite1"].sprite.getGlobalBounds(), 1, &clicked_mat, this));

        addSprite("sprite2", "../data/sprite2.png",
                  sf::Vector2f(BottomMiddle.x + 75 * 0, BottomMiddle.y),
                  sf::Vector2f(0, 1));

        buttons.push_back(Button<GameUI>(Button_Mode::oneclick, SpriteMap["sprite2"].sprite.getGlobalBounds(), 0, &clicked_mat, this));

        addSprite("sprite3", "../data/sprite3.png",
                  sf::Vector2f(BottomMiddle.x + 75 * 1, BottomMiddle.y),
                  sf::Vector2f(0, 1));

        buttons.push_back(Button<GameUI>(Button_Mode::oneclick, SpriteMap["sprite3"].sprite.getGlobalBounds(), 0, &clicked_mat, this));

        addSprite("sprite4", "../data/sprite4.png",
                  sf::Vector2f(BottomMiddle.x + 75 * 2, BottomMiddle.y),
                  sf::Vector2f(0, 1));

        buttons.push_back(Button<GameUI>(Button_Mode::oneclick, SpriteMap["sprite4"].sprite.getGlobalBounds(), 0, &clicked_mat, this));
    }

    void selected_item(int64_t id)
    {
        (*selected_id) = id;
        ChangedMaterial(id);
    }

    void ChangedMaterial(int64_t label_id) {
        SpriteMap[_PaletteSelectedImg].sprite.setPosition(
            BottomMiddle.x - 75 * 2 + 75 * label_id,
            SpriteMap[_PaletteSelectedImg].sprite.getPosition().y);
    }
};
