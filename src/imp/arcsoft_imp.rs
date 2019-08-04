#![allow(unused_imports)]

use super::arcsoft_sys::*;
use crate::face_detect::*;

use std::ffi::CString;
use std::ptr;
use std::mem;
use std::slice;

const NSCALE:MInt32 = 30;
const FACENUM:MInt32 = 64;
const FORMAT_RGB8:MInt32 = ASVL_PAF_RGB24_B8G8R8 as MInt32;	//图像数据为RGB24颜色格式

pub struct ArcsoftAi {
    engine_handle: MHandle,
}

impl ArcsoftAi {
    pub fn new(app_id:&str, app_sdk_key:&str) -> Result<Self, String> {
        let appid = CString::new(app_id).unwrap();
        let appsdk = CString::new(app_sdk_key).unwrap();
        let mut res:MRESULT  = unsafe {
            ASFOnlineActivation(appid.as_ptr() as MPChar, appsdk.as_ptr() as MPChar)
        };
        if MOK != res as u32 && MERR_ASF_ALREADY_ACTIVATED != res as u32  {
            error!("ASFOnlineActivation fail: {}", res);
            return Err(format!("online activation {}", res));
        }
        let mut handle:MHandle = 0 as MHandle;
        let mask:MInt32 = (ASF_FACE_DETECT | ASF_FACERECOGNITION | ASF_AGE | ASF_GENDER | ASF_FACE3DANGLE | ASF_LIVENESS | ASF_IR_LIVENESS) as MInt32;
        res = unsafe {
            ASFInitEngine(ASF_DETECT_MODE_IMAGE, ArcSoftFace_OrientPriority_ASF_OP_0_ONLY as MInt32, NSCALE, FACENUM, mask, (&mut handle) as *mut MHandle)
        };
        if res as u32 != MOK  {
            error!("ALInitEngine fail: {}", res);
            return Err(format!("init failed: {}", res));
        }


        Ok(ArcsoftAi{
            engine_handle: handle,
        })

    }
}

impl FaceDetect for ArcsoftAi {
    fn detect_rgb8(&self, width:u32, height:u32, data:*const u8) -> Result<Vec<FaceInfo>, String> {
        let mut face_infos:Vec<FaceInfo> = Vec::new();
        let mut detectedFaces1 = ASF_MultiFaceInfo{ 
            faceRect: ptr::null_mut(),
            faceOrient: ptr::null_mut(), 
            faceNum: 0, 
            faceID: ptr::null_mut()
        };
        // let mut copyfeature1 = ASF_FaceFeature {
        //     feature: ptr::null_mut(),
        //     featureSize: 0
        // };
        let res:MRESULT = unsafe {
            ASFDetectFaces(self.engine_handle, width as MInt32, height as MInt32, FORMAT_RGB8, data as *mut MUInt8, &mut detectedFaces1 as *mut ASF_MultiFaceInfo)
        };
        if res as u32 != MOK {
            error!("ASFDetectFaces fail: {}", res);
            if res as u32 == MERR_ASF_IMAGE_WIDTH_HEIGHT_NOT_SUPPORT {
                error!("IMAGE_WIDTH_HEIGHT_NOT_SUPPORT. condition: width %4 == 0 && height %4 == 0");
            }
            return Err(format!("detect failed {}",res));
        } else {
            info!("ASFDetectFaces sucess: {} faceNum {}", res, detectedFaces1.faceNum);
            let faces  = if detectedFaces1.faceID.is_null() {
                    &[]
                } else {
                    unsafe {
                        slice::from_raw_parts(detectedFaces1.faceID, detectedFaces1.faceNum as usize)
                    }
                };
            let orients = if detectedFaces1.faceOrient.is_null() {
                &[]
            } else {
                unsafe{
                    slice::from_raw_parts(detectedFaces1.faceOrient, detectedFaces1.faceNum as usize)
                }
            };
            for i in 0..(detectedFaces1.faceNum as usize) {
                unsafe {
                    // let faceId:MInt32 = *detectedFaces1.faceID.offset(i as isize);
                    if faces.len()>= i+1 {
                        info!("{} faceId {}", i, faces[i]);
                    }
                    let rect = &*detectedFaces1.faceRect.offset(i as isize);
                    info!("left {} top {} right {} bottom {}", rect.left, rect.top, rect.right, rect.bottom);
                    face_infos.push(FaceInfo{
                        face_id: if faces.len()>=i+1 { format!("{}", faces[i])} else { String::new() },
                        face_orient: if orients.len()>=i+1 { orients[i] } else { -1 },
                        width : (rect.right - rect.left) as i32,
                        height: (rect.bottom - rect.top) as i32,
                        position: Rect {
                            top: rect.top as i32,
                            left: rect.left as i32,
                            right: rect.right as i32,
                            bottom: rect.bottom as i32,
                        },
                    });
                }
            }
        }
        Ok(face_infos)
    }
    fn extract_feature_rgb8(&self, width:u32, height:u32, data: *const u8, face:&FaceInfo ) -> Result<FaceFeature, String> {
        // ASF_SingleFaceInfo SingleDetectedFaces = { 0 };
        // ASF_FaceFeature feature1 = { 0 };
        let mut face_info = ASF_SingleFaceInfo {
            faceRect: MRECT {
                left: face.position.left as MInt32,
                top: face.position.top as MInt32,
                right: face.position.right as MInt32,
                bottom: face.position.bottom as MInt32,
            },
            faceOrient: face.face_orient as MInt32,
        };
        let mut feature = ASF_FaceFeature {
            feature: ptr::null_mut(),
            featureSize: 0,
        };
		let res:MRESULT= unsafe {
            ASFFaceFeatureExtract(self.engine_handle, width as MInt32, height as MInt32, FORMAT_RGB8, data as *mut MUInt8, 
                        &mut face_info as LPASF_SingleFaceInfo, &mut feature as LPASF_FaceFeature) };
		if res as u32 != MOK {
            return Err(format!("failed: {}",res));
        }
        if feature.feature.is_null() {
            return Err("no feature info".to_string());
        }
        let mut ff:Vec<u8> = Vec::new();
        let tmp = unsafe{slice::from_raw_parts(feature.feature, feature.featureSize as usize)};
        ff.extend_from_slice(tmp);
        Ok(FaceFeature {
            feature: ff,
        })
    }
}

impl Drop for ArcsoftAi {
    fn drop(&mut self) {
        if !self.engine_handle.is_null() {
            let res = unsafe { ASFUninitEngine(self.engine_handle) };
            info!("drop ArcsoftAi engine_handle return {}", res);
        }
    }
}