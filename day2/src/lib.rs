use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Debug, Clone)]
#[display("{l}x{w}x{h}")]
pub struct Present { pub l: u64, pub w: u64, pub h: u64 }

pub fn present_to_areas(p: &Present) -> Vec<u64> {
  vec![p.l * p.w, p.l * p.w, p.w * p.h, p.w * p.h, p.l * p.h, p.l * p.h]
}

pub fn present_to_perimeters(p: &Present) -> Vec<u64> {
  vec![(p.l + p.w) * 2, (p.w + p.h) * 2, (p.l + p.h) * 2]
}

pub fn present_to_volume(p: &Present) -> u64 {
  p.l * p.w * p.h
}