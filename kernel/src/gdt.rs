#[allow(dead_code)]
pub struct GlobalDescriptorTable {
    // GDT의 엔트리들을 저장할 배열
    entries: [Descriptor; 7],
}

impl GlobalDescriptorTable {
    pub const fn new(entries: [Descriptor; 7]) -> Self {
        GlobalDescriptorTable { entries }
    }
}

#[repr(C, packed)]
#[allow(dead_code)]
pub struct Descriptor {
    // GDT 엔트리의 필드들을 정의
    limit_low: u16,  // 2바이트(0-15), limit의 하위 16비트
    base_low: u16,   // 2바이트(16-31), base의 하위 16비트
    base_middle: u8, // 1바이트(32-39), base의 중간 8비트(16-23)
    access: u8,      // 1바이트(40-47), access byte
    limit_flag: u8,  // 1바이트(48-55), limit의 상위 4비트와 flags
    base_high: u8,   // 1바이트(56-63), base의 상위 8비트
}

impl Descriptor {
    pub const fn new(base: u32, limit: u32, access: u8, flags: u8) -> Self {
        // limit: (20비트: 0 - 19)
        // 하위 16비트: 0xFFFF = 0b 1111 1111 1111 1111
        let limit_low: u16 = (limit & 0xFFFF) as u16;
        // 상위 4비트: 0x0F = 0b 0000 FFFF
        let limit_high: u8 = ((limit >> 16) & 0x0F) as u8;

        // base (32비트: 0 - 31)
        // 하위 16비트
        let base_low: u16 = (base & 0xFFFF) as u16;
        // 중간 8비트: 0xFF(0b1111_1111)로 하위 8비트만 남김
        let base_middle: u8 = ((base >> 16) & 0xFF) as u8;
        // 상위 8비트: 사실 24비트를 밀어버려서 마스크가 없어도 될듯 ...?
        let base_high: u8 = ((base >> 24) & 0xFF) as u8;

        // limit flag 바이트 조립
        // 상위 4비트 flag, 하위 4비트 limit
        // flag를 먼저 4비트로 밀어서 상위로 옯김
        // limit_high와 OR로 합침
        let limit_flag: u8 = (flags << 4) | limit_high;

        Descriptor {
            limit_low,
            base_low,
            base_middle,
            access,
            limit_flag,
            base_high,
        }
    }
}
