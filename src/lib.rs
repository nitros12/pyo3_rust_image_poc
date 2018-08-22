#![feature(custom_attribute)]
#![feature(specialization)]

#[macro_use]
extern crate pyo3;
extern crate image;

use image::DynamicImage;

use pyo3::prelude::*;

use pyo3::{PyResult, Python, PyModule, PyRawObject};

py_exception!(image_meme, ImageError, pyo3::exc::Exception);

#[pyclass]
struct ImageWrapper {
    inner: DynamicImage,
}

#[pymethods]
impl ImageWrapper {
    #[new]
    fn __new__(obj: &PyRawObject, path: &str) -> PyResult<()> {
        let img = image::open(path).map_err(|e| ImageError::new(e.to_string()))?;

        obj.init(|_| ImageWrapper {
            inner: img
        })
    }

    pub fn fliph(&self) -> PyResult<Py<ImageWrapper>> {
        let img = ImageWrapper {
            inner: self.inner.fliph()
        };

        let gil = Python::acquire_gil();
        let py = gil.python();

        py.init(|_| img)
    }

    fn save(&self, path: &str) -> PyResult<()> {
        self.inner.save(path).map_err(|e| ImageError::new(e.to_string()))
    }
}

#[pymodinit]
fn image_meme(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ImageWrapper>()?;

    Ok(())
}
