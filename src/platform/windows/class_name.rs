use windows::core::PCWSTR;

pub enum ClassName {
    Button,
    Edit,
    Static,
    RENDEROBJECT
}

impl ClassName {
    pub fn as_pcwstr(&self) -> PCWSTR {
        let class_name;
        match self {
            ClassName::Button => class_name = "BUTTON\0",
            ClassName::Edit =>  class_name = "EDIT\0",
            ClassName::Static => class_name = "STATIC\0",
            ClassName::RENDEROBJECT => class_name = "RENDEROBJECT\0",
        }
        PCWSTR(class_name.encode_utf16().collect::<Vec<u16>>().as_ptr()) 
    }
}