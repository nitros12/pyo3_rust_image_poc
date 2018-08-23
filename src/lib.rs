#![feature(custom_attribute)]
#![feature(specialization)]

#[macro_use]
extern crate pyo3;
extern crate image;

use image::DynamicImage;

use pyo3::prelude::*;

use pyo3::{PyResult, Python, PyModule};

py_exception!(image_meme, ImageError, pyo3::exc::Exception);


#[pyclass]
struct ImageWrapper {
    inner: DynamicImage,
}


#[pymethods]
impl ImageWrapper {
    #[classmethod]
    fn from_path(cls: &PyType, path: &str) -> PyResult<Py<ImageWrapper>> {
        let py = cls.py();

        let img = image::open(path).map_err(|e| ImageError::new(e.to_string()))?;

        py.init(|_| ImageWrapper {
            inner: img
        })
    }

    #[classmethod]
    fn from_bytes(cls: &PyType, data: Vec<u8>) -> PyResult<Py<ImageWrapper>> {
        let py = cls.py();

        let img = image::load_from_memory(&data).map_err(|e| ImageError::new(e.to_string()))?;

        py.init(|_| ImageWrapper {
            inner: img
        })
    }

    // sadly we can't impl these with a macro because of the procedural pymethods macro

    fn rotate90(&self) -> PyResult<Py<ImageWrapper>> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        py.init(|_| ImageWrapper {
            inner: self.inner.rotate90()
        })
    }

    fn rotate180(&self) -> PyResult<Py<ImageWrapper>> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        py.init(|_| ImageWrapper {
            inner: self.inner.rotate180()
        })
    }

    fn rotate270(&self) -> PyResult<Py<ImageWrapper>> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        py.init(|_| ImageWrapper {
            inner: self.inner.rotate270()
        })
    }

    fn fliph(&self) -> PyResult<Py<ImageWrapper>> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        py.init(|_| ImageWrapper {
            inner: self.inner.fliph()
        })
    }

    fn flipv(&self) -> PyResult<Py<ImageWrapper>> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        py.init(|_| ImageWrapper {
            inner: self.inner.flipv()
        })
    }

    fn raw_pixels(&self) -> PyResult<Py<PyByteArray>> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let pixels = self.inner.raw_pixels();

        Ok(PyByteArray::new(py, &pixels).into())
    }

    fn grayscale(&self) -> PyResult<Py<ImageWrapper>> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        py.init(|_| ImageWrapper {
            inner: self.inner.grayscale()
        })
    }

    fn invert(&mut self) -> PyResult<()> {
        self.inner.invert();

        Ok(())
    }

    fn resize(&self, nwidth: u32, nheight: u32, filter: &str) -> PyResult<Py<ImageWrapper>> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let filter_meth = match filter {
            "Nearest"    => image::FilterType::Nearest,
            "Triangle"   => image::FilterType::Triangle,
            "CatmullRom" => image::FilterType::CatmullRom,
            "Gaussian"   => image::FilterType::Gaussian,
            "Lanczos3"   => image::FilterType::Lanczos3,
            _            => return Err(ImageError::new(format!("Unrecognised filter: {}", filter))),
        };

        py.init(|_| ImageWrapper {
            inner: self.inner.resize(nwidth, nheight, filter_meth)
        })
    }

    fn resize_exact(&self, nwidth: u32, nheight: u32, filter: &str) -> PyResult<Py<ImageWrapper>> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let filter_meth = match filter {
            "Nearest"    => image::FilterType::Nearest,
            "Triangle"   => image::FilterType::Triangle,
            "CatmullRom" => image::FilterType::CatmullRom,
            "Gaussian"   => image::FilterType::Gaussian,
            "Lanczos3"   => image::FilterType::Lanczos3,
            _            => return Err(ImageError::new(format!("Unrecognised filter: {}", filter))),
        };

        py.init(|_| ImageWrapper {
            inner: self.inner.resize_exact(nwidth, nheight, filter_meth)
        })
    }

    fn resize_to_fill(&self, nwidth: u32, nheight: u32, filter: &str) -> PyResult<Py<ImageWrapper>> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let filter_meth = match filter {
            "Nearest"    => image::FilterType::Nearest,
            "Triangle"   => image::FilterType::Triangle,
            "CatmullRom" => image::FilterType::CatmullRom,
            "Gaussian"   => image::FilterType::Gaussian,
            "Lanczos3"   => image::FilterType::Lanczos3,
            _            => return Err(ImageError::new(format!("Unrecognised filter: {}", filter))),
        };

        py.init(|_| ImageWrapper {
            inner: self.inner.resize_to_fill(nwidth, nheight, filter_meth)
        })
    }

    fn thumbnail(&self, nwidth: u32, nheight: u32) -> PyResult<Py<ImageWrapper>> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        py.init(|_| ImageWrapper {
            inner: self.inner.thumbnail(nwidth, nheight)
        })
    }

    fn thumbnail_exact(&self, nwidth: u32, nheight: u32) -> PyResult<Py<ImageWrapper>> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        py.init(|_| ImageWrapper {
            inner: self.inner.thumbnail_exact(nwidth, nheight)
        })
    }

    fn blur(&self, sigma: f32) -> PyResult<Py<ImageWrapper>> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        py.init(|_| ImageWrapper {
            inner: self.inner.blur(sigma)
        })
    }

    fn unsharpen(&self, sigma: f32, threshold: i32) -> PyResult<Py<ImageWrapper>> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        py.init(|_| ImageWrapper {
            inner: self.inner.unsharpen(sigma, threshold)
        })
    }

    fn filter3x3(&self, kernel: Vec<f32>) -> PyResult<Py<ImageWrapper>> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        if kernel.len() != 9 {
            return Err(ImageError::new("kernel must be 3x3"));
        }

        py.init(|_| ImageWrapper {
            inner: self.inner.filter3x3(&kernel)
        })
    }

    fn adjust_contrast(&self, c: f32) -> PyResult<Py<ImageWrapper>> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        py.init(|_| ImageWrapper {
            inner: self.inner.adjust_contrast(c)
        })
    }

    fn brighten(&self, value: i32) -> PyResult<Py<ImageWrapper>> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        py.init(|_| ImageWrapper {
            inner: self.inner.brighten(value)
        })
    }

    fn huerotate(&self, value: i32) -> PyResult<Py<ImageWrapper>> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        py.init(|_| ImageWrapper {
            inner: self.inner.huerotate(value)
        })
    }

    fn save(&self, path: &str) -> PyResult<()> {
        self.inner.save(path).map_err(|e| ImageError::new(e.to_string()))
    }

    fn as_bytes(&self, format: PyObject) -> PyResult<Py<PyByteArray>> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let format = if let Ok(format) = format.extract::<&str>(py) {
            match format {
                "PNG" => image::ImageOutputFormat::PNG,
                "GIF" => image::ImageOutputFormat::GIF,
                "ICO" => image::ImageOutputFormat::ICO,
                "BMP" => image::ImageOutputFormat::BMP,
                _     => return Err(ImageError::new(format!("invalid image format: {:?}", format))),
            }
        } else if let Ok(("JPEG", size)) = format.extract::<(&str, u8)>(py) {
            image::ImageOutputFormat::JPEG(size)
        } else {
            return Err(ImageError::new(format!("invalid image format: {:?}", format)));
        };

        let mut buf = Vec::new();

        self.inner.write_to(&mut buf, format).map_err(|e| ImageError::new(e.to_string()))?;

        Ok(PyByteArray::new(py, &buf).into())
    }
}

#[pymodinit]
fn image_meme(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ImageWrapper>()?;

    Ok(())
}
