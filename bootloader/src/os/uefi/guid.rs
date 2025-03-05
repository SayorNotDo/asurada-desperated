#[macro_export]
macro_rules! guid {
    ($guid_str:literal) => {
        $crate::os::uefi::guid::Guid::parse_str($guid_str)
    };
}
const HYPHENATED_LEN: usize = 36;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Guid(pub u32, pub u16, pub u16, pub [u8; 8]);

impl Guid {
    /// 字符串序列转化为GUID
    pub const fn parse_str(literal: &str) -> Self {
        if literal.len() != HYPHENATED_LEN {
            panic!("invalid GUID length");
        }

        let bytes = literal.as_bytes();

        // 校验连接号
        if bytes[8] != b'-' || bytes[13] != b'-' || bytes[18] != b'-' || bytes[23] != b'-' {
            panic!("invalid GUID format");
        }

        let mut raw = [0u8; 16];
        let mut i = 0;
        let mut j = 0;

        while i < HYPHENATED_LEN {
            if i == 8 || i == 13 || i == 18 || i == 23 {
                i += 1;
            }

            let hi = hex_to_u8(bytes[i]);
            let lo = hex_to_u8(bytes[i + 1]);

            let b = hi << 4 | lo;
            raw[j] = b;

            i += 1;
            j += 1;
        }

        let d1 = u32::from_be_bytes([raw[0], raw[1], raw[2], raw[3]]);
        let d2 = u16::from_be_bytes([raw[4], raw[5]]);
        let d3 = u16::from_be_bytes([raw[6], raw[7]]);
        let d4 = [
            raw[8], raw[9], raw[10], raw[11], raw[12], raw[13], raw[14], raw[15],
        ];
        Self(d1, d2, d3, d4)
    }
}

/// 十六进制字符转化为对应的整型值
const fn hex_to_u8(hex: u8) -> u8 {
    match hex {
        b'0'..=b'9' => hex - b'0',
        b'A'..=b'F' => hex - b'A' + 10,
        b'a'..=b'f' => hex - b'a' + 10,
        _ => panic!("invalid hex value in GUID"),
    }
}
