use rv_decoder::convert_binary_string_to_vector;
use rv_decoder::instruction_decoder;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn load_instructions() {
        //Load Byte
        let binary_instruction = "00000000010000000000001000000011";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "LB x4, 4(x0)";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // Load Halfword
        let binary_instruction = "00000000010000000001000100000011";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "LH x2, 4(x0)";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // Load Word
        let binary_instruction = "00000000010000000010000100000011";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "LW x2, 4(x0)";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // Load Byte Unsigned
        let binary_instruction = "00000000010000000100000100000011";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "LBU x2, 4(x0)";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // Load Halfword Unsigned
        let binary_instruction = "00000000010000000101000100000011";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "LHU x2, 4(x0)";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);
    }

    #[test]
    fn store_instructions() {
        // Store Byte
        let binary_instruction = "00000000111000010000010000100011";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "SB x14, 8(x2)";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // Store Halfword
        let binary_instruction = "00000000111000010001010000100011";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "SH x14, 8(x2)";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // Store Word
        let binary_instruction = "00000000111000010010010000100011";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "SW x14, 8(x2)";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);
    }

    #[test]
    fn immediate_instructions() {
        // Add immediate instruction
        let binary_instruction = "11111100111000001000011110010011";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "ADDI x15, x1, -50";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // Set Less Than Immediate
        let binary_instruction = "11111100111000001010011110010011";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "SLTI x15, x1, -50";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // Set Less Than Immediate Unsigned
        let binary_instruction = "11111100111000001011011110010011";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "SLTIU x15, x1, -50";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // XOR Immediate
        let binary_instruction = "11111100111000001100011110010011";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "XORI x15, x1, -50";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // OR Immediate
        let binary_instruction = "11111100111000001110011110010011";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "ORI x15, x1, -50";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // AND Immediate
        let binary_instruction = "11111100111000001111011110010011";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "ANDI x15, x1, -50";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);
    }

    #[test]
    fn lui_instruction() {
        // Load Upper Immediate
        let binary_instruction = "10000111011001010100000110110111";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "LUI x3, -493996";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);
    }

    #[test]
    fn auipc_instruction() {
        // Add Upper Immediate to PC
        let binary_instruction = "10000111011001010100000110010111";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "AUIPC x3, -493996";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);
    }

    #[test]
    fn jump_instruction() {
        let binary_instruction = "00001000010000000000000101101111";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "JAL x2, 132";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);
    }

    #[test]
    fn jump_to_register_instruction() {
        let binary_instruction = "00001000010000000000000101100111";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "JALR x2, x0, 132";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);
    }

    #[test]
    fn arithmetic_instructions() {
        // Add
        let binary_instruction = "00000000001100010000000010110011";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "ADD x1, x2, x3";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // Sub
        let binary_instruction = "01000000001100010000000010110011";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "SUB x1, x2, x3";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // Shift Left Logical
        let binary_instruction = "00000000001100010001000010110011";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "SLL x1, x2, x3";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // Set Less Than
        let binary_instruction = "00000000001100010010000010110011";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "SLT x1, x2, x3";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // Set less than unsigned
        let binary_instruction = "00000000001100010011000010110011";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "SLTU x1, x2, x3";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // XOR
        let binary_instruction = "00000000001100010100000010110011";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "XOR x1, x2, x3";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // Shift Right Logical
        let binary_instruction = "00000000001100010101000010110011";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "SRL x1, x2, x3";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // Shift Right Arithmetic
        let binary_instruction = "01000000001100010101000010110011";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "SRA x1, x2, x3";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // OR
        let binary_instruction = "00000000001100010110000010110011";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "OR x1, x2, x3";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // And
        let binary_instruction = "00000000001100010111000010110011";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "AND x1, x2, x3";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);
    }

    #[test]
    fn branch_instructions() {
        // BEQ
        let binary_instruction = "00110000101010011000101001100011";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "BEQ x19, x10, 788";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // BNE
        let binary_instruction = "10110000101010011001101001100011";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "BNE x19, x10, -3308";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);
         
        // BLT 
        let binary_instruction = "10110000101010011100101001100011";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "BLT x19, x10, -3308";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // BGE
        let binary_instruction = "10110000101010011101101001100011";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "BGE x19, x10, -3308";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // BLTU
        let binary_instruction = "10110000101010011110101001100011";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "BLTU x19, x10, -3308";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // BGEU
        let binary_instruction = "10110000101010011111101001100011";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "BGEU x19, x10, -3308";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);
    }

    #[test]
    fn multiplication_extension() {
        // Multiply
        let binary_instruction = "00000010001100010000000010110011";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "MUL x1, x2, x3";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // Multiply High Signed
        let binary_instruction = "00000010001100010001000010110011";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "MULH x1, x2, x3";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // Multiply signed and unsigned
        let binary_instruction = "00000010001100010010000010110011";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "MULHSU x1, x2, x3";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // Multiply unsigned
        let binary_instruction = "00000010001100010011000010110011";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "MULHU x1, x2, x3";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // Divide signed
        let binary_instruction = "00000010001100010100000010110011";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "DIV x1, x2, x3";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // Divide unsigned
        let binary_instruction = "00000010001100010101000010110011";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "DIVU x1, x2, x3";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // Remainder signed
        let binary_instruction = "00000010001100010110000010110011";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "REM x1, x2, x3";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // Remainder unsigned
        let binary_instruction = "00000010001100010111000010110011";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "REMU x1, x2, x3";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);
    }

    #[test]
    fn atomic_instructions() {
        // Load Reserved
        let binary_instruction = "00010000000000011010000010101111";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "LR.W x1, x3";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // Store if reserved
        let binary_instruction = "00011000000100011010000010101111";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "SC.W x1, x1, x3";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // Amoswap
        let binary_instruction = "00001000000100011010000010101111";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "AMOSWAP.W x1, x1, x3";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // Amoadd
        let binary_instruction = "00000000000100011010000010101111";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "AMOADD.W x1, x1, x3";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // Amoxor
        let binary_instruction = "00100000000100011010000010101111";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "AMOXOR.W x1, x1, x3";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // Amoand
        let binary_instruction = "01100000000100011010000010101111";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "AMOAND.W x1, x1, x3";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // Amoor
        let binary_instruction = "01000000000100011010000010101111";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "AMOOR.W x1, x1, x3";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // Amomin
        let binary_instruction = "10000000000100011010000010101111";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "AMOMIN.W x1, x1, x3";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // Amomax
        let binary_instruction = "10100000000100011010000010101111";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "AMOMAX.W x1, x1, x3";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // Amomin unsigned
        let binary_instruction = "11000000000100011010000010101111";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "AMOMINU.W x1, x1, x3";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // Amomax unsigned
        let binary_instruction = "11100000000100011010000010101111";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "AMOMAXU.W x1, x1, x3";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);
    }

    // #[test]
    // fn floating_point_instructions() {
    //     // Fused multiply addition
    //     let binary_instruction = "00100001000001000001000111000011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FMADD.S f3, f8, f16, f4, rtz";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // Fused multiply subtraction
    //     let binary_instruction = "00100001000001000001000111000111";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FMSUB.S f3, f8, f16, f4, rtz";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // Fused negative multiply addition
    //     let binary_instruction = "00100001000001000001000111001111";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FNMADD.S f3, f8, f16, f4, rtz";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // Fused negative multiply subtraction
    //     let binary_instruction = "00100001000001000001000111001011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FNMSUB.S f3, f8, f16, f4, rtz";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // Floating point addition
    //     let binary_instruction = "00000001000001000001000111010011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FADD.S f3, f8, f16, rtz";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // Floating point subtraction
    //     let binary_instruction = "00001001000001000001000111010011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FSUB.S f3, f8, f16, rtz";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // Floating point multiplication
    //     let binary_instruction = "00010001000001000001000111010011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FMUL.S f3, f8, f16, rtz";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // Floating point division
    //     let binary_instruction = "00011001000001000001000111010011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FDIV.S f3, f8, f16, rtz";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // Floating point square root
    //     let binary_instruction = "01011001000001000001000111010011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FSQRT.S f3, f8, rtz";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // Floating point sign injection
    //     let binary_instruction = "00100001000001000000000111010011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FSGNJ.S f3, f8, f16";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // Floating point injection negative
    //     let binary_instruction = "00100001000001000001000111010011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FSGNJN.S f3, f8, f16";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // Sign injection with xor 
    //     let binary_instruction = "00100001000001000010000111010011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FSGNJX.S f3, f8, f16";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // Floating point minimum
    //     let binary_instruction = "00101001000001000000000111010011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FMIN.S f3, f8, f16";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // Floating point maximum
    //     let binary_instruction = "00101001000001000001000111010011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FMAX.S f3, f8, f16";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // Floating point to integer
    //     let binary_instruction = "11000000000001000001000111010011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FCVT.W.S x3, f8, rtz";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // Floating point to unsigned integer
    //     let binary_instruction = "11000000000101000001000111010011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FCVT.WU.S x3, f8, rtz";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // Floating point (IEEE 754-2008) to integer
    //     let binary_instruction = "11100000000001000000000111010011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FMV.X.W x3, f8";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // FEQ.S
    //     let binary_instruction = "10100000000101000010000111010011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FEQ.S x3, f8, f1";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // FLT.S
    //     let binary_instruction = "10100000000101000001000111010011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FLT.S x3, f8, f1";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // FLE.S
    //     let binary_instruction = "10100000000101000000000111010011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FLE.S x3, f8, f1";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // FCLASS.S
    //     let binary_instruction = "10100000000101000000000111010011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FCLASS.S x3, f8";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);
  
    //     // FCVT.S.W
    //     let binary_instruction = "11010000000001000001000111010011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FCVT.S.W f3, x8, rtz";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // FCVT.S.WU
    //     let binary_instruction = "11010000000101000001000111010011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FCVT.S.WU f3, x8, rtz";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // FMV.W.X
    //     let binary_instruction = "11110000000001000001000111010011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FMV.W.X f3, x8";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // FLW
    //     let binary_instruction = "00000110010001000010001110000111";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FLW f7, 100(x8)";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // FSW
    //     let binary_instruction = "00000110111001001010001000100111";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FSW f14, 100(x9)";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);
    // }

    // #[test]
    // fn d_extension_instructions() {
    //     // FMADD.D
    //     let binary_instruction = "00001011001011000001001111000011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FMADD.D f7, f24, f18, f1, rtz";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // FMSUB.D
    //     let binary_instruction = "00001011001011000001001111000111";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FMSUB.D f7, f24, f18, f1, rtz";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // FNMADD.D
    //     let binary_instruction = "00001011001011000001001111001111";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FNMADD.D f7, f24, f18, f1, rtz";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // FNMSUB.D
    //     let binary_instruction = "00001011001011000001001111001011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FNMSUB.D f7, f24, f18, f1, rtz";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // FADD.D
    //     let binary_instruction = "00000011001011000001001111010011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FADD.D f7, f24, f18, rtz";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // FSUB.D
    //     let binary_instruction = "00001011001011000001001111010011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FSUB.D f7, f24, f18, rtz";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // FMUL.D
    //     let binary_instruction = "00010011001011000001001111010011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FMUL.D f7, f24, f18, rtz";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // FDIV.D
    //     let binary_instruction = "00011011001011000001001111010011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FDIV.D f7, f24, f18, rtz";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // FSQRT.D
    //     let binary_instruction = "01011011001011000001001111010011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FSQRT.D f7, f24, rtz";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // FSGNJ.D
    //     let binary_instruction = "00100011001011000000001111010011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FSGNJ.D f7, f24, f18";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // FSGNJN.D
    //     let binary_instruction = "00100011001011000001001111010011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FSGNJN.D f7, f24, f18";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // FSGNJX.D
    //     let binary_instruction = "00100011001011000010001111010011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FSGNJX.D f7, f24, f18";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // FMIN.D
    //     let binary_instruction = "00101011001011000000001111010011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FMIN.D f7, f24, f18";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // FMAX.D
    //     let binary_instruction = "00101011001011000001001111010011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FMAX.D f7, f24, f18";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // FCVT.S.D
    //     let binary_instruction = "01000000000111000001001111010011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FCVT.S.D f7, f24, rtz";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // FCVT.D.S
    //     let binary_instruction = "01000010000011000001001111010011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FCVT.D.S f7, f24, rtz";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // FEQ.D
    //     let binary_instruction = "10100010000011000010001111010011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FEQ.D x7, f24, f0";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // FLT.D
    //     let binary_instruction = "10100010000011000001001111010011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FLT.D x7, f24, f0";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // FLE.D
    //     let binary_instruction = "10100010000011000000001111010011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FLE.D x7, f24, f0";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // FCLASS.D
    //     let binary_instruction = "11100010000011000001001111010011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FCLASS.D x7, f24";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // FCVT.W.D
    //     let binary_instruction = "11000010000011000001001111010011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FCVT.W.D x7, f24, rtz";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // FCVT.WU.D
    //     let binary_instruction = "11000010000111000001001111010011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FCVT.WU.D x7, f24, rtz";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // FCVT.D.W
    //     let binary_instruction = "11010010000011000001001111010011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FCVT.D.W f7, x24, rtz";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // FCVT.D.WU
    //     let binary_instruction = "11010010000111000001001111010011";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FCVT.D.WU f7, x24, rtz";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // FLD
    //     let binary_instruction = "00000110010101001011001010000111";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FLD f5, 101(x9)";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);

    //     // FSD
    //     let binary_instruction = "00000110110101000011000110100111";
    //     let instr = convert_binary_string_to_vector(binary_instruction);
    //     let expected = "FSD f13, 99(x8)";
    //     let result = instruction_decoder(instr);
    //     assert_eq!(result, expected);
    // }
}
