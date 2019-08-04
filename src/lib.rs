#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_must_use)]

#[macro_use]
extern crate log;

mod imp;
mod face_detect;

use imp::arcsoft_imp;

pub use face_detect::*;

pub struct FaceAiBuilder {
    app_id: String,
    app_secret: String,
}

impl FaceAiBuilder {
    pub fn new() -> FaceAiBuilder {
        FaceAiBuilder {
            app_id: String::new(),
            app_secret: String::new(),
        }
    }
    pub fn app_id(mut self, app_id:&str) -> Self {
        self.app_id = app_id.to_string();
        self
    }
    pub fn app_secret(mut self, app_secret:&str) -> Self {
        self.app_secret = app_secret.to_string();
        self
    }
    pub fn build(&self) -> Result<Box<face_detect::FaceDetect>,String> {
        let ai = Box::new(arcsoft_imp::ArcsoftAi::new(self.app_id.as_str(), self.app_secret.as_str())?);
        Ok(ai)
    }

}