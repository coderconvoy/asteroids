extern crate piston_window;

use piston_window::*;

pub struct Ship {
    x:f64,y:f64,rot:f64,
    vx:f64,vy : f64,  
    tx : G2dTexture,
    acc : bool,
}

impl Ship {
    pub fn new(x :f64,y:f64,r :f64,t: G2dTexture  )->Ship {
        Ship{
            x:x,
            y:y,
            rot:r,
            vx:0.0,
            vy:0.0,
            tx:t,
            acc:false,
        }
    }

    pub fn draw(&self, c: Context, g: &mut G2d) {
        image(&self.tx,c.transform.trans(self.x,self.y).rot_rad(self.rot).trans(-20.0,-20.0),g);
    }

    pub fn input(&mut self,ip : &Input){
        if let Input::Button(b) = *ip {
            let ispress = match b.state {
                ButtonState::Press => true,
                ButtonState::Release => false,
            };

            if let Button::Keyboard(key) = b.button {
                match key {
                    Key::Up => self.acc = ispress, 
                    Key::Left => self.rot -= 0.2,
                    Key::Right => self.rot += 0.2,
                    _ => {},
                }
            }
            
        }
    }
    pub fn update(& mut self, dt: f64){
        if self.acc{
            self.vx += dt *30.0* self.rot.sin(); 
            self.vy += -dt *30.0*  self.rot.cos();
        }
        self.vx *= 0.9999; 
        self.vy *= 0.9999;
        self.x += self.vx * dt;
        self.y += self.vy * dt;
        
    }
}
