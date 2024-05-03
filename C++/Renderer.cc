#include "Renderer.hh"

void test(RenderWindow& window)
{
    Texture t;
    t.loadFromFile("./../Textures/HappyFace.png", IntRect(Vector2<int>(10, 10), Vector2<int>(20, 20)));

    Sprite s(t);
    window.draw(s);
}

Renderer::Renderer(int render_buffer_size) : sprites(render_buffer_size)
{
    this->render_buffer_size = render_buffer_size;

    DAtom* render_buffer = new DAtom[render_buffer_size];

    RenderWindow window(VideoMode(Vector2u(800, 600)), "BiomeBloom");
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