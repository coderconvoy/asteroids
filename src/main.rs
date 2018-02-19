extern crate piston_window;
extern crate find_folder;

use piston_window::*;

mod ship;


fn main() {
    println!("Hello, world!");
    let mut window :PistonWindow = 
        WindowSettings::new("Asteroids",[512,400]).exit_on_esc(true)
        .build().unwrap();

    let assetsf = find_folder::Search::ParentsThenKids(3,3).for_folder("assets").unwrap();
    let shipf = assetsf.join("ship.png");
    let shipt: G2dTexture = Texture::from_path(
        &mut window.factory,
        &shipf ,
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    let mut ship = ship::Ship::new(100.0,100.0,0.0,shipt);

    
    //event loop
    while let Some(e) = window.next(){
        match e {
            Event::Loop(l) => {
                if let Loop::Update(u) = l {
                    ship.update(u.dt);
                }
            },
            Event::Input(i) => {
                ship.input(&i);
                continue
            },
            _ => {}
        }
        window.draw_2d(&e,|c,g|{
            clear([0.5,0.5,0.5,1.0],g);
            rectangle([1.0,0.0,0.0,1.0],[0.0,0.0,100.0,100.0],c.transform,g);
            ship.draw(c,g);
        });
    }
}
