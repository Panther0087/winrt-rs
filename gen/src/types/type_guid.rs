#[derive(Debug, Clone, Default)]
pub struct TypeGuid([GuidConstant; 11]);

#[derive(Debug, Clone)]
pub enum GuidConstant {
    U32(u32),
    U16(u16),
    U8(u8),
}

// tODO: get rid of this
impl Default for GuidConstant {
    fn default() -> Self {
        Self::U8(0)
    }
}

impl TypeGuid {
    pub fn new() -> TypeGuid {
        Default::default()
    }
}
