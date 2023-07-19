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
    fn atmomic_instructions() {
        // Load Reserved
        let binary_instruction = "00010000001000011010000010101111";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "LR.W x1, x3";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // Store if reserved
        let binary_instruction = "00011000000100011010000010101111";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "SC.W x1, x3, x2";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // Amoswap
        let binary_instruction = "00001000000100011010000010101111";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "AMOSWAP.W x1, x3, x2";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // Amoadd
        let binary_instruction = "00000000000100011010000010101111";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "AMOADD.W x1, x3, x2";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // Amoxor
        let binary_instruction = "00100000000100011010000010101111";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "AMOXOR.W x1, x3, x2";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // Amoand
        let binary_instruction = "01100000000100011010000010101111";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "AMOAND.W x1, x3, x2";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // Amoor
        let binary_instruction = "01000000000100011010000010101111";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "AMOOR.W x1, x3, x2";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // Amomin
        let binary_instruction = "10000000000100011010000010101111";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "AMOMIN.W x1, x3, x2";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // Amomax
        let binary_instruction = "10100000000100011010000010101111";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "AMOMAX.W x1, x3, x2";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // Amomin unsigned
        let binary_instruction = "11000000000100011010000010101111";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "AMOMINU.W x1, x3, x2";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);

        // Amomax unsigned
        let binary_instruction = "11100000000100011010000010101111";
        let instr = convert_binary_string_to_vector(binary_instruction);
        let expected = "AMOMAXU.W x1, x3, x2";
        let result = instruction_decoder(instr);
        assert_eq!(result, expected);
    }
}
