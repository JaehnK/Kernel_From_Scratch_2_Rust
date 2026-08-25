#[repr(C)]
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

#[allow(dead_code)]
mod access {
    const PRESENT: u8 = 1 << 7; // 1: 세그먼트 존재, 0: 세그먼트 미존재 0x80

    const DPL_RING0: u8 = 0 << 5; // 0: 커널 모드 0x00
    const DPL_RING1: u8 = 1 << 5; // 1: 사용자 모드 0x20
    const DPL_RING2: u8 = 2 << 5; // 2: 사용자 모드 0x40
    const DPL_RING3: u8 = 3 << 5; // 3: 사용자 모드 0x60

    const CODE_DATA: u8 = 1 << 4; // 0: 시스템 세그먼트, 1: 코드/데이터 세그먼트 0x10
    const EXEC: u8 = 1 << 3; // 1: 실행 가능, 0: 실행 불가능 0x08
    const DC: u8 = 1 << 2; // 데이터(E=0) → Direction: 0=expand-up, 1=expand-down
                           //코드(E=1) → Conforming: 0=이 DPL에서만, 1=같거나 낮은 특권도 실행 가능
    const RW: u8 = 1 << 1; // 데이터 세그먼트 - 1: 읽기/쓰기 가능, 0: 읽기 전용
                           // 코드 세그먼트 - 1: 읽기 가능, 0: 읽기 불가능 0x02
    const ACCESSED: u8 = 1 << 0; // 1: 접근됨, 0: 접근 안됨 0x01 -> OS 단에서 초기화 시 항상 0으로 설정
}

#[allow(dead_code)]
mod flags {
    const GRANULARITY: u8 = 1 << 3; // 1: 4KB 단위, 0: 바이트 단위 0x08
    const SIZE: u8 = 1 << 2; // 1: 32비트, 0: 16비트 0x04
    const LONG_MODE: u8 = 1 << 1; // 1: 64비트, 0: 32비트/16비트 0x02
    const AVL: u8 = 1 << 0; // x86 아키텍처에서 CPU가 사용되지 비트, OS가 필요 시 자유롭게 사용

    pub const FLAT_42BIT: u8 = GRANULARITY | SIZE; // 0x0C
}

const _: () = {
    assert!(core::mem::size_of::<Descriptor>() == 8);
};

pub fn init_gdt() {}
