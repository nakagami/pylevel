/*
*MIT License
*
*Copyright (c) 2021 Hajime Nakagami
*
*Permission is hereby granted, free of charge, to any person obtaining a copy
*of this software and associated documentation files (the "Software"), to deal
*in the Software without restriction, including without limitation the rights
*to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
*copies of the Software, and to permit persons to whom the Software is
*furnished to do so, subject to the following conditions:
*
*The above copyright notice and this permission notice shall be included in all
*copies or substantial portions of the Software.
*
*THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
*IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
*FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
*AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
*LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
*OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
*SOFTWARE.
*/
use pyo3::prelude::*;
use rusty_leveldb;
use std::path::Path;

#[pyclass]
struct DB {
    database: rusty_leveldb::DB,
}

#[pymethods]
impl DB {
    #[new]
    fn new(dirname: &str, create_if_missing: Option<bool>) -> Self {
        let mut options = rusty_leveldb::Options::default();
        options.create_if_missing = match create_if_missing {
            Some(b) => b,
            None => false,
        };

        let path = Path::new(dirname);
        let database = rusty_leveldb::DB::open(path, options).unwrap();
        DB { database }
    }

    pub fn get(&mut self, key: &[u8]) -> PyResult<Option<Vec<u8>>> {
        Ok(self.database.get(key))
    }

    pub fn put(&mut self, k: &[u8], v: &[u8]) -> PyResult<()> {
        self.database.put(k, v);
        Ok(())
    }

    pub fn delete(&mut self, key: &[u8]) -> PyResult<()> {
        self.database.delete(key);
        Ok(())
    }

    pub fn flush(&mut self) -> PyResult<()> {
        self.database.flush();
        Ok(())
    }
}

#[pymodule]
fn rslevel(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<DB>()?;
    Ok(())
}
