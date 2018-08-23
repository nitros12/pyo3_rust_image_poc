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
    /// from_path(cls, path: str, /) -> ImageWrapper
    /// --
    ///
    /// Construct an instance of `ImageWrapper` from a path.
    fn from_path(cls: &PyType, path: &str) -> PyResult<Py<ImageWrapper>> {
        let py = cls.py();

        let img = image::open(path).map_err(|e| ImageError::new(e.to_string()))?;

        py.init(|_| ImageWrapper {
            inner: img
        })
    }

    #[classmethod]
    /// from_bytes(cls, data: ByteString, /) -> ImageWrapper
    /// --
    ///
    /// Construct an instance of `ImageWrapper` from a bytes like object.
    fn from_bytes(cls: &PyType, data: Vec<u8>) -> PyResult<Py<ImageWrapper>> {
        let py = cls.py();

        let img = image::load_from_memory(&data).map_err(|e| ImageError::new(e.to_string()))?;

        py.init(|_| ImageWrapper {
            inner: img
        })
    }

    // sadly we can't impl these with a macro because of the procedural pymethods macro

    /// rotate90(self, /) -> ImageWrapper
    /// --
    ///
    /// Rotate this image 90 degrees clockwise.
    fn rotate90(&self) -> PyResult<Py<ImageWrapper>> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        py.init(|_| ImageWrapper {
            inner: self.inner.rotate90()
        })
    }

    /// rotate180(self, /) -> ImageWrapper
    /// --
    ///
    /// Rotate this image 180 degrees clockwise.
    fn rotate180(&self) -> PyResult<Py<ImageWrapper>> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        py.init(|_| ImageWrapper {
            inner: self.inner.rotate180()
        })
    }

    /// rotate270(self, /) -> ImageWrapper
    /// --
    ///
    /// Rotate this image 270 degrees clockwise.
    fn rotate270(&self) -> PyResult<Py<ImageWrapper>> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        py.init(|_| ImageWrapper {
            inner: self.inner.rotate270()
        })
    }

    /// fliph(self, /) -> ImageWrapper
    /// --
    ///
    /// Rotate this image 90 degrees clockwise.
    fn fliph(&self) -> PyResult<Py<ImageWrapper>> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        py.init(|_| ImageWrapper {
            inner: self.inner.fliph()
        })
    }

    /// flipv(self, /) -> ImageWrapper
    /// --
    ///
    /// Flip this image vertically
    fn flipv(&self) -> PyResult<Py<ImageWrapper>> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        py.init(|_| ImageWrapper {
            inner: self.inner.flipv()
        })
    }

    /// raw_pixels(self, /) -> bytearray
    /// --
    ///
    /// Return this image's pixels as a byte array
    fn raw_pixels(&self) -> PyResult<Py<PyByteArray>> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let pixels = self.inner.raw_pixels();

        Ok(PyByteArray::new(py, &pixels).into())
    }

    /// grayscale(self, /) -> ImageWrapper
    /// --
    ///
    /// Return a grayscale version of this image.
    fn grayscale(&self) -> PyResult<Py<ImageWrapper>> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        py.init(|_| ImageWrapper {
            inner: self.inner.grayscale()
        })
    }

    /// invert(self, /)
    /// --
    ///
    /// Invert the colors of this image, This method operates inplace (for some reason)
    fn invert(&mut self) -> PyResult<()> {
        self.inner.invert();

        Ok(())
    }

    /// resize(self, nwidth: int, nheight: int, filter: str, /) -> ImageWrapper
    /// --
    ///
    /// Resize this image using the specified filter algorithm.
    /// Returns a new image.
    /// The image's aspect ratio is preserved.
    /// The image is scaled to the maximum possible size that fits within the bounds specified by `nwidth` and `nheight`.
    ///
    /// The parameter `filter` should be a string of any of \"Nearest\", \"Triangle\", \"CatmullRom\", \"Gaussian\", or \"Lanczos3\".
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

    /// resize_exact(self, nwidth: int, nheight: int, filter: str, /) -> ImageWrapper
    /// --
    /// Resize this image using the specified filter algorithm.
    /// Returns a new image.
    /// Does not preserve aspect ratio.
    /// `nwidth` and `nheight` are the new image's dimensions
    ///
    /// The parameter `filter` should be a string of any of \"Nearest\", \"Triangle\", \"CatmullRom\", \"Gaussian\", or \"Lanczos3\".
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

    /// resize(self, nwidth: int, nheight: int, filter: str, /) -> ImageWrapper
    /// --
    ///
    /// Resize this image using the specified filter algorithm.
    /// Returns a new image.
    /// The image's aspect ratio is preserved.
    /// The image is scaled to the maximum possible size that fits within the larger (relative to aspect ratio) of the bounds specified by `nwidth` and `nheight`, then cropped to fit within the other bound.
    ///
    /// The parameter `filter` should be a string of any of \"Nearest\", \"Triangle\", \"CatmullRom\", \"Gaussian\", or \"Lanczos3\".
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

    /// thumbnail(self, nwidth: int, nheight: int, /) -> ImageWrapper
    /// --
    ///
    /// Scale this image down to fit within a specific size.
    /// Returns a new image.
    /// The image's aspect ratio is preserved.
    /// The image is scaled to the maximum possible size that fits within the bounds specified by `nwidth` and `nheight`.
    ///
    /// This method uses a fast integer algorithm where each source pixel contributes to exactly one target pixel.
    /// May give aliasing artifacts if new size is close to old size.
    fn thumbnail(&self, nwidth: u32, nheight: u32) -> PyResult<Py<ImageWrapper>> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        py.init(|_| ImageWrapper {
            inner: self.inner.thumbnail(nwidth, nheight)
        })
    }

    /// thumbnail_exact(self, nwidth: int, nheight: int, /) -> ImageWrapper
    /// --
    ///
    /// Scale this image down to a specific size.
    /// Returns a new image.
    /// Does not preserve aspect ratio.
    /// `nwidth` and `nheight` are the new image's dimensions.
    /// This method uses a fast integer algorithm where each source pixel contributes to exactly one target pixel.
    /// May give aliasing artifacts if new size is close to old size.
    fn thumbnail_exact(&self, nwidth: u32, nheight: u32) -> PyResult<Py<ImageWrapper>> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        py.init(|_| ImageWrapper {
            inner: self.inner.thumbnail_exact(nwidth, nheight)
        })
    }

    /// blur(self, sigma: float, /) -> ImageWrapper
    /// --
    ///
    /// Performs a Gaussian blur on this image.
    /// `sigma` is a measure of how much to blur by.
    fn blur(&self, sigma: f32) -> PyResult<Py<ImageWrapper>> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        py.init(|_| ImageWrapper {
            inner: self.inner.blur(sigma)
        })
    }

    /// unsharpen(self, sigma: float, threshold: int, /) -> ImageWrapper
    /// --
    ///
    /// Performs an unsharpen mask on this image.
    /// `sigma` is the amount to blur the image by.
    /// `threshold` is a control of how much to sharpen.
    fn unsharpen(&self, sigma: f32, threshold: i32) -> PyResult<Py<ImageWrapper>> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        py.init(|_| ImageWrapper {
            inner: self.inner.unsharpen(sigma, threshold)
        })
    }

    /// filter3x3(self, kernel: List[float], /) -> ImageWrapper
    /// --
    ///
    /// Filters this image with the specified 3x3 kernel.
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

    /// adjust_contrast(self, c: float, /) -> ImageWrapper
    /// --
    ///
    /// Adjust the contrast of this image.
    /// `contrast` is the amount to adjust the contrast by.
    /// Negative values decrease the contrast and positive values increase the contrast.
    fn adjust_contrast(&self, c: f32) -> PyResult<Py<ImageWrapper>> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        py.init(|_| ImageWrapper {
            inner: self.inner.adjust_contrast(c)
        })
    }

    /// brighten(self, value: int, /) -> ImageWrapper
    /// --
    ///
    /// Brighten the pixels of this image.
    /// `value` is the amount to brighten each pixel by.
    /// Negative values decrease the brightness and positive values increase it.
    fn brighten(&self, value: i32) -> PyResult<Py<ImageWrapper>> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        py.init(|_| ImageWrapper {
            inner: self.inner.brighten(value)
        })
    }

    /// huerotate(self, value: int, /) -> ImageWrapper
    /// --
    ///
    /// Hue rotate the supplied image.
    /// `value` is the degrees to rotate each pixel by.
    /// 0 and 360 do nothing, the rest rotates by the given degree value.
    /// just like the css webkit filter hue-rotate(180)
    fn huerotate(&self, value: i32) -> PyResult<Py<ImageWrapper>> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        py.init(|_| ImageWrapper {
            inner: self.inner.huerotate(value)
        })
    }

    /// save(self, path: str)
    /// --
    ///
    /// Save the buffer to a file as the path specified.
    /// The image format is derived from the file extension.
    fn save(&self, path: &str) -> PyResult<()> {
        self.inner.save(path).map_err(|e| ImageError::new(e.to_string()))
    }

    /// as_bytes(self, format: Union[str, Tuple[str, int]]) -> bytearray
    /// --
    ///
    /// Save the image to a bytearray object.
    /// `format` should be either one of the strings \"PNG\", \"GIF\", \"ICO\", \"BMP\",
    /// or a tuple `(\"JPEG\", quality)` where `quality` is the quality of the image.
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
