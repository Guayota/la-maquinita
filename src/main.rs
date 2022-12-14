use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
struct CPU {
    register_a: u8,
    register_x: u8,
    register_y: u8,
    program_counter: u16,
    stack_pointer: u8,
    status_register: u8,

    /// CPU Memory Map
    /// [0x0000..0x2000]  -> CPU RAM
    /// [0x2000..0x4200]  -> IO Registers (PPU, APU, GamePads, etc)
    /// [0x4200..0x6000]  -> Expansion ROM
    /// [0x6000..0X8000]  -> Save RAM (For saving the game in Zelda and others)
    /// [0x8000..=0xFFFF] -> Program ROM (PRG ROM) on Cartridge
    #[derivative(Debug="ignore")] // Or else we'll get like 2^16-1 zeroes out.
    memory: [u8; CPU::MEMORY_MAP_SIZE]
}

impl CPU {
    const MEMORY_MAP_SIZE: usize  = 0xFFFF;
}

static cpu: CPU = CPU {
    register_a:      0u8,
    register_x:      0u8,
    register_y:      0u8,
    program_counter: 0u16,
    stack_pointer:   0u8,
    status_register: 0u8,
    memory: [0u8; CPU::MEMORY_MAP_SIZE]
};

fn main() {
    println!("{:?}", cpu);
}
