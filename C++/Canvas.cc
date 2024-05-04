#include "Canvas.hh"

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

sf::Text &Canvas::addText(string textName, string content, string fontDir,
                          unsigned int size, sf::Vector2f pos,
                          sf::Vector2f anchor) {
    sf::Font f;
    f.loadFromFile(fontDir);
    TextMap[textName] = sf::Text(content, f, size);
    sf::Text &text = TextMap[textName];
    // text.setOrigin(sf::Vector2f(anchor.x * text.))
    text.setPosition(pos);

    return text;
}
sf::Sprite Canvas::addSprite(string spriteName, string textureDir,
                             sf::Vector2f pos, sf::Vector2f anchor,
                             float scale) {
    sf::Texture t;
    t.loadFromFile(textureDir);
    SpriteMap[spriteName] = sf::Sprite(t);
    SpriteMap[spriteName].setOrigin(anchor.x * t.getSize().x,
                                    anchor.y * t.getSize().y);
    SpriteMap[spriteName].setPosition(pos);
    SpriteMap[spriteName].setScale(sf::Vector2f(scale, scale));
    return SpriteMap[spriteName];
}
void Canvas::removeText(string textName) { TextMap.erase(textName); }
void Canvas::removeSprite(string spriteName) { SpriteMap.erase(spriteName); }
const map<string, sf::Sprite> &Canvas::get_sprite_map() const {
    return SpriteMap;
}
const map<string, sf::Text> &Canvas::get_text_map() const { return TextMap; }

void Canvas::Setup() { return; };
