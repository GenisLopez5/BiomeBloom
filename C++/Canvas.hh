#include <SFML/Graphics.hpp>
#include <iostream>
#include <map>
#include <string>

#pragma once

using namespace std;

class Canvas {
  private:
    int size_x;
    int size_y;

    const static int native_witdth = 1920;
    const static int native_height = 1080;

  public:
    struct TexturedSprite {
        sf::Sprite sprite;
        sf::Texture texture;
    };
    map<string, TexturedSprite> SpriteMap;
    map<string, sf::Text> TextMap;

    sf::Vector2f topMiddle;
    sf::Vector2f topLeft;
    sf::Vector2f topRight;
    sf::Vector2f MiddleLeft;
    sf::Vector2f MiddleMiddle;
    sf::Vector2f MiddleRight;
    sf::Vector2f BottomLeft;
    sf::Vector2f BottomMiddle;
    sf::Vector2f BottomRight;

    Canvas(int size_x, int size_y);
    sf::Text& addText(string textName, string content, string fontDir,
                 unsigned int size, sf::Vector2f pos, sf::Vector2f anchor);
    sf::Sprite& addSprite(string spriteName, string textureDir, sf::Vector2f pos,
                   sf::Vector2f anchor, float scale = 1);
    void removeText(string textName);
    void removeSprite(string spriteText);
};
