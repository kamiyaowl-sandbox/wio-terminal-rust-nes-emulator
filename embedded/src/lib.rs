#![crate_type="staticlib"]

extern crate rust_nes_emulator;
use rust_nes_emulator::prelude::*;

pub const EMBEDDED_EMULATOR_BINARY_SIZE_LIMIT: usize = 0x10000;
pub const EMBEDDED_EMULATOR_NUM_OF_COLOR: usize = 3;
pub const EMBEDDED_EMULATOR_VISIBLE_SCREEN_WIDTH: usize = 256;
pub const EMBEDDED_EMULATOR_VISIBLE_SCREEN_HEIGHT: usize = 240;

#[repr(u8)]
pub enum KeyEvent {
    PressA,
    PressB,
    PressSelect,
    PressStart,
    PressUp,
    PressDown,
    PressLeft,
    PressRight,
    ReleaseA,
    ReleaseB,
    ReleaseSelect,
    ReleaseStart,
    ReleaseUp,
    ReleaseDown,
    ReleaseLeft,
    ReleaseRight,
}

pub struct EmbeddedEmulator {
    pub cpu: Cpu,
    pub cpu_sys: System,
    pub ppu: Ppu,
    pub video_sys: VideoSystem,
}

impl Default for EmbeddedEmulator {
    fn default() -> Self {
        Self {
            cpu: Cpu::default(),
            cpu_sys: System::default(),
            ppu: Ppu::default(),
            video_sys: VideoSystem::default(),
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn EmbeddedEmulator_init(emu: *mut EmbeddedEmulator) {
    (*emu).cpu = Cpu::default();
    (*emu).cpu_sys = System::default();
    (*emu).ppu = Ppu::default();
    (*emu).video_sys = VideoSystem::default();
}

/// エミュレータをリセットします
/// カセットの中身はリセットしないので実機のリセット相当の処理です
#[no_mangle]
pub unsafe extern "C" fn EmbeddedEmulator_reset(emu: *mut EmbeddedEmulator) {
    (*emu).cpu.reset();
    (*emu).cpu_sys.reset();
    (*emu).ppu.reset();
    (*emu).video_sys.reset();
    (*emu).cpu.interrupt(&mut (*emu).cpu_sys, Interrupt::RESET);
}

/// .nesファイルを読み込みます
/// `data` - nesファイルのバイナリ
#[no_mangle]
pub unsafe extern "C" fn EmbeddedEmulator_load(
    emu: *mut EmbeddedEmulator,
    binary: &[u8; EMBEDDED_EMULATOR_BINARY_SIZE_LIMIT],
) -> bool {
    let success = (*emu)
        .cpu_sys
        .cassette
        .from_ines_binary(|addr: usize| binary[addr]);
    if success {
        EmbeddedEmulator_reset(emu);
    }
    success
}

/// 描画領域1面分更新します
#[no_mangle]
pub unsafe extern "C" fn EmbeddedEmulator_update_screen(
    emu: *mut EmbeddedEmulator,
    fb: &mut [[[u8; EMBEDDED_EMULATOR_NUM_OF_COLOR]; EMBEDDED_EMULATOR_VISIBLE_SCREEN_WIDTH];
             EMBEDDED_EMULATOR_VISIBLE_SCREEN_HEIGHT],
) {
    let cycle_for_draw_once = CPU_CYCLE_PER_LINE * usize::from(RENDER_SCREEN_HEIGHT + 1);
    let mut total_cycle: usize = 0;
    while total_cycle < cycle_for_draw_once {
        let cpu_cycle = usize::from((*emu).cpu.step(&mut (*emu).cpu_sys));
        (*emu).ppu.step(
            cpu_cycle,
            &mut (*emu).cpu,
            &mut (*emu).cpu_sys,
            &mut (*emu).video_sys,
            fb,
        );
        total_cycle = total_cycle + cpu_cycle;
    }
}

/// キー入力します
#[no_mangle]
pub unsafe extern "C" fn EmbeddedEmulator_update_key(emu: *mut EmbeddedEmulator, key: KeyEvent) {
    match key {
        KeyEvent::PressA => (*emu).cpu_sys.pad1.push_button(PadButton::A),
        KeyEvent::PressB => (*emu).cpu_sys.pad1.push_button(PadButton::B),
        KeyEvent::PressSelect => (*emu).cpu_sys.pad1.push_button(PadButton::Select),
        KeyEvent::PressStart => (*emu).cpu_sys.pad1.push_button(PadButton::Start),
        KeyEvent::PressUp => (*emu).cpu_sys.pad1.push_button(PadButton::Up),
        KeyEvent::PressDown => (*emu).cpu_sys.pad1.push_button(PadButton::Down),
        KeyEvent::PressLeft => (*emu).cpu_sys.pad1.push_button(PadButton::Left),
        KeyEvent::PressRight => (*emu).cpu_sys.pad1.push_button(PadButton::Right),

        KeyEvent::ReleaseA => (*emu).cpu_sys.pad1.release_button(PadButton::A),
        KeyEvent::ReleaseB => (*emu).cpu_sys.pad1.release_button(PadButton::B),
        KeyEvent::ReleaseSelect => (*emu).cpu_sys.pad1.release_button(PadButton::Select),
        KeyEvent::ReleaseStart => (*emu).cpu_sys.pad1.release_button(PadButton::Start),
        KeyEvent::ReleaseUp => (*emu).cpu_sys.pad1.release_button(PadButton::Up),
        KeyEvent::ReleaseDown => (*emu).cpu_sys.pad1.release_button(PadButton::Down),
        KeyEvent::ReleaseLeft => (*emu).cpu_sys.pad1.release_button(PadButton::Left),
        KeyEvent::ReleaseRight => (*emu).cpu_sys.pad1.release_button(PadButton::Right),
    }
}
