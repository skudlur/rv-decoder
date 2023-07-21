#![allow(warnings, unused)]

/// Takes a RISC-V 32-bit binary instruction and returns the decoded assembly instruction
///
/// # Examples:
///
/// let mut instr = Vec::new();
/// instr = ["1", "1", "1", "1", "1", "1", "1", "0", "0", "0", "0", "0", "0", "0", "0", "1", "0", "0", "0", "0", "0", "0", "0", "1", "0", "0", "0", "1", "0", "0", "1", "1"];
/// assert_eq!("Add Immediate (ADDI) instruction decoded", rv_decoder::instruction_decoder(instr));

pub fn convert_binary_string_to_vector(binary_string: &str) -> Vec<String> {
    let mut vector = Vec::new();
    for i in 0..binary_string.len() {
      vector.push(binary_string.chars().nth(i).unwrap().to_string());
    }
    return vector;
  }

fn rm_decoder(rm_slice_joined: &str) -> String {
    match rm_slice_joined {
        "000" => {
            return "RNE".to_string(); // Round to nearest, ties to even
        }
        "001" => {
            return "RTZ".to_string(); // Round towards zero
        }
        "010" => {
            return "RDN".to_string(); // Round down (towards negative infinity)
        }
        "011" => {
            return "RUP".to_string(); // Round up (towards positive infinity)
        }
        "100" => {
            return "RMM".to_string(); // Round to nearest, ties to max magnitude
        }
        "101" => {
            return "Invalid".to_string();
        }
        "110" => {
            return "Invalid".to_string();
        }
        "111" => {
            return "DYN".to_string(); // Dynamic rounding mode 
        }
        &_ => todo!()
    }
}


