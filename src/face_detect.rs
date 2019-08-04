#[derive(Debug)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[derive(Debug)]
pub struct FaceInfo {
    pub face_id: String,
    pub face_orient:i32,
    pub width: i32,
    pub height: i32,
    pub position: Rect,
}

#[derive(Debug)]
pub struct FaceFeature {
    pub feature: Vec<u8>,
}

pub trait FaceDetect {
    fn detect_rgb8(&self, width:u32, height:u32, data:*const u8) -> Result<Vec<FaceInfo>, String>;
    fn extract_feature_rgb8(&self, width:u32, height:u32, data: *const u8, face:&FaceInfo ) -> Result<FaceFeature, String>;
}

pub fn feature_to_string(feature:Vec<u8>) -> String {
    let mut res = String::new();
    let src = feature.as_slice();
    for i in 0..feature.len() {
        res.push_str(format!("{:02x}", src[i]).as_str());
    }
    res
}