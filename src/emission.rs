use crate::assembly::{
    AssemblyASTFunction, AssemblyASTInstruction, AssemblyASTOperand, AssemblyASTRootNode,
};

pub fn emit_code(assembly_ast: AssemblyASTRootNode) -> String {
    let mut program = String::new();
    match assembly_ast {
        AssemblyASTRootNode::Program(f) => {
            program.push_str(&emit_function(f));
            program.push_str(".section .note.GNU-stack,\"\",@progbits");
        }
    }

    return program;
}

fn emit_function(f: AssemblyASTFunction) -> String {
    let mut function_string = String::new();

    function_string += &format!("  .globl {}\n{}:\n", f.name, f.name);
    for instr in f.instructions {
        function_string += &emit_instruction(instr);
    }

    return function_string;
}

fn emit_instruction(asm_instr: AssemblyASTInstruction) -> String {
    match asm_instr {
        AssemblyASTInstruction::Mov { src, dst } => {
            format!("   movl {}, {}\n", emit_operand(src), emit_operand(dst))
        }
        AssemblyASTInstruction::Ret => String::from("   ret\n"),
    }
}

fn emit_operand(asm_operand: AssemblyASTOperand) -> String {
    match asm_operand {
        AssemblyASTOperand::Register => String::from("%eax"),
        AssemblyASTOperand::Imm(c) => format!("${c}"),
    }
}
