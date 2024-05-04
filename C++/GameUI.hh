#include "Canvas.hh"

using namespace sf;

class GameUI : public Canvas{
    private:
        Text _PaletteText;
        Sprite _PaletteImg;
        Sprite _PaletteSelectedImg;

        void Setup()
        {
            _PaletteText = addText("_PaletteImg", "Selected material: ", "../data/defaultFont.otf", 30, BottomMiddle + sf::Vector2f(50, 50), sf::Vector2f(0.5,0));
            _PaletteImg = addSprite("_PaletteImg", "../data/palette.png", BottomMiddle, sf::Vector2f(0.5, 0));
            _PaletteSelectedImg = addSprite("_PaletteSelectedImg", "../data/PaletteSelected.png", sf::Vector2f(BottomMiddle.x - 75*4, BottomMiddle.y), sf::Vector2f(0.5, 0));
        }
    public:
        GameUI(int size_x, int size_y) : Canvas(size_x, size_y){}
};