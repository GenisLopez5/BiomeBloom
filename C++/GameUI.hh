#include "Canvas.hh"

#pragma once

using namespace sf;

class GameUI : public Canvas {
  private:
    Text _PaletteText;
    Sprite _PaletteImg;
    Sprite _PaletteSelectedImg;

  public:
    GameUI(int size_x, int size_y);
};
