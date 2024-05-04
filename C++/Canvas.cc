#include "Canvas.hh"
#include <SFML/Graphics/Texture.hpp>

Canvas::Canvas(int size_x, int size_y) {
    this->size_x = size_x;
    this->size_y = size_y;

    topLeft = sf::Vector2f(0, 0);
    topMiddle = sf::Vector2f(0.5 * size_x, 0);
    topRight = sf::Vector2f(size_x, 0);
    MiddleLeft = sf::Vector2f(0, 0.5 * size_y);
    MiddleMiddle = sf::Vector2f(0.5 * size_x, 0.5 * size_y);
    MiddleRight = sf::Vector2f(size_x, 0.5 * size_y);
    BottomLeft = sf::Vector2f(0, size_y);
    BottomMiddle = sf::Vector2f(0.5 * size_x, size_y);
    BottomRight = sf::Vector2f(size_x, size_y);
}

void Canvas::addText(string textName, string content, string fontDir,
                     unsigned int size, sf::Vector2f pos, sf::Vector2f anchor) {
    sf::Font f;
    f.loadFromFile(fontDir);
    TextMap[textName] = sf::Text(content, f, size);
    // text.setOrigin(sf::Vector2f(anchor.x * text.))
    TextMap[textName].setPosition(pos);
}
void Canvas::addSprite(string spriteName, string textureDir, sf::Vector2f pos,
                       sf::Vector2f anchor, float scale) {
    SpriteMap[spriteName] = {sf::Sprite(), sf::Texture()};
    SpriteMap[spriteName].texture.loadFromFile(textureDir);
    SpriteMap[spriteName].sprite.setTexture(SpriteMap[spriteName].texture);

    SpriteMap[spriteName].sprite.setOrigin(
        anchor.x * SpriteMap[spriteName].texture.getSize().x,
        anchor.y * SpriteMap[spriteName].texture.getSize().y);

    SpriteMap[spriteName].sprite.setPosition(pos);
    SpriteMap[spriteName].sprite.setScale(sf::Vector2f(scale, scale));
}
void Canvas::removeText(string textName) { TextMap.erase(textName); }
void Canvas::removeSprite(string spriteName) { SpriteMap.erase(spriteName); }
