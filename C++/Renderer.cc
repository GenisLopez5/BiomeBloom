#include "Renderer.hh"


Renderer::Renderer(int render_buffer_size) : textures(1)
{
    //SETUP texture[0] DEFAULT TEXTURE
    textures[0].loadFromFile("./../Textures/Default.png");
    
    //SETUP SPRITES VECTOR (ALL START BEING DEFAULT)
    Sprite default_sprite(textures[0]);
    vector<Sprite> sprites(render_buffer_size, default_sprite);
    this->sprites = sprites;

    //SETUP RENDER BUFFER
    this->render_buffer_size = render_buffer_size;
    DAtom* render_buffer = new DAtom[render_buffer_size];

    //START WINDOW
    RenderWindow window(VideoMode(const Vector2u(800, 600)), "BiomeBloom");  
}

void Renderer::set_new_texture(const DAtom& d_atom, Sprite& sprite)
{
    switch (d_atom.material)
    {
        default:
            sprite.setTexture(textures[0]);
            break;
    }
}

void Renderer::render() {
    window.clear();

        for (int i = 0; i < render_buffer_size; ++i)
        {
            if (render_buffer[i].obsolete)
                set_new_texture(render_buffer[i], sprites[i]);

            window.draw(sprites[i]);
        }


    window.display();
}
