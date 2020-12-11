use crate::pos::Pos;
use std::{fmt, iter::FromIterator, str};

#[derive(Clone, Default, Eq, PartialEq)]
pub struct Map {
    width: i32,
    height: i32,
    data: Vec<u8>,
}

impl Map {
    #[inline]
    pub fn from_input(input: &str) -> Map {
        let mut width = 0;
        let mut data = Vec::new();

        for (y, line) in input.lines().enumerate() {
            if y == 0 {
                width = line.len() as i32;
            }

            for ch in line.bytes() {
                data.push(ch);
            }
        }

        let height = data.len() as i32 / width;

        Map {
            width,
            height,
            data,
        }
    }

    #[inline]
    pub fn get(&self, pos: Pos) -> Option<char> {
        if pos.0 < 0 || pos.0 >= self.width || pos.1 < 0 || pos.1 >= self.height {
            None
        } else {
            Some(self.data[(pos.0 + pos.1 * self.width) as usize] as char)
        }
    }

    #[inline]
    pub fn iter(&self) -> MapIter {
        MapIter {
            map: self,
            i: 0,
            end: self.width * self.height,
        }
    }

    #[inline]
    pub fn values(&self) -> MapValuesIter {
        MapValuesIter {
            map: self,
            i: 0,
            end: self.width * self.height,
        }
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in 0..self.height {
            f.write_str(
                str::from_utf8(
                    &self.data
                        [((line * self.width) as usize)..(((line + 1) * self.width) as usize)],
                )
                .unwrap(),
            )?;
            f.write_str("\n")?;
        }

        Ok(())
    }
}

impl FromIterator<(Pos, char)> for Map {
    fn from_iter<I: IntoIterator<Item = (Pos, char)>>(iter: I) -> Self {
        let mut max_x = 0;
        let mut max_y = 0;

        let mut data = Vec::new();

        for (pos, ch) in iter {
            data.push(ch as u8);
            max_x = max_x.max(pos.0);
            max_y = max_y.max(pos.1);
        }

        let width = max_x + 1;
        let height = max_y + 1;

        Map {
            width,
            height,
            data,
        }
    }
}

pub struct MapIter<'a> {
    map: &'a Map,
    i: i32,
    end: i32,
}

impl<'a> Iterator for MapIter<'a> {
    type Item = (Pos, char);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.i == self.end {
            return None;
        }

        let x = self.i % self.map.width;
        let y = self.i / self.map.width;

        let ch = self.map.data[self.i as usize] as char;

        self.i += 1;

        Some((Pos(x, y), ch))
    }
}

pub struct MapValuesIter<'a> {
    map: &'a Map,
    i: i32,
    end: i32,
}

impl<'a> Iterator for MapValuesIter<'a> {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.i == self.end {
            return None;
        }

        let ch = self.map.data[self.i as usize] as char;

        self.i += 1;

        Some(ch)
    }
}
