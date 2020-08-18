use super::game2048core::Game2048Core;
use std::io::{Write, Stdout};
use crossterm::{QueueableCommand, terminal, Result, event::{read, Event, KeyCode}};
use crossterm::{event::KeyModifiers};
use std::process::exit;

/** 
 *                         2048
 *     ---------------------------------------------
 *     |          |          |          |          |
 *     |    2     |          |          |          |
 *     |          |          |          |          |
 *     ---------------------------------------------
 *     |          |          |          |          |
 *     |          |          |          |          |
 *     |          |          |          |          |
 *     ---------------------------------------------
 *     |          |          |          |          |
 *     |   2048   |          |          |          |
 *     |          |          |          |          |
 *     ---------------------------------------------
 *     |          |          |          |          |
 *     |          |          |          |          |
 *     |          |          |          |          |
 *     ---------------------------------------------
 *
 */

trait KeyModifier {
    fn has_alt(&self) -> bool;
    fn has_ctrl(&self) -> bool;
    fn has_shift(&self) -> bool;
    fn none(&self) -> bool;
}
impl KeyModifier for KeyModifiers {
    fn has_alt(&self) -> bool {
        return (*self & KeyModifiers::ALT) == KeyModifiers::ALT;
    }
    fn has_ctrl(&self) -> bool {
        return (*self & KeyModifiers::CONTROL) == KeyModifiers::CONTROL;
    }
    fn has_shift(&self) -> bool {
        return (*self & KeyModifiers::SHIFT) == KeyModifiers::SHIFT;
    }
    fn none(&self) -> bool {
        return *self == KeyModifiers::NONE;
    }
}


pub struct Game2048Terminal<'a> {
    out: &'a mut Stdout,
    game: Game2048Core
}


impl<'a> Game2048Terminal<'a> {
    fn _exit(&self, status: i32) {
        match terminal::disable_raw_mode() {
            _ => {}
        }
        println!("2048 Exit");
        exit(status);
    }

    pub fn run(&mut self) -> Result<()> {
        self.flush_data()?;

        loop {
            terminal::enable_raw_mode()?;
            match read()? {
                Event::Key(key) => {
                    let mut b: bool = false;
                    match key.code {
                        KeyCode::Left  => b = self.game.left(),
                        KeyCode::Right => b = self.game.right(),
                        KeyCode::Up    => b = self.game.up(),
                        KeyCode::Down  => b = self.game.down(),
                        KeyCode::Char(c) => {
                            if c == 'q' {
                                if key.modifiers.none()  {
                                    self._exit(0);
                                }
                            }
                            if key.modifiers.has_ctrl() {
                                self._exit(1);
                            }
                        },
                        _ => {}
                    }
                    terminal::disable_raw_mode()?;
                    if b {
                        self.flush_data()?;
                    }

                    if self.game.gameover() {
                        println!("Game Over");
                        break;
                    }
                },
                _ => {}
            }
        }

        return Ok(());
    }

    fn render_line(&mut self, d: &[i16], width: i16) -> Result<()> {
        let s = ((width - 5) / 4) as usize;
        self.out.write("|".as_bytes())?;
        if (width as usize) > (s * 4 + 5) {
            self.out.write(&vec![b' '; (width as usize)- s * 4 - 5])?;
        }

        for v in d.iter() {
            if *v == 0 {
                self.out.write(&vec![b' '; s])?;
            } else {
                let vs = v.to_string();
                let s2 = (s - vs.len()) / 2;
                self.out.write(&vec![b' ';s2])?;
                self.out.write(vs.as_bytes())?;
                self.out.write(&vec![b' ';s - vs.len() - s2])?;
            }
            self.out.write("|".as_bytes())?;
        }
        self.out.write("\n".as_bytes())?;

        return Ok(());
    }

    fn flush_data(&mut self) -> crossterm::Result<()> {
        self.out.queue(terminal::Clear(terminal::ClearType::All))?;
        let width: i16 = 45;
        self.out.write(String::from_utf8(vec![b'-'; width as usize])?.as_bytes())?;
        self.out.write("\n".as_bytes())?;

        let data: [i16;16] = *self.game.data();
        for i in 0..4 {
            self.render_line(&[0,0,0,0], width)?;
            self.render_line(&data[i*4 .. i*4+4], width)?;
            self.render_line(&[0,0,0,0], width)?;
            self.out.write(String::from_utf8(vec![b'-'; width as usize])?.as_bytes())?;
            self.out.write("\n".as_bytes())?;
        }
        self.out.flush()?;

        return Ok(());
    }

    pub fn new(out_term: &'a mut Stdout) -> Game2048Terminal {
        return Game2048Terminal {
            out: out_term,
            game: Game2048Core::new()
        }
    }
}