pub fn instruction_decoder(instr: Vec<String>) -> String {
    /*
     * This decoder is based on the RISC-V Unprivileged Spec v2.2
     *
     * Instruction breakdown
     * RV32I Base Instruction Set`
     * 
     * Note: imm = offset = immediate value
     *
     * R-type 
     * 31------25 24-----20 19-----15 14-----12 11-------7 6-------0
     * /func7/     /rs2/     /rs1/     /func3/     /rd/      /opcode/
     * 0-------6 7-----11 12-----16 17--------19 20-----24 25--------31
     * /func7/    /rs2/     /rs1/     /func3/      /rd/      /opcode/
     *  
     * I-type
     * 31---------20 19-----15 14-------12 11------7 6--------0
     * /imm[11:0]/     /rs1/     /func3/     /rd/      /opcode/
     * 0----------11 12-----16 17-------19 20-----24 25-------31
     * /imm[11:0]/     /rs1/     /func3/     /rd/      /opcode/ 
     * 
     * S-type
     * 31--------25 24-----20 19-----15 14--------12 11-----------7 6---------0
     * /imm[11:5]/    /rs2/     /rs1/     /func3/      /imm[4:0]/     /opcode/
     * 0----------6 7------11 12-----16 17--------19 20----------24 25--------31
     * /imm[11:5]/    /rs2/      /rs1/    /func3/      /imm[4:0]/     /opcode/
     * 
     * B-type
     * 31------------25 24-----20 19-----15 14-------12 11--------------7 6---------0
     * /imm[12|10:5]/     /rs2/     /rs1/     /func3/     /imm[4:1|11]/    /opcode/
     * 0--------------6 7------11 12-----16 17-------19 20-------------24 25--------31
     * /imm[12|10:5]/     /rs2/     /rs1/     /func3/      /imm[4:1|11]/    /opcode/
     * 
     * U-type
     * 31-----------12 11-------7 6---------0
     * /imm[31:12]/       /rd/     /opcode/
     * 0------------19 20------24 25--------31
     * /imm[31:12]/       /rd/      /opcode/
     * 
     * J-type
     * 31----------------------12 11------7 6----------0
     * /imm[20|10:1|11|19:12]/      /rd/      /opcode/
     * 0-----------------------19 20-----24 25--------31
     * /imm[20|10:1|11|19:12]/      /rd/      /opcode/ 
     *                               
     * 
     * RV32A Atomic Instruction Breakdown
     * 
     * 31--------27 26----- 25-----  24-----20 19------15 14-------12 11------7 6--------0
     *   /func5/      /aq/    /rl/     /rs2/     /rs1/      /func3/      /rd/    /opcode/
     * 0---------4  5------ 6------  7------11 12------16 17-------19 20-----24 25-------31
     *   /func5/      /aq/    /rl/     /rs2/     /rs1/      /func3/      /rd/    /opcode/
     * 
     * RV32F/RV32D Instruction Breakdown
     * 
     * 31------27 26----25  24-----20 19------15 14----12 11----7 6--------0
     *   /rs3/      /00/      /rs2/     /rs1/      /rm/     /rd/   /opcode/
     * 0-------4  5-----6  7------11 12------16 17-----19 20----24 25-------31
     *  /rs3/      /00/      /rs2/     /rs1/      /rm/     /rd/     /opcode/
     * 
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
                    return format!("LB x{}, {}(x{})", rd_bits, imm_bits, rs1_bits);
                }
                "001" => {      // Load Half-word (16-bits)
                    println!("Load Half-word (LH) instruction decoded");
                    println!("Destination Register address: x{}", rd_bits);
                    println!("Register One address: x{}", rs1_bits);
                    println!("Immediate value: {}", imm_bits);
                    println!("LH x{}, {}(x{})", rd_bits, imm_bits, rs1_bits);
                    println!("--------------------------------");
                    return format!("LH x{}, {}(x{})", rd_bits, imm_bits, rs1_bits);
                }
                "010" => {      // Load Word (32-bits)
                    println!("Load Word (LW) instruction decoded");
                    println!("Destination Register address: x{}", rd_bits);
                    println!("Register One address: x{}", rs1_bits);
                    println!("Immediate value: {}", imm_bits);
                    println!("LW x{}, {}(x{})", rd_bits, imm_bits, rs1_bits);
                    println!("--------------------------------");
                    return format!("LW x{}, {}(x{})", rd_bits, imm_bits, rs1_bits);
                }
                "100" => {      // Load Byte Unsigned (u8-bits)
                    println!("Load Byte Unsigned (LBU) instruction decoded");
                    println!("Destination Register address: x{}", rd_bits);
                    println!("Register One address: x{}", rs1_bits);
                    println!("Immediate value: {}", imm_bits);
                    println!("LBU x{}, {}(x{})", rd_bits, imm_bits, rs1_bits);
                    println!("--------------------------------");
                    return format!("LBU x{}, {}(x{})", rd_bits, imm_bits, rs1_bits);
                }
                "101" => {      // Load Half-word Unsigned (u16-bits)
                    println!("Load Half-word Unsigned (LHU) instruction decoded");
                    println!("Destination Register address: x{}", rd_bits);
                    println!("Register One address: x{}", rs1_bits);
                    println!("Immediate value: {}", imm_bits);
                    println!("LHU x{}, {}(x{})", rd_bits, imm_bits, rs1_bits);
                    println!("--------------------------------");
                    return format!("LHU x{}, {}(x{})", rd_bits, imm_bits, rs1_bits);
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
                    return format!("SB x{}, {}(x{})", rs2_bits, imm_bits, rs1_bits);
                }
                "001" => {      // Store Half-word (16-bit)
                    println!("Store Half-word (SH) instruction decoded");
                    println!("Register One address: x{}", rs1_bits);
                    println!("Register Two address: x{}", rs2_bits);
                    println!("Immediate value: {}", imm_bits);
                    println!("SH x{}, {}(x{})", rs2_bits, imm_bits, rs1_bits);
                    println!("--------------------------------");
                    return format!("SH x{}, {}(x{})", rs2_bits, imm_bits, rs1_bits);
                }
                "010" => {      // Store Word (32-bit)
                    println!("Store Word (SW) instruction decoded");
                    println!("Register One address: x{}", rs1_bits);
                    println!("Register Two address: x{}", rs2_bits);
                    println!("Immediate value: {}", imm_bits);
                    println!("SW x{}, {}(x{})", rs2_bits, imm_bits, rs1_bits);
                    println!("--------------------------------");
                    return format!("SW x{}, {}(x{})", rs2_bits, imm_bits, rs1_bits);
                }
                default => {
                    panic!("Instruction format error!");
                }
            &_ => todo!()
            }
        }

        "1100011" =>{
            let funct3_slice = &instr[17..20];
            let funct3_slice_joined = funct3_slice.join("");
            let rs1_slice = &instr[12..17];
            let rs1_slice_joined = rs1_slice.join("");
            let rs2_slice = &instr[7..12];
            let rs2_slice_joined = rs2_slice.join("");
            let imm1_slice = &instr[0..7];
            let imm2_slice = &instr[20..25];
            let imm_slice_1 = imm1_slice[0].to_string(); // imm [12]
            let imm_slice_2 = &imm1_slice[1..7]; // imm [10:5]
            let imm_slice_2_joined = imm_slice_2.join("");
            let imm_slice_3 = &imm2_slice[0..4]; // imm [4:1]
            let imm_slice_3_joined = imm_slice_3.join("");
            let imm_slice_4 = imm2_slice[4].to_string(); // imm [11]
            let zero = "0".to_string();
            let mut imm_final = imm_slice_1.clone() + &imm_slice_4 + &imm_slice_2_joined + &imm_slice_3_joined + &zero;

            let rs1_bits = i32::from_str_radix(&rs1_slice_joined, 2).unwrap();
            let rs2_bits = i32::from_str_radix(&rs2_slice_joined, 2).unwrap();
            let mut imm_bits = i32::from_str_radix(&imm_final, 2).unwrap();

            // Immediate generator/handler
            if imm_slice_1 == "1" {
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
                "000" => {      // Branch Equal
                    println!("Branch Equal (BEQ) instruction decoded");
                    println!("Register One address: x{}", rs1_bits);
                    println!("Register Two address: x{}", rs2_bits);
                    println!("Immediate value: {}", imm_bits);
                    println!("BEQ x{}, x{}, {}", rs1_bits, rs2_bits, imm_bits);
                    println!("--------------------------------");
                    return format!("BEQ x{}, x{}, {}", rs1_bits, rs2_bits, imm_bits);
                }
                "001" => {      // Branch Not Equal
                    println!("Branch Not Equal (BNE) instruction decoded");
                    println!("Register One address: x{}", rs1_bits);
                    println!("Register Two address: x{}", rs2_bits);
                    println!("Immediate value: {}", imm_bits);
                    println!("BNE x{}, x{}, {}", rs1_bits, rs2_bits, imm_bits);
                    println!("--------------------------------");
                    return format!("BNE x{}, x{}, {}", rs1_bits, rs2_bits, imm_bits);
                }
                "100" => {      // Branch Less Than
                    println!("Branch Less Than (BLT) instruction decoded");
                    println!("Register One address: x{}", rs1_bits);
                    println!("Register Two address: x{}", rs2_bits);
                    println!("Immediate value: {}", imm_bits);
                    println!("BLT x{}, x{}, {}", rs1_bits, rs2_bits, imm_bits);
                    println!("--------------------------------");
                    return format!("BLT x{}, x{}, {}", rs1_bits, rs2_bits, imm_bits);
                }
                "101" => {      // Branch Greater Than or Equal
                    println!("Branch Greater Than or Equal (BGE) instruction decoded");
                    println!("Register One address: x{}", rs1_bits);
                    println!("Register Two address: x{}", rs2_bits);
                    println!("Immediate value: {}", imm_bits);
                    println!("BGE x{}, x{}, {}", rs1_bits, rs2_bits, imm_bits);
                    println!("--------------------------------");
                    return format!("BGE x{}, x{}, {}", rs1_bits, rs2_bits, imm_bits);
                }
                "110" => {      // Branch Less Than Unsigned
                    println!("Branch Less Than Unsigned (BLTU) instruction decoded");
                    println!("Register One address: x{}", rs1_bits);
                    println!("Register Two address: x{}", rs2_bits);
                    println!("Immediate value: {}", imm_bits);
                    println!("BLTU x{}, x{}, {}", rs1_bits, rs2_bits, imm_bits);
                    println!("--------------------------------");
                    return format!("BLTU x{}, x{}, {}", rs1_bits, rs2_bits, imm_bits);
                }
                "111" => {      // Branch Greater Than or Equal Unsigned
                    println!("Branch Greater Than or Equal Unsigned (BGEU) instruction decoded");
                    println!("Register One address: x{}", rs1_bits);
                    println!("Register Two address: x{}", rs2_bits);
                    println!("Immediate value: {}", imm_bits);
                    println!("BGEU x{}, x{}, {}", rs1_bits, rs2_bits, imm_bits);
                    println!("--------------------------------");
                    return format!("BGEU x{}, x{}, {}", rs1_bits, rs2_bits, imm_bits);
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
                    return format!("ADDI x{}, x{}, {}", rd_bits, rs1_bits, imm_bits);
                }
                "010" => {      // Set less than immediate
                    println!("Set less than Immediate (SLTI) instruction decoded");
                    println!("Destination Register address: x{}", rd_bits);
                    println!("Register One address: x{}", rs1_bits);
                    println!("Immediate value: {}", imm_bits);
                    println!("SLTI x{}, x{}, {}", rd_bits, rs1_bits, imm_bits);
                    println!("--------------------------------");
                    return format!("SLTI x{}, x{}, {}", rd_bits, rs1_bits, imm_bits);
                }
                "011" => {      // Set less than immediate unsigned
                    println!("Set less than Immediate unsigned (SLTIU) instruction decoded");
                    println!("Destination Register address: x{}", rd_bits);
                    println!("Register One address: x{}", rs1_bits);
                    println!("Immediate value: {}", imm_bits);
                    println!("SLTIU x{}, x{}, {}", rd_bits, rs1_bits, imm_bits);
                    println!("--------------------------------");
                    return format!("SLTIU x{}, x{}, {}", rd_bits, rs1_bits, imm_bits);
                }
                "100" => {      // XOR Immediate
                    println!("XOR Immediate (XORI) instruction decoded");
                    println!("Destination Register address: x{}", rd_bits);
                    println!("Register One address: x{}", rs1_bits);
                    println!("Immediate value: {}", imm_bits);
                    println!("XORI x{}, x{}, {}", rd_bits, rs1_bits, imm_bits);
                    println!("--------------------------------");
                    return format!("XORI x{}, x{}, {}", rd_bits, rs1_bits, imm_bits);
                }
                "110" => {      // OR Immediate
                    println!("OR Immediate (ORI) instruction decoded");
                    println!("Destination Register address: x{}", rd_bits);
                    println!("Register One address: x{}", rs1_bits);
                    println!("Immediate value: {}", imm_bits);
                    println!("ORI x{}, x{}, {}", rd_bits, rs1_bits, imm_bits);
                    println!("--------------------------------");
                    return format!("ORI x{}, x{}, {}", rd_bits, rs1_bits, imm_bits);
                }
                "111" => {      // AND Immediate
                    println!("AND Immediate (ANDI) instruction decoded");
                    println!("Destination Register address: x{}", rd_bits);
                    println!("Register One address: x{}", rs1_bits);
                    println!("Immediate value: {}", imm_bits);
                    println!("ANDI x{}, x{}, {}", rd_bits, rs1_bits, imm_bits);
                    println!("--------------------------------");
                    return format!("ANDI x{}, x{}, {}", rd_bits, rs1_bits, imm_bits);
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
            return format!("LUI x{}, {}", rd_bits, imm_bits);
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
            return format!("AUIPC x{}, {}", rd_bits, imm_bits);
        }

        "1101111" => {      // Jump and link
            let rd_slice = &instr[20..25];
            let rd_slice_joined = rd_slice.join("");
            let imm_slice = &instr[0..20];                      // imm[20:1]
            let mut imm_slice_offset = &imm_slice[0..12];
            let imm_slice_joined = imm_slice_offset.join("");

            let rd_bits = u32::from_str_radix(&rd_slice_joined, 2).unwrap();
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

            println!("Jump and Link (JAL) instruction decoded");
            println!("Destination Register address: x{}", rd_bits);
            println!("Immediate address: {}", imm_bits);
            println!("JAL x{}, {}", rd_bits, imm_bits);
            println!("--------------------------------");
            return format!("JAL x{}, {}", rd_bits, imm_bits);
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
            return format!("JALR x{}, x{}, {}", rd_bits, rs1_bits, imm_bits);
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
                            return format!("ADD x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                        }
                        "0100000" => {      // Sub
                            println!("Subtraction (SUB) instruction decoded");
                            println!("Destination Register address: x{}", rd_bits);
                            println!("Register One address: x{}", rs1_bits);
                            println!("Register Two value: {}", rs2_bits);
                            println!("SUB x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                            println!("--------------------------------");
                            return format!("SUB x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                        }
                        "0000001" => {      // Multiply signed rs1 and rs2
                            println!("Multiplication (MUL) instruction decoded");
                            println!("Destination Register address: x{}", rd_bits);
                            println!("Register One address: x{}", rs1_bits);
                            println!("Register Two value: {}", rs2_bits);
                            println!("MUL x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                            println!("--------------------------------");
                            return format!("MUL x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                        }
                        &_ => todo!()
                    }
                }
                "001" => {      
                    match funct7_slice_joined.as_str() {
                        "0000000" => {      // Shift left logical
                            println!("Shift Left Logical (SLL) instruction decoded");
                            println!("Destination Register address: x{}", rd_bits);
                            println!("Register One address: x{}", rs1_bits);
                            println!("Register Two value: {}", rs2_bits);
                            println!("SLL x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                            println!("--------------------------------");
                            return format!("SLL x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                        }
                        "0000001" => {      // Multiply high signed rs1 and rs2
                            println!("Multiply High Signed (MULH) instruction decoded");
                            println!("Destination Register address: x{}", rd_bits);
                            println!("Register One address: x{}", rs1_bits);
                            println!("Register Two value: {}", rs2_bits);
                            println!("MULH x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                            println!("--------------------------------");
                            return format!("MULH x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                        }
                        &_ => todo!()
                    }
                }
                "010" => {      
                    match funct7_slice_joined.as_str() {
                        "0000000" => {      // Set less than
                            println!("Set less than (SLT) instruction decoded");
                            println!("Destination Register address: x{}", rd_bits);
                            println!("Register One address: x{}", rs1_bits);
                            println!("Register Two value: {}", rs2_bits);
                            println!("SLT x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                            println!("--------------------------------");
                            return format!("SLT x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                        }
                        "0000001" => {      // Multiply signed rs1 and unsigned rs2
                            println!("Multiply High Unsigned with signed (MULHSU) instruction decoded");
                            println!("Destination Register address: x{}", rd_bits);
                            println!("Register One address: x{}", rs1_bits);
                            println!("Register Two value: {}", rs2_bits);
                            println!("MULHSU x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                            println!("--------------------------------");
                            return format!("MULHSU x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                        }
                        &_ => todo!()
                    }
                }
                "011" => {      
                    match funct7_slice_joined.as_str() {
                        "0000000" => {      // Set less than unsigned
                            println!("Set less than unsigned (SLTU) instruction decoded");
                            println!("Destination Register address: x{}", rd_bits);
                            println!("Register One address: x{}", rs1_bits);
                            println!("Register Two value: {}", rs2_bits);
                            println!("SLTU x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                            println!("--------------------------------");
                            return format!("SLTU x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                        }
                        "0000001" => {      // Multiply unsigned rs1 and rs2
                            println!("Multiply High Unsigned (MULHU) instruction decoded");
                            println!("Destination Register address: x{}", rd_bits);
                            println!("Register One address: x{}", rs1_bits);
                            println!("Register Two value: {}", rs2_bits);
                            println!("MULHU x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                            println!("--------------------------------");
                            return format!("MULHU x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                        }
                        &_ => todo!()
                    }
                }
                "100" => {      
                    match funct7_slice_joined.as_str() {
                        "0000000" => {      // XOR
                            println!("XOR instruction decoded");
                            println!("Destination Register address: x{}", rd_bits);
                            println!("Register One address: x{}", rs1_bits);
                            println!("Register Two value: {}", rs2_bits);
                            println!("XOR x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                            println!("--------------------------------");
                            return format!("XOR x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                        }
                        "0000001" => {      // Divide signed rs1 and rs2 (rounding towards zero)
                            println!("Divide Signed (DIV) instruction decoded");
                            println!("Destination Register address: x{}", rd_bits);
                            println!("Register One address: x{}", rs1_bits);
                            println!("Register Two address: x{}", rs2_bits);
                            println!("DIV x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                            println!("--------------------------------");
                            return format!("DIV x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                        }
                        &_ => todo!()
                    }
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
                            return format!("SRL x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                        }
                        "0000001" => {      // Divide unsigned rs1 and rs2 (rounding towards zero)
                            println!("Divide Unsigned (DIVU) instruction decoded");
                            println!("Destination Register address: x{}", rd_bits);
                            println!("Register One address: x{}", rs1_bits);
                            println!("Register Two address: x{}", rs2_bits);
                            println!("DIVU x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                            println!("--------------------------------");
                            return format!("DIVU x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                        }
                        "0100000" => {      // Shift right arithmetic
                            println!("Shift Right Arithmetic (SRA) instruction decoded");
                            println!("Destination Register address: x{}", rd_bits);
                            println!("Register One address: x{}", rs1_bits);
                            println!("Register Two value: {}", rs2_bits);
                            println!("SRA x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                            println!("--------------------------------");
                            return format!("SRA x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                        }
                        &_ => todo!()
                    }
                }
                "110" => {      
                    match funct7_slice_joined.as_str() {
                        "0000000" => {      // OR
                            println!("OR instruction decoded");
                            println!("Destination Register address: x{}", rd_bits);
                            println!("Register One address: x{}", rs1_bits);
                            println!("Register Two value: {}", rs2_bits);
                            println!("OR x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                            println!("--------------------------------");
                            return format!("OR x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                        }
                        "0000001" => {      // Remainder signed rs1 and rs2
                            println!("Remainder Signed (REM) instruction decoded");
                            println!("Destination Register address: x{}", rd_bits);
                            println!("Register One address: x{}", rs1_bits);
                            println!("Register Two address: x{}", rs2_bits);
                            println!("REM x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                            println!("--------------------------------");
                            return format!("REM x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                        }
                        &_ => todo!()
                    }
                }
                "111" => {      
                    match funct7_slice_joined.as_str() {
                        "0000000" => {      // AND
                            println!("AND instruction decoded");
                            println!("Destination Register address: x{}", rd_bits);
                            println!("Register One address: x{}", rs1_bits);
                            println!("Register Two value: {}", rs2_bits);
                            println!("AND x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                            println!("--------------------------------");
                            return format!("AND x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                        }
                        "0000001" => {      // Remainder unsigned rs1 and rs2
                            println!("Remainder Unsigned (REMU) instruction decoded");
                            println!("Destination Register address: x{}", rd_bits);
                            println!("Register One address: x{}", rs1_bits);
                            println!("Register Two address: x{}", rs2_bits);
                            println!("REMU x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                            println!("--------------------------------");
                            return format!("REMU x{}, x{}, x{}", rd_bits, rs1_bits, rs2_bits);
                        }
                        &_ => todo!()
                    }
                }
            &_ => todo!()
            }
        }

        "0101111" => {
            let funct5_slice = &instr[0..5];
            let funct5_slice_joined = funct5_slice.join("");
            let func3_slice = &instr[17..20];
            let func3_slice_joined = func3_slice.join("");
            let aq_slice = &instr[5].to_string();
            let rl_slice = &instr[6].to_string();
            let rs2_slice = &instr[7..12];
            let rs2_slice_joined = rs2_slice.join("");
            let rs1_slice = &instr[12..17];
            let rs1_slice_joined = rs1_slice.join("");
            let rd_slice = &instr[20..25];
            let rd_slice_joined = rd_slice.join("");

            let rs1_bits = i32::from_str_radix(&rs1_slice_joined, 2).unwrap();
            let rs2_bits = i32::from_str_radix(&rs2_slice_joined, 2).unwrap();
            let mut rd_bits = i32::from_str_radix(&rd_slice_joined, 2).unwrap();

            match funct5_slice_joined.as_str() {
                "00010" => {      // Load Word
                    println!("Load Word (LR.W) instruction decoded");
                    println!("Destination Register address: x{}", rd_bits);
                    println!("Register One address: x{}", rs1_bits);
                    println!("LR.W x{}, x{}", rd_bits, rs1_bits);
                    println!("--------------------------------");
                    return format!("LR.W x{}, x{}", rd_bits, rs1_bits);
                }
                "00011" =>{       // Store Word
                    println!("Store Word (SC.W) instruction decoded");
                    println!("Destination Register address: x{}", rd_bits);
                    println!("Register One address: x{}", rs1_bits);
                    println!("Register Two address: x{}", rs2_bits);
                    println!("SC.W x{}, x{}, x{}",rd_bits, rs2_bits, rs1_bits);
                    println!("--------------------------------");
                    return format!("SC.W x{}, x{}, x{}",rd_bits, rs2_bits, rs1_bits);
                }
                "00001" =>{       // Atomic Swap
                    println!("Atomic Swap (AMOSWAP.W) instruction decoded");
                    println!("Destination Register address: x{}", rd_bits);
                    println!("Register One address: x{}", rs1_bits);
                    println!("Register Two address: x{}", rs2_bits);
                    println!("AMOSWAP.W x{}, x{}, x{}",rd_bits, rs2_bits, rs1_bits);
                    println!("--------------------------------");
                    return format!("AMOSWAP.W x{}, x{}, x{}",rd_bits, rs2_bits, rs1_bits);
                }
                "00000" =>{       // Atomic Add
                    println!("Atomic Add (AMOADD.W) instruction decoded");
                    println!("Destination Register address: x{}", rd_bits);
                    println!("Register One address: x{}", rs1_bits);
                    println!("Register Two address: x{}", rs2_bits);
                    println!("AMOADD.W x{}, x{}, x{}",rd_bits, rs2_bits, rs1_bits);
                    println!("--------------------------------");
                    return format!("AMOADD.W x{}, x{}, x{}",rd_bits, rs2_bits, rs1_bits);
                }
                "00100" =>{       // Atomic XOR
                    println!("Atomic XOR (AMOXOR.W) instruction decoded");
                    println!("Destination Register address: x{}", rd_bits);
                    println!("Register One address: x{}", rs1_bits);
                    println!("Register Two address: x{}", rs2_bits);
                    println!("AMOXOR.W x{}, x{}, x{}",rd_bits, rs2_bits, rs1_bits);
                    println!("--------------------------------");
                    return format!("AMOXOR.W x{}, x{}, x{}",rd_bits, rs2_bits, rs1_bits);
                }
                "01100" =>{       // Atomic AND
                    println!("Atomic AND (AMOAND.W) instruction decoded");
                    println!("Destination Register address: x{}", rd_bits);
                    println!("Register One address: x{}", rs1_bits);
                    println!("Register Two address: x{}", rs2_bits);
                    println!("AMOAND.W x{}, x{}, x{}",rd_bits, rs2_bits, rs1_bits);
                    println!("--------------------------------");
                    return format!("AMOAND.W x{}, x{}, x{}",rd_bits, rs2_bits, rs1_bits);
                }
                "01000" =>{       // Atomic OR
                    println!("Atomic OR (AMOOR.W) instruction decoded");
                    println!("Destination Register address: x{}", rd_bits);
                    println!("Register One address: x{}", rs1_bits);
                    println!("Register Two address: x{}", rs2_bits);
                    println!("AMOOR.W x{}, x{}, x{}",rd_bits, rs2_bits, rs1_bits);
                    println!("--------------------------------");
                    return format!("AMOOR.W x{}, x{}, x{}",rd_bits, rs2_bits, rs1_bits);
                }
                "10000" =>{       // Atomic Minimum
                    println!("Atomic Minimum (AMOMIN.W) instruction decoded");
                    println!("Destination Register address: x{}", rd_bits);
                    println!("Register One address: x{}", rs1_bits);
                    println!("Register Two address: x{}", rs2_bits);
                    println!("AMOMIN.W x{}, x{}, x{}",rd_bits, rs2_bits, rs1_bits);
                    println!("--------------------------------");
                    return format!("AMOMIN.W x{}, x{}, x{}",rd_bits, rs2_bits, rs1_bits);
                }
                "10100" =>{       // Atomic Maximum
                    println!("Atomic Maximum (AMOMAX.W) instruction decoded");
                    println!("Destination Register address: x{}", rd_bits);
                    println!("Register One address: x{}", rs1_bits);
                    println!("Register Two address: x{}", rs2_bits);
                    println!("AMOMAX.W x{}, x{}, x{}",rd_bits, rs2_bits, rs1_bits);
                    println!("--------------------------------");
                    return format!("AMOMAX.W x{}, x{}, x{}",rd_bits, rs2_bits, rs1_bits);
                }
                "11000" =>{       // Atomic Unsigned Minimum
                    println!("Atomic Unsigned Minimum (AMOMINU.W) instruction decoded");
                    println!("Destination Register address: x{}", rd_bits);
                    println!("Register One address: x{}", rs1_bits);
                    println!("Register Two address: x{}", rs2_bits);
                    println!("AMOMINU.W x{}, x{}, x{}",rd_bits, rs2_bits, rs1_bits);
                    println!("--------------------------------");
                    return format!("AMOMINU.W x{}, x{}, x{}",rd_bits, rs2_bits, rs1_bits);
                }
                "11100" =>{       // Atomic Unsigned Maximum
                    println!("Atomic Unsigned Maximum (AMOMAXU.W) instruction decoded");
                    println!("Destination Register address: x{}", rd_bits);
                    println!("Register One address: x{}", rs1_bits);
                    println!("Register Two address: x{}", rs2_bits);
                    println!("AMOMAXU.W x{}, x{}, x{}",rd_bits, rs2_bits, rs1_bits);
                    println!("--------------------------------");
                    return format!("AMOMAXU.W x{}, x{}, x{}",rd_bits, rs2_bits, rs1_bits);
                }
                &_ => todo!()
            }            
        }

        "1000011" => {
            let rs3_slice = &instr[0..5];
            let rs3_slice_joined = rs3_slice.join("");
            let funct3_slice = &instr[5..7];
            let funct3_slice_joined = funct3_slice.join("");
            let rs2_slice = &instr[7..12];
            let rs2_slice_joined = rs2_slice.join("");
            let rs1_slice = &instr[12..17];
            let rs1_slice_joined = rs1_slice.join("");
            let rm_slice = &instr[17..20];
            let rm_slice_joined = rm_slice.join("");
            let rd_slice = &instr[20..25];
            let rd_slice_joined = rd_slice.join("");
            
            let rs1_bits = i32::from_str_radix(&rs1_slice_joined, 2).unwrap();
            let rs2_bits = i32::from_str_radix(&rs2_slice_joined, 2).unwrap();
            let rs3_bits = i32::from_str_radix(&rs3_slice_joined, 2).unwrap();
            let mut rd_bits = i32::from_str_radix(&rd_slice_joined, 2).unwrap();

            match funct3_slice_joined.as_str() {
                "00" => {
                    println!("Floating Point Addition (FMADD.S) instruction decoded");
                    println!("Destination Register address: f{}", rd_bits);
                    println!("Register One address: f{}", rs1_bits);
                    println!("Register Two address: f{}", rs2_bits);
                    println!("Register Three address: f{}", rs3_bits);
                    println!("FMADD.S f{}, f{}, f{}, f{}, {}", rd_bits, rs1_bits, rs2_bits, rs3_bits, rm_decoder(&rm_slice_joined));
                    println!("--------------------------------");
                    return format!("FMADD.S f{}, f{}, f{}, f{}, {}", rd_bits, rs1_bits, rs2_bits, rs3_bits, rm_decoder(&rm_slice_joined));
                }
                "01" => {
                    println!("Double Floating Point Addition (FMADD.D) instruction decoded");
                    println!("Destination Register address: f{}", rd_bits);
                    println!("Register One address: f{}", rs1_bits);
                    println!("Register Two address: f{}", rs2_bits);
                    println!("Register Three address: f{}", rs3_bits);
                    println!("FMADD.D f{}, f{}, f{}, f{}, {}", rd_bits, rs1_bits, rs2_bits, rs3_bits, rm_decoder(&rm_slice_joined));
                    println!("--------------------------------");
                    return format!("FMADD.D f{}, f{}, f{}, f{}, {}", rd_bits, rs1_bits, rs2_bits, rs3_bits, rm_decoder(&rm_slice_joined));
                }
                &_ => todo!()
            }
            
        }

        "1000111" => {
            let rs3_slice = &instr[0..5];
            let rs3_slice_joined = rs3_slice.join("");
            let funct3_slice = &instr[5..7];
            let funct3_slice_joined = funct3_slice.join("");
            let rs2_slice = &instr[7..12];
            let rs2_slice_joined = rs2_slice.join("");
            let rs1_slice = &instr[12..17];
            let rs1_slice_joined = rs1_slice.join("");
            let rm_slice = &instr[17..20];
            let rm_slice_joined = rm_slice.join("");
            let rd_slice = &instr[20..25];
            let rd_slice_joined = rd_slice.join("");
           
            let rs1_bits = i32::from_str_radix(&rs1_slice_joined, 2).unwrap();
            let rs2_bits = i32::from_str_radix(&rs2_slice_joined, 2).unwrap();
            let rs3_bits = i32::from_str_radix(&rs3_slice_joined, 2).unwrap();
            let mut rd_bits = i32::from_str_radix(&rd_slice_joined, 2).unwrap();

            match funct3_slice_joined.as_str() {
                "00" => {
                    println!("Floating Point Subtraction (FMSUB.S) instruction decoded");
                    println!("Destination Register address: f{}", rd_bits);
                    println!("Register One address: f{}", rs1_bits);
                    println!("Register Two address: f{}", rs2_bits);
                    println!("Register Three address: f{}", rs3_bits);
                    println!("FMSUB.S f{}, f{}, f{}, f{}, {}", rd_bits, rs1_bits, rs2_bits, rs3_bits, rm_decoder(&rm_slice_joined));
                    println!("--------------------------------");
                    return format!("FMSUB.S f{}, f{}, f{}, f{}, {}", rd_bits, rs1_bits, rs2_bits, rs3_bits, rm_decoder(&rm_slice_joined));
                }
                "01" => {
                    println!("Double Floating Point Subtraction (FMSUB.D) instruction decoded");
                    println!("Destination Register address: f{}", rd_bits);
                    println!("Register One address: f{}", rs1_bits);
                    println!("Register Two address: f{}", rs2_bits);
                    println!("Register Three address: f{}", rs3_bits);
                    println!("FMSUB.D f{}, f{}, f{}, f{}, {}", rd_bits, rs1_bits, rs2_bits, rs3_bits, rm_decoder(&rm_slice_joined));
                    println!("--------------------------------");
                    return format!("FMSUB.D f{}, f{}, f{}, f{}, {}", rd_bits, rs1_bits, rs2_bits, rs3_bits, rm_decoder(&rm_slice_joined));
                }
                &_ => todo!()
            }
        }

        "1001011" => {
            let rs3_slice = &instr[0..5];
            let rs3_slice_joined = rs3_slice.join("");
            let funct3_slice = &instr[5..7];
            let funct3_slice_joined = funct3_slice.join("");
            let rs2_slice = &instr[7..12];
            let rs2_slice_joined = rs2_slice.join("");
            let rs1_slice = &instr[12..17];
            let rs1_slice_joined = rs1_slice.join("");
            let rm_slice = &instr[17..20];
            let rm_slice_joined = rm_slice.join("");
            let rd_slice = &instr[20..25];
            let rd_slice_joined = rd_slice.join("");
           
            let rs1_bits = i32::from_str_radix(&rs1_slice_joined, 2).unwrap();
            let rs2_bits = i32::from_str_radix(&rs2_slice_joined, 2).unwrap();
            let rs3_bits = i32::from_str_radix(&rs3_slice_joined, 2).unwrap();
            let mut rd_bits = i32::from_str_radix(&rd_slice_joined, 2).unwrap();

            match funct3_slice_joined.as_str() {
                "00" => {
                    println!("Floating Point Negative Subtraction (FNMSUB.S) instruction decoded");
                    println!("Destination Register address: f{}", rd_bits);
                    println!("Register One address: f{}", rs1_bits);
                    println!("Register Two address: f{}", rs2_bits);
                    println!("Register Three address: f{}", rs3_bits);
                    println!("FNMSUB.S f{}, f{}, f{}, f{}, {}", rd_bits, rs1_bits, rs2_bits, rs3_bits, rm_decoder(&rm_slice_joined));
                    println!("--------------------------------");
                    return format!("FNMSUB.S f{}, f{}, f{}, f{}, {}", rd_bits, rs1_bits, rs2_bits, rs3_bits, rm_decoder(&rm_slice_joined));
                }
                "01" => {
                    println!("Double Floating Point Negative Subtraction (FNMSUB.D) instruction decoded");
                    println!("Destination Register address: f{}", rd_bits);
                    println!("Register One address: f{}", rs1_bits);
                    println!("Register Two address: f{}", rs2_bits);
                    println!("Register Three address: f{}", rs3_bits);
                    println!("FNMSUB.D f{}, f{}, f{}, f{}, {}", rd_bits, rs1_bits, rs2_bits, rs3_bits, rm_decoder(&rm_slice_joined));
                    println!("--------------------------------");
                    return format!("FNMSUB.D f{}, f{}, f{}, f{}, {}", rd_bits, rs1_bits, rs2_bits, rs3_bits, rm_decoder(&rm_slice_joined));
                }
                &_ => todo!()
            }

        }

        "1001111" => {
            let rs3_slice = &instr[0..5];
            let rs3_slice_joined = rs3_slice.join("");
            let funct3_slice = &instr[5..7];
            let funct3_slice_joined = funct3_slice.join("");
            let rs2_slice = &instr[7..12];
            let rs2_slice_joined = rs2_slice.join("");
            let rs1_slice = &instr[12..17];
            let rs1_slice_joined = rs1_slice.join("");
            let rm_slice = &instr[17..20];
            let rm_slice_joined = rm_slice.join("");
            let rd_slice = &instr[20..25];
            let rd_slice_joined = rd_slice.join("");
           
            let rs1_bits = i32::from_str_radix(&rs1_slice_joined, 2).unwrap();
            let rs2_bits = i32::from_str_radix(&rs2_slice_joined, 2).unwrap();
            let rs3_bits = i32::from_str_radix(&rs3_slice_joined, 2).unwrap();
            let mut rd_bits = i32::from_str_radix(&rd_slice_joined, 2).unwrap();

            match funct3_slice_joined.as_str() {
                "00" => {
                    println!("Floating Point Negative Addition (FNMADD.S) instruction decoded");
                    println!("Destination Register address: f{}", rd_bits);
                    println!("Register One address: f{}", rs1_bits);
                    println!("Register Two address: f{}", rs2_bits);
                    println!("Register Three address: f{}", rs3_bits);
                    println!("FNMADD.S f{}, f{}, f{}, f{}, {}", rd_bits, rs1_bits, rs2_bits, rs3_bits, rm_decoder(&rm_slice_joined));
                    println!("--------------------------------");
                    return format!("FNMADD.S f{}, f{}, f{}, f{}, {}", rd_bits, rs1_bits, rs2_bits, rs3_bits, rm_decoder(&rm_slice_joined));
                }
                "01" => {
                    println!("Double Floating Point Negative Addition (FNMADD.D) instruction decoded");
                    println!("Destination Register address: f{}", rd_bits);
                    println!("Register One address: f{}", rs1_bits);
                    println!("Register Two address: f{}", rs2_bits);
                    println!("Register Three address: f{}", rs3_bits);
                    println!("FNMADD.D f{}, f{}, f{}, f{}, {}", rd_bits, rs1_bits, rs2_bits, rs3_bits, rm_decoder(&rm_slice_joined));
                    println!("--------------------------------");
                    return format!("FNMADD.D f{}, f{}, f{}, f{}, {}", rd_bits, rs1_bits, rs2_bits, rs3_bits, rm_decoder(&rm_slice_joined));
                }
                &_ => todo!()
            }
        }

        "1010011" => {
            let rs3_slice = &instr[0..5];
            let rs3_slice_joined = rs3_slice.join("");
            let funct3_slice = &instr[5..7];
            let funct3_slice_joined = funct3_slice.join("");
            let rs2_slice = &instr[7..12];
            let rs2_slice_joined = rs2_slice.join("");
            let rs1_slice = &instr[12..17];
            let rs1_slice_joined = rs1_slice.join("");
            let rm_slice = &instr[17..20];
            let rm_slice_joined = rm_slice.join("");
            let rd_slice = &instr[20..25];
            let rd_slice_joined = rd_slice.join("");
           
            let rs1_bits = i32::from_str_radix(&rs1_slice_joined, 2).unwrap();
            let rs2_bits = i32::from_str_radix(&rs2_slice_joined, 2).unwrap();
            let mut rd_bits = i32::from_str_radix(&rd_slice_joined, 2).unwrap();

            match rs3_slice_joined.as_str() {
                "00000" => {
                    match funct3_slice_joined.as_str() {
                        "00" => {
                            println!("Floating Point Negative Addition (FADD.S) instruction decoded");
                            println!("Destination Register address: f{}", rd_bits);
                            println!("Register One address: f{}", rs1_bits);
                            println!("Register Two address: f{}", rs2_bits);
                            println!("FADD.S f{}, f{}, f{}, {}", rd_bits, rs1_bits, rs2_bits, rm_decoder(&rm_slice_joined));
                            println!("--------------------------------");
                            return format!("FADD.S f{}, f{}, f{}, {}", rd_bits, rs1_bits, rs2_bits, rm_decoder(&rm_slice_joined));
                        }
                        "01" => {
                            println!("Double Floating Point Negative Addition (FADD.D) instruction decoded");
                            println!("Destination Register address: f{}", rd_bits);
                            println!("Register One address: f{}", rs1_bits);
                            println!("Register Two address: f{}", rs2_bits);
                            println!("FADD.D f{}, f{}, f{}, {}", rd_bits, rs1_bits, rs2_bits, rm_decoder(&rm_slice_joined));
                            println!("--------------------------------");
                            return format!("FADD.D f{}, f{}, f{}, {}", rd_bits, rs1_bits, rs2_bits, rm_decoder(&rm_slice_joined));
                        }
                        &_ => todo!()
                    }
                }
                "00001" => {
                    match funct3_slice_joined.as_str() {
                        "00" => {
                            println!("Floating Point Subtraction (FSUB.S) instruction decoded");
                            println!("Destination Register address: f{}", rd_bits);
                            println!("Register One address: f{}", rs1_bits);
                            println!("Register Two address: f{}", rs2_bits);
                            println!("FSUB.S f{}, f{}, f{}, {}", rd_bits, rs1_bits, rs2_bits, rm_decoder(&rm_slice_joined));
                            println!("--------------------------------");
                            return format!("FSUB.S f{}, f{}, f{}, {}", rd_bits, rs1_bits, rs2_bits, rm_decoder(&rm_slice_joined));  
                        }
                        "01" => {
                            println!("Double Floating Point Subtraction (FSUB.D) instruction decoded");
                            println!("Destination Register address: f{}", rd_bits);
                            println!("Register One address: f{}", rs1_bits);
                            println!("Register Two address: f{}", rs2_bits);
                            println!("FSUB.D f{}, f{}, f{}, {}", rd_bits, rs1_bits, rs2_bits, rm_decoder(&rm_slice_joined));
                            println!("--------------------------------");
                            return format!("FSUB.D f{}, f{}, f{}, {}", rd_bits, rs1_bits, rs2_bits, rm_decoder(&rm_slice_joined));  
                        }
                        &_ => todo!()
                    }
                }
                "00010" => {
                   match funct3_slice_joined.as_str() {
                        "00" => {
                            println!("Floating Point Multiplication (FMUL.S) instruction decoded");
                            println!("Destination Register address: f{}", rd_bits);
                            println!("Register One address: f{}", rs1_bits);
                            println!("Register Two address: f{}", rs2_bits);
                            println!("FMUL.S f{}, f{}, f{}, {}", rd_bits, rs1_bits, rs2_bits, rm_decoder(&rm_slice_joined));
                            println!("--------------------------------");
                            return format!("FMUL.S f{}, f{}, f{}, {}", rd_bits, rs1_bits, rs2_bits, rm_decoder(&rm_slice_joined));
                        }
                        "01" => {
                            println!("Double Floating Point Multiplication (FMUL.D) instruction decoded");
                            println!("Destination Register address: f{}", rd_bits);
                            println!("Register One address: f{}", rs1_bits);
                            println!("Register Two address: f{}", rs2_bits);
                            println!("FMUL.D f{}, f{}, f{}, {}", rd_bits, rs1_bits, rs2_bits, rm_decoder(&rm_slice_joined));
                            println!("--------------------------------");
                            return format!("FMUL.D f{}, f{}, f{}, {}", rd_bits, rs1_bits, rs2_bits, rm_decoder(&rm_slice_joined));
                        }
                        &_ => todo!()
                   }
                }
                "00011" => {
                    match funct3_slice_joined.as_str() {
                        "00" => {
                            println!("Floating Point Division (FDIV.S) instruction decoded");
                            println!("Destination Register address: f{}", rd_bits);
                            println!("Register One address: f{}", rs1_bits);
                            println!("Register Two address: f{}", rs2_bits);
                            println!("FDIV.S f{}, f{}, f{}, {}", rd_bits, rs1_bits, rs2_bits, rm_decoder(&rm_slice_joined));
                            println!("--------------------------------");
                            return format!("FDIV.S f{}, f{}, f{}, {}", rd_bits, rs1_bits, rs2_bits, rm_decoder(&rm_slice_joined));
                        }
                        "01" => {
                            println!("Double Floating Point Division (FDIV.D) instruction decoded");
                            println!("Destination Register address: f{}", rd_bits);
                            println!("Register One address: f{}", rs1_bits);
                            println!("Register Two address: f{}", rs2_bits); 
                            println!("FDIV.D f{}, f{}, f{}, {}", rd_bits, rs1_bits, rs2_bits, rm_decoder(&rm_slice_joined));
                            println!("--------------------------------");
                            return format!("FDIV.D f{}, f{}, f{}, {}", rd_bits, rs1_bits, rs2_bits, rm_decoder(&rm_slice_joined));
                        }
                        &_ => todo!()
                    }
                }
                "01011" => {
                    match funct3_slice_joined.as_str() {
                        "00" => {
                            println!("Floating Point Square Root (FSQRT.S) instruction decoded");
                            println!("Destination Register address: f{}", rd_bits);
                            println!("Register One address: f{}", rs1_bits);
                            println!("FSQRT.S f{}, f{}, {}", rd_bits, rs1_bits, rm_decoder(&rm_slice_joined));
                            println!("--------------------------------");
                            return format!("FSQRT.S f{}, f{}, {}", rd_bits, rs1_bits, rm_decoder(&rm_slice_joined));
                        }
                        "01" => {
                            println!("Double Floating Point Square Root (FSQRT.D) instruction decoded");
                            println!("Destination Register address: f{}", rd_bits);
                            println!("Register One address: f{}", rs1_bits);
                            println!("FSQRT.D f{}, f{}, {}", rd_bits, rs1_bits, rm_decoder(&rm_slice_joined));
                            println!("--------------------------------");
                            return format!("FSQRT.D f{}, f{}, {}", rd_bits, rs1_bits, rm_decoder(&rm_slice_joined));
                        }
                        &_ => todo!()
                    }
                }
                "01000" => {
                    match rs2_slice_joined.as_str() {
                        "00000" => {
                            println!("Double Conversion (FCVT.D.S) instruction decoded");
                            println!("Destination Register address: f{}", rd_bits);
                            println!("Register One address: f{}", rs1_bits);
                            println!("FCVT.D.S f{}, f{}, {}", rd_bits, rs1_bits, rm_decoder(&rm_slice_joined));
                            println!("--------------------------------");
                            return format!("FCVT.D.S f{}, f{}, {}", rd_bits, rs1_bits, rm_decoder(&rm_slice_joined));
                        }
                        "00001" => {
                            println!("Double Conversion (FCVT.S.D) instruction decoded");
                            println!("Destination Register address: f{}", rd_bits);
                            println!("Register One address: f{}", rs1_bits);
                            println!("FCVT.S.D f{}, f{}, {}", rd_bits, rs1_bits, rm_decoder(&rm_slice_joined));
                            println!("--------------------------------");
                            return format!("FCVT.S.D f{}, f{}, {}", rd_bits, rs1_bits, rm_decoder(&rm_slice_joined));
                        }
                        &_ => todo!()
                    }
                }
                "00100" => {
                    match rm_slice_joined.as_str() {
                            "000" => {
                                match funct3_slice_joined.as_str() {
                                    "00" => {
                                        println!("Floating Point Sign Injection (FSGNJ.S) instruction decoded");
                                        println!("Destination Register address: f{}", rd_bits);
                                        println!("Register One address: f{}", rs1_bits);
                                        println!("Register Two address: f{}", rs2_bits);
                                        println!("FSGNJ.S f{}, f{}, f{}", rd_bits, rs1_bits, rs2_bits); 
                                        println!("--------------------------------");
                                        return format!("FSGNJ.S f{}, f{}, f{}", rd_bits, rs1_bits, rs2_bits);
                                    }
                                    "01" => {
                                        println!("Double Sign Injection (FSGNJN.D) instruction decoded");
                                        println!("Destination Register address: f{}", rd_bits);
                                        println!("Register One address: f{}", rs1_bits);
                                        println!("Register Two address: f{}", rs2_bits);
                                        println!("FSGNJ.D f{}, f{}, f{}", rd_bits, rs1_bits, rs2_bits); 
                                        println!("--------------------------------");
                                        return format!("FSGNJ.D f{}, f{}, f{}", rd_bits, rs1_bits, rs2_bits);
                                    }
                                    &_ => todo!()
                                }
                            }
                            "001" => {
                                match funct3_slice_joined.as_str() {
                                    "00" => {
                                        println!("Floating Point Sign Injection (FSGNJN.S) instruction decoded");
                                        println!("Destination Register address: f{}", rd_bits);
                                        println!("Register One address: f{}", rs1_bits);
                                        println!("Register Two address: f{}", rs2_bits);
                                        println!("FSGNJN.S f{}, f{}, f{}", rd_bits, rs1_bits, rs2_bits); 
                                        println!("--------------------------------");
                                        return format!("FSGNJN.S f{}, f{}, f{}", rd_bits, rs1_bits, rs2_bits);
                                    }
                                    "01" => {
                                        println!("Double Sign Injection (FSGNJN.D) instruction decoded");
                                        println!("Destination Register address: f{}", rd_bits);
                                        println!("Register One address: f{}", rs1_bits);
                                        println!("Register Two address: f{}", rs2_bits);
                                        println!("FSGNJN.D f{}, f{}, f{}", rd_bits, rs1_bits, rs2_bits); 
                                        println!("--------------------------------");
                                        return format!("FSGNJN.D f{}, f{}, f{}", rd_bits, rs1_bits, rs2_bits);
                                    }
                                    &_ => todo!()
                                }
                            }
                            "010" => {
                                match funct3_slice_joined.as_str() {
                                    "00" => {
                                        println!("Floating Point Sign Injection (FSGNJX.S) instruction decoded");
                                        println!("Destination Register address: f{}", rd_bits);
                                        println!("Register One address: f{}", rs1_bits);
                                        println!("Register Two address: f{}", rs2_bits);
                                        println!("FSGNJX.S f{}, f{}, f{}", rd_bits, rs1_bits, rs2_bits); 
                                        println!("--------------------------------");
                                        return format!("FSGNJX.S f{}, f{}, f{}", rd_bits, rs1_bits, rs2_bits);
                                    }
                                    "01" => {
                                        println!("Double Sign Injection (FSGNJX.D) instruction decoded");
                                        println!("Destination Register address: f{}", rd_bits);
                                        println!("Register One address: f{}", rs1_bits);
                                        println!("Register Two address: f{}", rs2_bits);
                                        println!("FSGNJX.D f{}, f{}, f{}", rd_bits, rs1_bits, rs2_bits); 
                                        println!("--------------------------------");
                                        return format!("FSGNJX.D f{}, f{}, f{}", rd_bits, rs1_bits, rs2_bits);
                                    }
                                    &_ => todo!()
                                }
                            }
                            &_ => todo!()
                    }
                }
                "00101" => {
                    match rm_slice_joined.as_str() {
                        "000" => {
                            match funct3_slice_joined.as_str() {
                                "00" => {
                                    println!("Floating Point Minimum (FMIN.S) instruction decoded");
                                    println!("Destination Register address: f{}", rd_bits);
                                    println!("Register One address: f{}", rs1_bits);
                                    println!("Register Two address: f{}", rs2_bits);
                                    println!("FMIN.S f{}, f{}, f{}", rd_bits, rs1_bits, rs2_bits);
                                    println!("--------------------------------");
                                    return format!("FMIN.S f{}, f{}, f{}", rd_bits, rs1_bits, rs2_bits);
                                }
                                "01" => {
                                    println!("Double Floating Point Minimum (FMIN.D) instruction decoded");
                                    println!("Destination Register address: f{}", rd_bits);
                                    println!("Register One address: f{}", rs1_bits);
                                    println!("Register Two address: f{}", rs2_bits);
                                    println!("FMIN.D f{}, f{}, f{}", rd_bits, rs1_bits, rs2_bits);
                                    println!("--------------------------------");
                                    return format!("FMIN.D f{}, f{}, f{}", rd_bits, rs1_bits, rs2_bits);
                                }
                                &_ => todo!()
                            }
                        }
                        "001" => {
                            match funct3_slice_joined.as_str() {
                                "00" => {
                                    println!("Floating Point Maximum (FMAX.S) instruction decoded");
                                    println!("Destination Register address: f{}", rd_bits);
                                    println!("Register One address: f{}", rs1_bits);
                                    println!("Register Two address: f{}", rs2_bits);
                                    println!("FMAX.S f{}, f{}, f{}", rd_bits, rs1_bits, rs2_bits);
                                    println!("--------------------------------");
                                    return format!("FMAX.S f{}, f{}, f{}", rd_bits, rs1_bits, rs2_bits);
                                }
                                "01" => {
                                    println!("Double Floating Point Maximum (FMAX.D) instruction decoded");
                                    println!("Destination Register address: f{}", rd_bits);
                                    println!("Register One address: f{}", rs1_bits);
                                    println!("Register Two address: f{}", rs2_bits);
                                    println!("FMAX.D f{}, f{}, f{}", rd_bits, rs1_bits, rs2_bits);
                                    println!("--------------------------------");
                                    return format!("FMAX.D f{}, f{}, f{}", rd_bits, rs1_bits, rs2_bits);
                                }
                                &_ => todo!()
                            }
                        }
                        &_ => todo!()
                    }
                }
                "11000" => {
                    match rs2_slice_joined.as_str() {
                        "00000" => {
                            match funct3_slice_joined.as_str() {
                                "00" => {
                                    println!("Floating Point Conversion to Integer (FCVT.W.S) instruction decoded");
                                    println!("Destination Register address: x{}", rd_bits);
                                    println!("Register One address: f{}", rs1_bits);
                                    println!("FCVT.W.S x{}, f{}, {}", rd_bits, rs1_bits, rm_decoder(&rm_slice_joined));
                                    println!("--------------------------------");
                                    return format!("FCVT.W.S x{}, f{}, {}", rd_bits, rs1_bits, rm_decoder(&rm_slice_joined));
                                }
                                "01" => {
                                    println!("Double Conversion to Integer (FCVT.W.D) instruction decoded");
                                    println!("Destination Register address: x{}", rd_bits);
                                    println!("Register One address: f{}", rs1_bits);
                                    println!("FCVT.W.D x{}, f{}, {}", rd_bits, rs1_bits, rm_decoder(&rm_slice_joined));
                                    println!("--------------------------------");
                                    return format!("FCVT.W.D x{}, f{}, {}", rd_bits, rs1_bits, rm_decoder(&rm_slice_joined));
                                }
                                &_ => todo!()
                            }
                        }
                        "00001" => {
                            match funct3_slice_joined.as_str() {
                                "00" => {
                                    println!("Floating Point Conversion to Integer (FCVT.WU.S) instruction decoded");
                                    println!("Destination Register address: x{}", rd_bits);
                                    println!("Register One address: f{}", rs1_bits);
                                    println!("FCVT.WU.S x{}, f{}, {}", rd_bits, rs1_bits, rm_decoder(&rm_slice_joined));
                                    println!("--------------------------------");
                                    return format!("FCVT.WU.S x{}, f{}, {}", rd_bits, rs1_bits, rm_decoder(&rm_slice_joined));
                                }
                                "01" => {
                                    println!("Double Conversion to Integer (FCVT.WU.D) instruction decoded");
                                    println!("Destination Register address: x{}", rd_bits);
                                    println!("Register One address: f{}", rs1_bits);
                                    println!("FCVT.WU.D x{}, f{}, {}", rd_bits, rs1_bits, rm_decoder(&rm_slice_joined));
                                    println!("--------------------------------");
                                    return format!("FCVT.WU.D x{}, f{}, {}", rd_bits, rs1_bits, rm_decoder(&rm_slice_joined));
                                }
                                &_ => todo!()
                            }
                        }
                        &_ => todo!()
                    }
                }
                "11100" => {
                    match rm_slice_joined.as_str() {
                    "000" => {
                        println!("Floating Point Conversion to Integer (FMV.X.W) instruction decoded");
                        println!("Destination Register address: x{}", rd_bits);
                        println!("Register One address: f{}", rs1_bits);
                        println!("FMV.X.W x{}, f{}", rd_bits, rs1_bits);
                        println!("--------------------------------");
                        return format!("FMV.X.W x{}, f{}", rd_bits, rs1_bits);
                    }
                    "001" => {
                        match funct3_slice_joined.as_str() {
                            "00" => {
                                println!("Floating Point Class (FCLASS.S) instruction decoded");
                                println!("Destination Register address: x{}", rd_bits);
                                println!("Register One address: f{}", rs1_bits);
                                println!("FCLASS.S x{}, f{}", rd_bits, rs1_bits);
                                println!("--------------------------------");
                                return format!("FCLASS.S x{}, f{}", rd_bits, rs1_bits);
                            }
                            "01" => {
                                println!("Double Class (FCLASS.D) instruction decoded");
                                println!("Destination Register address: x{}", rd_bits);
                                println!("Register One address: f{}", rs1_bits);
                                println!("FCLASS.D x{}, f{}", rd_bits, rs1_bits);
                                println!("--------------------------------");
                                return format!("FCLASS.D x{}, f{}", rd_bits, rs1_bits);
                            }
                            &_ => todo!()
                        }
                    }
                    &_ => todo!()
                    }
                }
                "10100" => {
                    match rm_slice_joined.as_str() {
                        "000" => {
                            match funct3_slice_joined.as_str() {
                                "00" => {
                                    println!("Floating Point Conversion to Integer (FLE.S) instruction decoded");
                                    println!("Destination Register address: x{}", rd_bits);
                                    println!("Register One address: f{}", rs1_bits);
                                    println!("Register Two address: f{}", rs2_bits);
                                    println!("FLE.S x{}, f{}, f{}", rd_bits, rs1_bits, rs2_bits);
                                    println!("--------------------------------");
                                    return format!("FLE.S x{}, f{}, f{}", rd_bits, rs1_bits, rs2_bits);
                                }
                                "01" => {
                                    println!("Double Conversion to Integer (FLE.D) instruction decoded");
                                    println!("Destination Register address: x{}", rd_bits);
                                    println!("Register One address: f{}", rs1_bits);
                                    println!("Register Two address: f{}", rs2_bits);
                                    println!("FLE.D x{}, f{}, f{}", rd_bits, rs1_bits, rs2_bits);
                                    println!("--------------------------------");
                                    return format!("FLE.D x{}, f{}, f{}", rd_bits, rs1_bits, rs2_bits);
                                }
                                &_ => todo!()
                            }
                        }
                        "010" => {
                            match funct3_slice_joined.as_str() {
                                "00" => {
                                    println!("Floating Point Conversion to Integer (FEQ.S) instruction decoded");
                                    println!("Destination Register address: x{}", rd_bits);
                                    println!("Register One address: f{}", rs1_bits);
                                    println!("Register Two address: f{}", rs2_bits);
                                    println!("FEQ.S x{}, f{}, f{}", rd_bits, rs1_bits, rs2_bits);
                                    println!("--------------------------------");
                                    return format!("FEQ.S x{}, f{}, f{}", rd_bits, rs1_bits, rs2_bits);
                                }
                                "01" => {
                                    println!("Double Conversion to Integer (FEQ.D) instruction decoded");
                                    println!("Destination Register address: x{}", rd_bits);
                                    println!("Register One address: f{}", rs1_bits);
                                    println!("Register Two address: f{}", rs2_bits);
                                    println!("FEQ.D x{}, f{}, f{}", rd_bits, rs1_bits, rs2_bits);
                                    println!("--------------------------------");
                                    return format!("FEQ.D x{}, f{}, f{}", rd_bits, rs1_bits, rs2_bits);
                                }
                                &_ => todo!()
                            }
                        }
                        "001" => {
                            match funct3_slice_joined.as_str() {
                                "00" => {
                                    println!("Floating Point Conversion to Integer (FLT.S) instruction decoded");
                                    println!("Destination Register address: x{}", rd_bits);
                                    println!("Register One address: f{}", rs1_bits);
                                    println!("Register Two address: f{}", rs2_bits);
                                    println!("FLT.S x{}, f{}, f{}", rd_bits, rs1_bits, rs2_bits);
                                    println!("--------------------------------");
                                    return format!("FLT.S x{}, f{}, f{}", rd_bits, rs1_bits, rs2_bits);
                                }
                                "01" => {
                                    println!("Double Conversion to Integer (FLT.D) instruction decoded");
                                    println!("Destination Register address: x{}", rd_bits);
                                    println!("Register One address: f{}", rs1_bits);
                                    println!("Register Two address: f{}", rs2_bits);
                                    println!("FLT.D x{}, f{}, f{}", rd_bits, rs1_bits, rs2_bits);
                                    println!("--------------------------------");
                                    return format!("FLT.D x{}, f{}, f{}", rd_bits, rs1_bits, rs2_bits);
                                }
                                &_ => todo!()
                            }
                        }
                        &_ => todo!()
                    }
                }
                "11010" => {
                    match rs2_slice_joined.as_str() {
                        "00000" => {
                            match funct3_slice_joined.as_str() {
                                "00" => {
                                    println!("Floating Point Conversion to Integer (FCVT.S.W) instruction decoded");
                                    println!("Destination Register address: f{}", rd_bits);
                                    println!("Register One address: x{}", rs1_bits);
                                    println!("FCVT.S.W f{}, x{}, {}", rd_bits, rs1_bits, rm_decoder(&rm_slice_joined));
                                    println!("--------------------------------");
                                    return format!("FCVT.S.W f{}, x{}, {}", rd_bits, rs1_bits, rm_decoder(&rm_slice_joined));
                                }
                                "01" => {
                                    println!("Double Conversion to Integer (FCVT.D.W) instruction decoded");
                                    println!("Destination Register address: f{}", rd_bits);
                                    println!("Register One address: x{}", rs1_bits);
                                    println!("FCVT.D.W f{}, x{}, {}", rd_bits, rs1_bits, rm_decoder(&rm_slice_joined));
                                    println!("--------------------------------");
                                    return format!("FCVT.D.W f{}, x{}, {}", rd_bits, rs1_bits, rm_decoder(&rm_slice_joined));
                                }
                                &_ => todo!()
                            }
                        }
                        "00001" => {
                            match funct3_slice_joined.as_str() {
                                "00" => {
                                    println!("Floating Point Conversion to Integer (FCVT.S.WU) instruction decoded");
                                    println!("Destination Register address: f{}", rd_bits);
                                    println!("Register One address: x{}", rs1_bits);
                                    println!("FCVT.S.WU f{}, x{}, {}", rd_bits, rs1_bits, rm_decoder(&rm_slice_joined));
                                    println!("--------------------------------");
                                    return format!("FCVT.S.WU f{}, x{}, {}", rd_bits, rs1_bits, rm_decoder(&rm_slice_joined));
                                }
                                "01" => {
                                    println!("Double Conversion to Integer (FCVT.D.WU) instruction decoded");
                                    println!("Destination Register address: f{}", rd_bits);
                                    println!("Register One address: x{}", rs1_bits);
                                    println!("FCVT.D.WU f{}, x{}, {}", rd_bits, rs1_bits, rm_decoder(&rm_slice_joined));
                                    println!("--------------------------------");
                                    return format!("FCVT.D.WU f{}, x{}, {}", rd_bits, rs1_bits, rm_decoder(&rm_slice_joined));
                                }
                                &_ => todo!()
                            }
                        }
                        &_ => todo!()
                    }
                }
                "11110" => {
                    println!("Floating Point Conversion to Integer (FMV.W.X) instruction decoded");
                    println!("Destination Register address: f{}", rd_bits);
                    println!("Register One address: x{}", rs1_bits);
                    println!("FMV.W.X f{}, x{}", rd_bits, rs1_bits);
                    println!("--------------------------------");
                    return format!("FMV.W.X f{}, x{}", rd_bits, rs1_bits);
                }
                &_ => todo!()
            }

        }
        "0000111" => {
            let imm_slice = &instr[0..12];
            let imm_slice_joined = imm_slice.join("");
            let rs1_slice = &instr[12..17];
            let rs1_slice_joined = rs1_slice.join("");
            let rd_slice = &instr[20..25];
            let rd_slice_joined = rd_slice.join("");
            let rm_slice = &instr[17..20];
            let rm_slice_joined = rm_slice.join("");
            
            let mut rd_bits = i32::from_str_radix(&rd_slice_joined, 2).unwrap();
            let rs1_bits = i32::from_str_radix(&rs1_slice_joined, 2).unwrap();
            let imm_bits = i32::from_str_radix(&imm_slice_joined, 2).unwrap();


            match rm_slice_joined.as_str() {
                "010" => {

                    println!("Floating Point Load Word (FLW) instruction decoded");
                    println!("Destination Register address: f{}", rd_bits);
                    println!("Register One address: x{}", rs1_bits);
                    println!("FLW f{}, {}(x{})", rd_bits, imm_bits, rs1_bits);
                    println!("--------------------------------");
                    return format!("FLW f{}, {}(x{})", rd_bits, imm_bits, rs1_bits);
                }
                "011" =>{
                    println!("Load Double (FLD) instruction decoded");
                    println!("Destination Register address: f{}", rd_bits);
                    println!("Register One address: x{}", rs1_bits);
                    println!("Immendiate: {}", imm_bits);
                    println!("FLD f{}, {}(x{})", rd_bits, imm_bits, rs1_bits);
                    println!("--------------------------------");
                    return format!("FLD f{}, {}(x{})", rd_bits, imm_bits, rs1_bits);
                }
                &_ => todo!()
            }

        }
        "0100111" => {
            let rs2_slice = &instr[7..12];
            let rs2_slice_joined = rs2_slice.join("");
            let rs1_slice = &instr[12..17];
            let rs1_slice_joined = rs1_slice.join("");
            let imm1_slice = &instr[0..7];
            let imm1_slice_joined = imm1_slice.join("");
            let imm2_slice = &instr[20..25];
            let imm2_slice_joined = imm2_slice.join("");
            let imm_slice_joined = imm1_slice_joined + &imm2_slice_joined;
            let rm_slice = &instr[17..20];
            let rm_slice_joined = rm_slice.join("");
            
            let mut imm_bits = i32::from_str_radix(&imm_slice_joined, 2).unwrap();
            let rs1_bits = i32::from_str_radix(&rs1_slice_joined, 2).unwrap();
            let rs2_bits = i32::from_str_radix(&rs2_slice_joined, 2).unwrap();

            match rm_slice_joined.as_str() {
                "010" => {
                    println!("Floating Point Store Word (FSW) instruction decoded");
                    println!("Register One address: f{}", rs1_bits);
                    println!("Register Two address: x{}", rs2_bits);
                    println!("Immediates: {}", imm_bits);
                    println!("FSW f{}, {}(x{})", rs2_bits, imm_bits, rs1_bits);
                    println!("--------------------------------");
                    return format!("FSW f{}, {}(x{})", rs2_bits, imm_bits, rs1_bits);
                }
                "011" => {
                    println!("Store Double (FSD) instruction decoded");
                    println!("Register One address: f{}", rs1_bits);
                    println!("Register Two address: x{}", rs2_bits);
                    println!("Immediates: {}", imm_bits);
                    println!("FSD f{}, {}(x{})", rs2_bits, imm_bits, rs1_bits);
                    println!("--------------------------------");
                    return format!("FSD f{}, {}(x{})", rs2_bits, imm_bits, rs1_bits);
                }
                &_ => todo!()
            }
        }
        // "0000111" => {
        //     let imm_slice = &instr[0..12];
        //     let imm_slice_joined = imm_slice.join("");
        //     let rs1_slice = &instr[12..17];
        //     let rs1_slice_joined = rs1_slice.join("");
        //     let rd_slice = &instr[20..25];
        //     let rd_slice_joined = rd_slice.join("");

        //     let mut rd_bits = i32::from_str_radix(&rd_slice_joined, 2).unwrap();
        //     let rs1_bits = i32::from_str_radix(&rs1_slice_joined, 2).unwrap();
        //     let imm_bits = i32::from_str_radix(&imm_slice_joined, 2).unwrap();

            
        // }

        default => {
            panic!("Opcode not found!");
        }
    &_ => todo!()
    }
}
