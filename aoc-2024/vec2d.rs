use std::fmt::Debug;

pub struct Vec2D<T> {
    pub data: Vec<T>,
    pub size: (usize, usize),
}

impl Clone for Vec2D<char> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            size: self.size,
        }
    }
}

impl<T> Vec2D<T> {
    pub fn new_default(size: (usize, usize), default: T) -> Self
    where
        T: Clone,
    {
        Self {
            data: vec![default; size.0 * size.1],
            size,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x >= self.size.0 || y >= self.size.1 {
            return None;
        }
        Some(&self.data[x * self.size.1 + y])
    }

    pub fn get_i32(&self, x: i32, y: i32) -> Option<&T> {
        if x < 0 || y < 0 || x as usize >= self.size.0 || y as usize >= self.size.1 {
            return None;
        }
        Some(&self.data[x as usize * self.size.1 + y as usize])
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        if x >= self.size.0 || y >= self.size.1 {
            return;
        }
        self.data[x * self.size.1 + y] = value;
    }
}

impl Debug for Vec2D<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.size.0 {
            for j in 0..self.size.1 {
                write!(f, "{}", self.data[i * self.size.1 + j])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Vec2D<char> {
    pub fn new_chars(input: &str) -> Self {
        let mut data = Vec::with_capacity(input.len());
        let mut size = (0, 0);

        size.0 = input.lines().count();
        for line in input.lines() {
            size.1 = line.len();
            for c in line.chars() {
                data.push(c);
            }
        }

        Self { data, size }
    }
}
