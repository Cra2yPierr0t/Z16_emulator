enum Opcode {
    Add,
    Sub,
    Mul,
    Div,
    Or,
    And,
    Xor,
    SLL,
    SRL,
    Addi,
    Subi,
    Load,
    Store,
    JALR,
    BEQ,
    BLT,
}

enum InstrType {
    RType,
    IType,
    LType,
    SType,
    JType,
    BType,
}

fn get_instr_type(instr: u16) -> InstrType {
    match instr & 0x000F {
        0x0..=0x8   => InstrType::RType,
        0x9..=0xA   => InstrType::IType,
        0xB         => InstrType::LType,
        0xC         => InstrType::SType,
        0xD         => InstrType::JType,
        0xE..=0xF   => InstrType::BType,
        _           => InstrType::RType,
    }
}

fn get_instr_opcode(instr: u16) -> Opcode {
    match instr & 0x000F {
        0x0 => Opcode::Add,
        0x1 => Opcode::Sub,
        0x2 => Opcode::Mul,
        0x3 => Opcode::Div,
        0x4 => Opcode::Or,
        0x5 => Opcode::And,
        0x6 => Opcode::Xor,
        0x7 => Opcode::SLL,
        0x8 => Opcode::SRL,
        0x9 => Opcode::Addi,
        0xA => Opcode::Subi,
        0xB => Opcode::Load,
        0xC => Opcode::Store,
        0xD => Opcode::JALR,
        0xE => Opcode::BEQ,
        0xF => Opcode::BLT,
        _   => Opcode::Add,
    }
}

fn get_rd_addr(instr: u16) -> u16 {
    match get_instr_type(instr) {
        InstrType::RType    => (instr >> 4) & 0x000F,
        InstrType::IType    => (instr >> 4) & 0x000F,
        InstrType::LType    => (instr >> 4) & 0x000F,
        InstrType::JType    => (instr >> 4) & 0x000F,
        _                   => 0,
    }
}

fn get_rs1_addr(instr: u16) -> u16 {
    match get_instr_type(instr) {
        InstrType::RType    => (instr >> 8) & 0x000F,
        InstrType::LType    => (instr >> 8) & 0x000F,
        InstrType::SType    => (instr >> 8) & 0x000F,
        InstrType::JType    => (instr >> 8) & 0x000F,
        InstrType::BType    => (instr >> 4) & 0x0003,
        _                   => 0,
    }
}

fn get_rs2_addr(instr: u16) -> u16 {
    match get_instr_type(instr) {
        InstrType::RType    => (instr >> 12) & 0x000F,
        InstrType::SType    => (instr >> 12) & 0x000F,
        InstrType::BType    => (instr >> 6) & 0x0003,
        _                   => 0,
    }
}

fn get_imm(instr: u16) -> i16 {
    match get_instr_type(instr) {
        InstrType::IType    => (instr as i16) >> 8,
        InstrType::LType    => instr as i16 >> 12,
        InstrType::SType    => (instr << 8) as i16 >> 12,
        InstrType::JType    => instr as i16 >> 12,
        InstrType::BType    => instr as i16 >> 8,
        _                   => 0,
    }
} 

fn alu(opcode: Opcode, data2: i16, data1: i16) -> i16 {
    match opcode {
        Opcode::Add     => data2 + data1,
        Opcode::Sub     => data2 - data1,
        Opcode::Mul     => data2 * data1,
        Opcode::Div     => data2 / data1,
        Opcode::Or      => data2 | data1,
        Opcode::And     => data2 & data1,
        Opcode::Xor     => data2 ^ data1,
        Opcode::SLL     => data1 << data2,
        Opcode::SRL     => (data1 as u16 >> data2 as u16) as i16,
        Opcode::Addi    => data2 + data1,
        Opcode::Subi    => data2 - data1,
        _               => data2 + data1
    }
}

fn main() {
    let InstrMemory: [u16; 5] = [
        0x0A19,
        0x1220,
        0x011A,
        0xFC4F,
        0x00FD
    ];


    // Degital Block
    let mut RegisterFile: [i16; 16] = [0; 16];
    let mut DataMemory: [i16; 32768] = [0; 32768];
    let mut pc: u16 = 0;

    // wires
    let mut instr: u16 = InstrMemory[pc as usize];
    let mut rs1_data: i16 = 0;
    let mut rs2_data: i16 = 0;
    let mut rd_data: i16  = 0;
    let mut imm: i16      = 0;

    for count in 0..50 {
        instr = InstrMemory[(pc >> 1) as usize];
        match get_instr_type(instr) {
            InstrType::RType => {
                rs1_data = RegisterFile[get_rs1_addr(instr) as usize];
                rs2_data = RegisterFile[get_rs2_addr(instr) as usize];
                RegisterFile[get_rd_addr(instr) as usize] = alu(get_instr_opcode(instr), rs2_data, rs1_data);
                pc += 2;
            },
            InstrType::IType => {
                rd_data = RegisterFile[get_rd_addr(instr) as usize];
                imm     = get_imm(instr);
                RegisterFile[get_rd_addr(instr) as usize] = alu(get_instr_opcode(instr), rd_data, imm);
                pc += 2;
            },
            InstrType::LType => {
                rs1_data = RegisterFile[get_rs1_addr(instr) as usize];
                imm     = get_imm(instr);
                RegisterFile[get_rd_addr(instr) as usize] = DataMemory[alu(Opcode::Add, rs1_data, imm) as usize];
                pc += 2;
            },
            InstrType::SType => {
                rs1_data    = RegisterFile[get_rs1_addr(instr) as usize];
                imm         = get_imm(instr);
                DataMemory[alu(Opcode::Add, rs1_data, imm) as usize] = RegisterFile[get_rs2_addr(instr) as usize];
                pc += 2;
            },
            InstrType::JType => {
                RegisterFile[get_rd_addr(instr) as usize] = (pc + 2) as i16;
                rs1_data    = RegisterFile[get_rs1_addr(instr) as usize];
                pc = (pc as i16 + get_imm(instr) + rs1_data as i16) as u16;
            },
            InstrType::BType => {
                rs1_data = RegisterFile[get_rs1_addr(instr) as usize];
                rs2_data = RegisterFile[get_rs2_addr(instr) as usize];
                match get_instr_opcode(instr) {
                    Opcode::BEQ => {
                        if rs2_data == rs1_data {
                            pc = (pc as i16 + get_imm(instr)) as u16;
                        } else {
                            pc += 2;
                        }
                    },
                    Opcode::BLT => {
                        if rs2_data > rs1_data {
                            pc = (pc as i16 + get_imm(instr)) as u16;
                        } else {
                            pc += 2;
                        }
                    },
                    _ => {},
                }
            },
        }

        println!("------------------------------------------------------------");
        println!("PC = {}", pc);
        for index in 0..4 {
            println!("R{} \t: {} \t| R{} \t: {} \t| R{} \t: {} \t| R{} \t: {}", index, RegisterFile[index], index+4, RegisterFile[index+4], index+8, RegisterFile[index+8], index+12, RegisterFile[index+12]);
        }
    }

}
