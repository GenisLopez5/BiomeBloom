#include "GameUI.hh"

void GameUI::Setup() {
    _PaletteText =
        addText("_PaletteImg", "Selected material: ", "../data/defaultFont.otf",
                30, BottomMiddle + sf::Vector2f(50, 50), sf::Vector2f(0.5, 0));
    _PaletteImg = addSprite("_PaletteImg", "../data/palette.png", BottomMiddle,
                            sf::Vector2f(0.5, 0));
    _PaletteSelectedImg =
        addSprite("_PaletteSelectedImg", "../data/PaletteSelected.png",
                  sf::Vector2f(BottomMiddle.x - 75 * 4, BottomMiddle.y),
                  sf::Vector2f(0.5, 0));
}
