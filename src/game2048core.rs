/* 2048 Game Core */
use rand::random;


#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Game2048Core {
    m_data: [i16; 16]
}


impl Game2048Core {
    fn next_value(&mut self) {
        let mut slots: Vec<i8> = Vec::new();
        for i in 0..16 {
            if self.m_data[i] == 0 {
                slots.push(i as i8);
            }
        }
        let mut l: u8 = random();
        l %= slots.len() as u8;

        let mut i = 0;
        match slots.get(l as usize) {
            None => assert_eq!(false, true),
            Some(v) => {
                i = *v;
            }
        }

        self.m_data[i as usize] = 2;
    }

    pub fn up(&mut self) -> bool {
        let mut updated: bool = false;
        for i in 0..4 {
            for j in 1..4 {
                let _up:   i16 = self.m_data[(j - 1) * 4 + i];
                let _down: i16 = self.m_data[j * 4 + i];
                if _down == 0 {
                    continue;
                } else {
                    if _up == 0 {
                        self.m_data[(j - 1) * 4 + i] = _down;
                        self.m_data[j * 4 + i] = 0;
                        updated = true;
                    } else if _up == _down {
                        self.m_data[(j - 1) * 4 + i] = _down * 2;
                        self.m_data[j * 4 + i] = 0;
                        updated = true;
                    }
                }
            }
        }
        if updated {
            self.next_value();
        }
        return updated;
    }
    pub fn down(&mut self) -> bool {
        let mut updated: bool = false;
        for i in 0..4 {
            for j in (1..4).rev() {
                let _up: i16   = self.m_data[(j - 1) * 4 + i];
                let _down: i16 = self.m_data[j * 4 + i];
                if _up == 0 {
                    continue;
                } else {
                    if _down == 0 {
                        self.m_data[j * 4 + i] = _up;
                        self.m_data[(j - 1) * 4 + i] = 0;
                        updated = true;
                    } else if _down == _up {
                        self.m_data[j * 4 + i] = _down * 2;
                        self.m_data[(j - 1) * 4 + i] = 0;
                        updated = true;
                    }
                }
            }
        }
        if updated {
            self.next_value();
        }
        return updated;
    }
    pub fn left(&mut self) -> bool {
        let mut updated: bool = false;
        for i in 0..4 {
            for j in 1..4 {
                let _left: i16 = self.m_data[i * 4 + j - 1];
                let _right: i16 = self.m_data[i * 4 + j];
                if _right == 0 {
                    continue;
                } else {
                    if _left == 0 {
                        self.m_data[i*4+j-1] = _right;
                        self.m_data[i*4+j] = 0;
                        updated = true;
                    } else if _left == _right {
                        self.m_data[i*4+j-1] = _right * 2;
                        self.m_data[i*4+j] = 0;
                        updated = true;
                    }
                }
            }
        }
        if updated {
            self.next_value();
        }
        return updated;
    }
    pub fn right(&mut self) -> bool {
        let mut updated: bool = false;
        for i in 0..4 {
            for j in (1..4).rev() {
                let _left: i16 = self.m_data[i * 4 + j - 1];
                let _right: i16 = self.m_data[i * 4 + j];
                if _left == 0 {
                    continue;
                } else {
                    if _right == 0 {
                        self.m_data[i*4+j] = _left;
                        self.m_data[i*4+j-1] = 0;
                        updated = true;
                    } else if _left == _right {
                        self.m_data[i*4+j] = _left * 2;
                        self.m_data[i*4+j-1] = 0;
                        updated = true;
                    }
                }
            }
        }
        if updated {
            self.next_value();
        }
        return updated;
    }

    pub fn gameover(mut self) -> bool {
        return !self.up() && !self.down() && !self.left() && !self.right();
    }

    pub fn data(&self) -> &[i16; 16] {
        return &self.m_data;
    }

    pub fn new() -> Game2048Core {
        return Game2048Core{m_data: [2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]};
    }
}

