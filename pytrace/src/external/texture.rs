use libtrace::internal;
use pyo3::prelude::*;

#[pyclass]
#[text_signature = "(r, g, b, /)"]
#[derive(Copy, Clone)]
pub struct RGB {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

#[pymethods]
impl RGB {
    #[new]
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }
}

impl RGB {
    pub fn to_internal(self) -> internal::RGB {
        internal::RGB(self.r, self.g, self.b)
    }
}

macro_rules! color {
    ( $name:ident : $alias:tt, $r:expr, $g:expr, $b:expr ) => {
        #[pymethods]
        impl RGB {
            #[staticmethod]
            #[text_signature = "(/)"]
            pub fn $name() -> RGB {
                RGB {
                    r: $r,
                    g: $g,
                    b: $b,
                }
            }
        }
    };
}

color!(red: _, 1.0, 0.0, 0.0);
color!(dkred: drd, 0.5, 0.0, 0.0);
color!(ltred: lrd, 1.0, 0.5, 0.5);
color!(blue: blu, 0.0, 0.0, 1.0);
color!(dkblue: dbl, 0.0, 0.0, 0.5);
color!(ltblue: lbl, 0.3, 0.6, 1.0);
color!(cyan: cyn, 0.0, 1.0, 1.0);
color!(green: grn, 0.0, 1.0, 0.0);
color!(dkgreen: dgr, 0.0, 0.5, 0.0);
color!(ltgreen: lgr, 0.7, 1.0, 0.0);
color!(purple: ppl, 0.7, 0.0, 0.0);
color!(magenta: mgt, 1.0, 0.0, 1.0);
color!(yellow: ylw, 1.0, 1.0, 0.0);
color!(brown: brn, 0.3, 0.2, 0.0);
color!(orange: org, 1.0, 0.4, 0.0);
color!(turquoise: tqs, 0.0, 0.9, 0.6);
color!(black: blk, 0.0, 0.0, 0.0);
color!(white: wht, 1.0, 1.0, 1.0);
color!(grey: gry, 0.5, 0.5, 0.5);
color!(dkgrey: dgy, 0.2, 0.2, 0.2);
color!(ltgrey: lgy, 0.8, 0.8, 0.8);

#[pyclass]
#[derive(Clone, Copy)]
pub struct Texture {
    contents: internal::Texture,
}

#[pymethods]
impl Texture {
    #[staticmethod]
    #[text_signature = "(color, /)"]
    pub fn lambertian(color: RGB) -> Self {
        Self {
            contents: internal::Texture::Lambertian(color.to_internal()),
        }
    }

    #[staticmethod]
    #[text_signature = "(color, fuzzy, /)"]
    pub fn metal(color: RGB, fuzzy: f64) -> Self {
        Self {
            contents: internal::Texture::Metal(color.to_internal(), fuzzy),
        }
    }

    #[staticmethod]
    #[text_signature = "(color, /)"]
    pub fn light(color: RGB) -> Self {
        Self {
            contents: internal::Texture::Light(color.to_internal()),
        }
    }

    #[staticmethod]
    #[text_signature = "(color, index, /)"]
    pub fn dielectric(color: RGB, index: f64) -> Self {
        Self {
            contents: internal::Texture::Dielectric(color.to_internal(), index),
        }
    }
}

impl Texture {
    pub fn to_internal(self) -> internal::Texture {
        self.contents
    }
}
