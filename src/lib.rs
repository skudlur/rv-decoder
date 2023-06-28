#![allow(warnings, unused)]

/// Takes a RISC-V 32-bit binary instruction and returns the decoded assembly instruction
///
/// # Examples:
///
/// let mut instr = Vec::new();
/// instr = ["1", "1", "1", "1", "1", "1", "1", "0", "0", "0", "0", "0", "0", "0", "0", "1", "0", "0", "0", "0", "0", "0", "0", "1", "0", "0", "0", "1", "0", "0", "1", "1"];
/// assert_eq!("Add Immediate (ADDI) instruction decoded", rv_decoder::instruction_decoder(instr));

pub fn instruction_decoder(instr: Vec<&str>) {
    /*
     * This decoder is based on the RISC-V Unprivileged Spec v2.2
     */
    /*
     * Instruction breakdown
     * 31 --------------------------------6------0
     * 0----------------------------------25------31
     * /                                  /opcode/
     */

    let opcode_slice = &instr[25..];    // opcode field
    let opcode_slice_joined = opcode_slice.join("");

    println!("--------------------------------");

    match opcode_slice_joined.as_str() {
        "0000011" => {      // Load Instructions
            let funct3_slice = &instr[17..20];
            let funct3_slice_joined = funct3_slice.join("");
            let rd_slice = &instr[20..25];
            let rd_slice_joined = rd_slice.join("");
            let rs1_slice = &instr[12..17];
            let rs1_slice_joined = rs1_slice.join("");
            let mut imm_slice = &instr[0..12];
            let mut imm_slice_joined = imm_slice.join("");

            let rd_bits = i32::from_str_radix(&rd_slice_joined, 2).unwrap();
            let rs1_bits = i32::from_str_radix(&rs1_slice_joined, 2).unwrap();
            let mut imm_bits = i32::from_str_radix(&imm_slice_joined, 2).unwrap();

            // Immediate generator/handler
            if imm_slice[0] == "1" {
                let mut x = 1;
                loop {
                    let mut twos = i32::pow(2, x);
                    if (imm_bits as f32)/(twos as f32) < 1.0 {
                        imm_bits = imm_bits - twos;
                        break;
                    }
                    else {
                        x = x + 1;
                    }
                }
            }

            match funct3_slice_joined.as_str() {
                "000" => {      // Load Byte (8-bits)
                    println!("Load Byte (LB) instruction decoded");
                    println!("Destination Register address: x{}", rd_bits);
                    println!("Register One address: x{}", rs1_bits);
                    println!("Immediate value: {}", imm_bits);
                    println!("LB x{}, {}(x{})", rd_bits, imm_bits, rs1_bits);
                    println!("--------------------------------");
                }
                "001" => {      // Load Half-word (16-bits)
                    println!("Load Half-word (LH) instruction decoded");
                    println!("Destination Register address: x{}", rd_bits);
                    println!("Register One address: x{}", rs1_bits);
                    println!("Immediate value: {}", imm_bits);
                    println!("LH x{}, {}(x{})", rd_bits, imm_bits, rs1_bits);
                    println!("--------------------------------");
                }
                "010" => {      // Load Word (32-bits)
                    println!("Load Word (LW) instruction decoded");
                    println!("Destination Register address: x{}", rd_bits);
                    println!("Register One address: x{}", rs1_bits);
                    println!("Immediate value: {}", imm_bits);
                    println!("LW x{}, {}(x{})", rd_bits, imm_bits, rs1_bits);
                    println!("--------------------------------");
                }
                "100" => {      // Load Byte Unsigned (u8-bits)
                    println!("Load Byte Unsigned (LBU) instruction decoded");
                    println!("Destination Register address: x{}", rd_bits);
                    println!("Register One address: x{}", rs1_bits);
                    println!("Immediate value: {}", imm_bits);
                    println!("LBU x{}, {}(x{})", rd_bits, imm_bits, rs1_bits);
                    println!("--------------------------------");
                }
                "101" => {      // Load Half-word Unsigned (u16-bits)
                    println!("Load Half-word Unsigned (LHU) instruction decoded");
                    println!("Destination Register address: x{}", rd_bits);
                    println!("Register One address: x{}", rs1_bits);
                    println!("Immediate value: {}", imm_bits);
                    println!("LHU x{}, {}(x{})", rd_bits, imm_bits, rs1_bits);
                    println!("--------------------------------");
                }
                default => {
                    panic!("Instruction format error!");
                }
            &_ => todo!()
            }
        }

        "0100011" => {      // Store Instructions
            let funct3_slice = &instr[17..20];
            let funct3_slice_joined = funct3_slice.join("");
            let rs2_slice = &instr[7..12];
            let rs2_slice_joined = rs2_slice.join("");
            let rs1_slice = &instr[12..17];
            let rs1_slice_joined = rs1_slice.join("");
            let mut imm_slice = &instr[0..7];
            let mut imm_slice_joined = imm_slice.join("");
            let imm2_slice = &instr[20..25];
            let imm2_slice_joined = imm2_slice.join("");

            imm_slice_joined = imm_slice_joined + &imm2_slice_joined;
            let rs1_bits = i32::from_str_radix(&rs1_slice_joined, 2).unwrap();
            let rs2_bits = i32::from_str_radix(&rs2_slice_joined, 2).unwrap();
            let mut imm_bits = i32::from_str_radix(&imm_slice_joined, 2).unwrap();

            // Immediate generator/handler
            if imm_slice[0] == "1" {
                let mut x = 1;
                loop {
                    let mut twos = i32::pow(2, x);
                    if (imm_bits as f32)/(twos as f32) < 1.0 {
                        imm_bits = imm_bits - twos;
                        break;
                    }
                    else {
                        x = x + 1;
                    }
                }
            }

            match funct3_slice_joined.as_str() {
                "000" => {      // Store Byte (8-bits)
                    println!("Store Byte (SB) instruction decoded");
                    println!("Register One address: x{}", rs1_bits);
                    println!("Register Two address: x{}", rs2_bits);
                    println!("Immediate value: {}", imm_bits);
                    println!("SB x{}, {}(x{})", rs2_bits, imm_bits, rs1_bits);
                    println!("--------------------------------");
                }
                "001" => {      // Store Half-word (16-bit)
                    println!("Store Half-word (SH) instruction decoded");
                    println!("Register One address: x{}", rs1_bits);
                    println!("Register Two address: x{}", rs2_bits);
                    println!("Immediate value: {}", imm_bits);
                    println!("SH x{}, {}(x{})", rs2_bits, imm_bits, rs1_bits);
                    println!("--------------------------------");
                }
                "010" => {      // Store Word (32-bit)
                    println!("Store Word (SW) instruction decoded");
                    println!("Register One address: x{}", rs1_bits);
                    println!("Register Two address: x{}", rs2_bits);
                    println!("Immediate value: {}", imm_bits);
                    println!("SW x{}, {}(x{})", rs2_bits, imm_bits, rs1_bits);
                    println!("--------------------------------");
                }
                default => {
                    panic!("Instruction format error!");
                }
            &_ => todo!()
            }
        }

        "0010011" => {      // Immediate type instructions
            let funct3_slice = &instr[17..20];
            let funct3_slice_joined = funct3_slice.join("");
            let rd_slice = &instr[20..25];
            let rd_slice_joined = rd_slice.join("");
            let rs1_slice = &instr[12..17];
            let rs1_slice_joined = rs1_slice.join("");
            let imm_slice = &instr[0..12];
            let mut imm_slice_joined = imm_slice.join("");

            let rd_bits = i32::from_str_radix(&rd_slice_joined, 2).unwrap();
            let rs1_bits = i32::from_str_radix(&rs1_slice_joined, 2).unwrap();
            let mut imm_bits = i32::from_str_radix(&imm_slice_joined, 2).unwrap();

            // Immediate generator/handler
            if imm_slice[0] == "1" {
                let mut x = 1;
                loop {
                    let mut twos = i32::pow(2, x);
                    if  (imm_bits as f32)/(twos as f32) < 1.0 {
                        imm_bits = imm_bits - twos;
                        break;
                    }
                    else {
                        x = x + 1;
                    }
                }
            }

            match funct3_slice_joined.as_str() {
                "000" => {      // Add immediate
                    println!("Add Immediate (ADDI) instruction decoded");
                    println!("Destination Register address: x{}", rd_bits);
                    println!("Register One address: x{}", rs1_bits);
                    println!("Immediate value: {}", imm_bits);
                    println!("ADDI x{}, x{}, {}", rd_bits, rs1_bits, imm_bits);
                    println!("--------------------------------");
                }
                "010" => {      // Set less than immediate
                    println!("Set less than Immediate (SLTI) instruction decoded");
                    println!("Destination Register address: x{}", rd_bits);
                    println!("Register One address: x{}", rs1_bits);
                    println!("Immediate value: {}", imm_bits);
                    println!("SLTI x{}, x{}, {}", rd_bits, rs1_bits, imm_bits);
                    println!("--------------------------------");
                }
                "011" => {      // Set less than immediate unsigned
                    println!("Set less than Immediate unsigned (SLTIU) instruction decoded");
                    println!("Destination Register address: x{}", rd_bits);
                    println!("Register One address: x{}", rs1_bits);
                    println!("Immediate value: {}", imm_bits);
                    println!("SLTIU x{}, x{}, {}", rd_bits, rs1_bits, imm_bits);
                    println!("--------------------------------");
                }
                "100" => {      // XOR Immediate
                    println!("XOR Immediate (XORI) instruction decoded");
                    println!("Destination Register address: x{}", rd_bits);
                    println!("Register One address: x{}", rs1_bits);
                    println!("Immediate value: {}", imm_bits);
                    println!("XORI x{}, x{}, {}", rd_bits, rs1_bits, imm_bits);
                    println!("--------------------------------");
                }
                "110" => {      // OR Immediate
                    println!("OR Immediate (ORI) instruction decoded");
                    println!("Destination Register address: x{}", rd_bits);
                    println!("Register One address: x{}", rs1_bits);
                    println!("Immediate value: {}", imm_bits);
                    println!("ORI x{}, x{}, {}", rd_bits, rs1_bits, imm_bits);
                    println!("--------------------------------");
                }
                "111" => {      // AND Immediate
                    println!("AND Immediate (ANDI) instruction decoded");
                    println!("Destination Register address: x{}", rd_bits);
                    println!("Register One address: x{}", rs1_bits);
                    println!("Immediate value: {}", imm_bits);
                    println!("ANDI x{}, x{}, {}", rd_bits, rs1_bits, imm_bits);
                    println!("--------------------------------");
                }
                default => {
                    panic!("Instruction format error!");
                }
            &_ => todo!()
            }
        }

        "0110111" => {      // Load upper immediate
            let rd_slice = &instr[20..25];
            let rd_slice_joined = rd_slice.join("");
            let imm_slice = &instr[0..20];
            let imm_slice_joined = imm_slice.join("");

            let rd_bits = i32::from_str_radix(&rd_slice_joined, 2).unwrap();
            let mut imm_bits = i32::from_str_radix(&imm_slice_joined, 2).unwrap();

            // Immediate generator/handler
            if imm_slice[0] == "1" {
                let mut x = 1;
                loop {
                    let mut twos = i32::pow(2, x);
                    if  (imm_bits as f32)/(twos as f32) < 1.0 {
                        imm_bits = imm_bits - twos;
                        break;
                    }
                    else {
                        x = x + 1;
                    }
                }
            }

            println!("Load Upper Immediate (LUI) instruction decoded");
            println!("Destination Register address: x{}", rd_bits);
            println!("Immediate address: x{}", imm_bits);
            println!("LUI x{}, {}", rd_bits, imm_bits);
            println!("--------------------------------");
        }

        "0010111" => {      // Add upper immediate with PC
            let rd_slice = &instr[20..25];
            let rd_slice_joined = rd_slice.join("");
            let imm_slice = &instr[0..20];
            let imm_slice_joined = imm_slice.join("");

            let rd_bits = i32::from_str_radix(&rd_slice_joined, 2).unwrap();
            let mut imm_bits = i32::from_str_radix(&imm_slice_joined, 2).unwrap();

            // Immediate generator/handler
            if imm_slice[0] == "1" {
                let mut x = 1;
                loop {
                    let mut twos = i32::pow(2, x);
                    if  (imm_bits as f32)/(twos as f32) < 1.0 {
                        imm_bits = imm_bits - twos;
                        break;
                    }
                    else {
                        x = x + 1;
                    }
                }
            }

            println!("Add upper immediate with PC (AUIPC) instruction decoded");
            println!("Destination Register address: x{}", rd_bits);
            println!("Immediate address: x{}", imm_bits);
            println!("AUIPC x{}, {}", rd_bits, imm_bits);
            println!("--------------------------------");
        }

        "1101111" => {      // Jump and link
            let rd_slice = &instr[20..25];
            let rd_slice_joined = rd_slice.join("");
            let imm_slice = &instr[0..20];
            let imm_slice_joined = imm_slice.join("");
            let imm_slice_1 = imm_slice[0].to_string();        // imm[19]
            //let imm_slice_1_joined = imm_slice_1.join("");
            let imm_slice_2 = &imm_slice[1..10];    // imm[7:0]
            let imm_slice_2_joined = imm_slice_2.join("");
            let imm_slice_3 = imm_slice[10].to_string();       // imm[8]
           // let imm_slice_3_joined = imm_slice_3.join("");
            let imm_slice_4 = &imm_slice[11..20];   // imm[18:9]
            let imm_slice_4_joined = imm_slice_4.join("");
            let mut imm_final = imm_slice_2_joined + &imm_slice_3 + &imm_slice_4_joined + &imm_slice_1;

            let rd_bits = u32::from_str_radix(&rd_slice_joined, 2).unwrap();
            let mut imm_bits = i32::from_str_radix(&imm_final, 2).unwrap();

            println!("Jump and Link (JAL) instruction decoded");
            println!("Destination Register address: x{}", rd_bits);
            println!("Immediate address: x{}", imm_bits);
            println!("JAL x{}, {}", rd_bits, imm_bits);
            println!("--------------------------------");
        }

        "1100111" => {      // Jump and link to register
            let rd_slice = &instr[20..25];
            let rd_slice_joined = rd_slice.join("");
            let imm_slice = &instr[0..12];
            let imm_slice_joined = imm_slice.join("");
            let rs1_slice = &instr[12..17];
            let rs1_slice_joined = rs1_slice.join("");

            let rd_bits = i32::from_str_radix(&rd_slice_joined, 2).unwrap();
            let rs1_bits = i32::from_str_radix(&rs1_slice_joined, 2).unwrap();
            let mut imm_bits = i32::from_str_radix(&imm_slice_joined, 2).unwrap();

            // Immediate generator/handler
            if imm_slice[0] == "1" {
                let mut x = 1;
                loop {
                    let mut twos = i32::pow(2, x);
                    if  (imm_bits as f32)/(twos as f32) < 1.0 {
                        imm_bits = imm_bits - twos;
                        break;
                    }
                    else {
                        x = x + 1;
                    }
                }
            }

            println!("Jump and Link to register (JALR) instruction decoded");
            println!("Destination Register address: x{}", rd_bits);
            println!("Register one address: x{}", rs1_bits);
            println!("Immediate value: {}", imm_bits);
            println!("JALR x{}, x{}, {}", rd_bits, rs1_bits, imm_bits);
            println!("--------------------------------");
        }

        "0110011" => {      // Arithmetic instructions
            let funct3_slice = &instr[17..20];
            let funct3_slice_joined = funct3_slice.join("");
            let funct7_slice = &instr[0..7];
            let funct7_slice_joined = funct7_slice.join("");
            let rs2_slice = &instr[7..12];
            let rs2_slice_joined = rs2_slice.join("");
            let rs1_slice = &instr[12..17];
            let rs1_slice_joined = rs1_slice.join("");
            let rd_slice = &instr[20..25];
            let rd_slice_joined = rd_slice.join("");

            let rs1_bits = i32::from_str_radix(&rs1_slice_joined, 2).unwrap();
            let rs2_bits = i32::from_str_radix(&rs2_slice_joined, 2).unwrap();
            let mut rd_bits = i32::from_str_radix(&rd_slice_joined, 2).unwrap();

            match funct3_slice_joined.as_str() {
                "000" => {
                    match funct7_slice_joined.as_str() {
                        "0000000" => {      // Add
                            println!("Addition (ADD) instruction decoded");
                            println!("Destination Register address: x{}", rd_bits);
                            println!("Register One address: x{}", rs1_bits);
                            println!("Register Two value: {}", rs2_bits);
                            println!("ADD x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                            println!("--------------------------------");
                        }
                        "0100000" => {      // Sub
                            println!("Subtraction (SUB) instruction decoded");
                            println!("Destination Register address: x{}", rd_bits);
                            println!("Register One address: x{}", rs1_bits);
                            println!("Register Two value: {}", rs2_bits);
                            println!("SUB x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                            println!("--------------------------------");
                        }
                        &_ => todo!()
                    }
                }
                "001" => {      // Shift left logical
                    println!("Shift Left Logical (SLL) instruction decoded");
                    println!("Destination Register address: x{}", rd_bits);
                    println!("Register One address: x{}", rs1_bits);
                    println!("Register Two value: {}", rs2_bits);
                    println!("SLL x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                    println!("--------------------------------");
                }
                "010" => {      // Set less than
                    println!("Set less than (SLT) instruction decoded");
                    println!("Destination Register address: x{}", rd_bits);
                    println!("Register One address: x{}", rs1_bits);
                    println!("Register Two value: {}", rs2_bits);
                    println!("SLT x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                    println!("--------------------------------");
                }
                "011" => {      // Set less than unsigned
                    println!("Set less than unsigned (SLTU) instruction decoded");
                    println!("Destination Register address: x{}", rd_bits);
                    println!("Register One address: x{}", rs1_bits);
                    println!("Register Two value: {}", rs2_bits);
                    println!("SLTU x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                    println!("--------------------------------");
                }
                "100" => {      // XOR
                    println!("XOR instruction decoded");
                    println!("Destination Register address: x{}", rd_bits);
                    println!("Register One address: x{}", rs1_bits);
                    println!("Register Two value: {}", rs2_bits);
                    println!("XOR x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                    println!("--------------------------------");
                }
                "101" => {      // Shift right
                    match funct7_slice_joined.as_str() {
                        "0000000" => {      // Shift right logical
                            println!("Shift Right Logical (SRL) instruction decoded");
                            println!("Destination Register address: x{}", rd_bits);
                            println!("Register One address: x{}", rs1_bits);
                            println!("Register Two value: {}", rs2_bits);
                            println!("SRL x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                            println!("--------------------------------");
                        }
                        "0100000" => {      // Shift right arithmetic
                            println!("Shift Right Arithmetic (SRA) instruction decoded");
                            println!("Destination Register address: x{}", rd_bits);
                            println!("Register One address: x{}", rs1_bits);
                            println!("Register Two value: {}", rs2_bits);
                            println!("SRA x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                            println!("--------------------------------");
                        }
                        &_ => todo!()
                    }
                }
                "110" => {      // OR
                    println!("OR instruction decoded");
                    println!("Destination Register address: x{}", rd_bits);
                    println!("Register One address: x{}", rs1_bits);
                    println!("Register Two value: {}", rs2_bits);
                    println!("OR x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                    println!("--------------------------------");
                }
                "111" => {      // AND
                    println!("AND instruction decoded");
                    println!("Destination Register address: x{}", rd_bits);
                    println!("Register One address: x{}", rs1_bits);
                    println!("Register Two value: {}", rs2_bits);
                    println!("AND x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                    println!("--------------------------------");
                }
            &_ => todo!()
            }
        }
        default => {
            panic!("Opcode not found!");
        }
    &_ => todo!()
    }
}
