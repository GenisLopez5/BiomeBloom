#include "Renderer.hh"

void test(sf::RenderWindow& window)
{
    sf::Texture t;
    t.loadFromFile("./../Textures/HappyFace.png", sf::IntRect(10,10,20,20));

    sf::Sprite s;
    s.setTexture(t);
    
    window.draw(s);
}

Renderer::Renderer(int render_buffer_size) : sprites(render_buffer_size)
{
    this->render_buffer_size = render_buffer_size;

    Atom* render_buffer = new Atom[render_buffer_size];

    RenderWindow window(sf::VideoMode(800, 600), "BiomeBloom");
}

void Renderer::render()
{
    window.clear();

    test(window);

    for (int i = 0; i < render_buffer_size; ++i)
    {
        if (render_buffer[i].obsolete)
        {

        }
        
        window.draw(sprites[i]);
    }

    window.display();
}