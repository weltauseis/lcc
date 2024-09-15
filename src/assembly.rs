use crate::parser::{ASTFunction, ASTRootNode, ASTStatement};

#[derive(Debug)]
pub enum AssemblyASTRootNode {
    Program(AssemblyASTFunction),
}

#[derive(Debug)]
pub struct AssemblyASTFunction {
    pub name: String,
    pub instructions: Vec<AssemblyASTInstruction>,
}

#[derive(Debug)]
pub enum AssemblyASTInstruction {
    Mov {
        src: AssemblyASTOperand,
        dst: AssemblyASTOperand,
    },
    Ret,
}

#[derive(Debug)]
pub enum AssemblyASTOperand {
    Register,
    Imm(i32),
}

pub fn generate_assembly(ast: ASTRootNode) -> AssemblyASTRootNode {
    match ast {
        ASTRootNode::Program(f) => AssemblyASTRootNode::Program(generate_function(f)),
    }
}

fn generate_function(f: ASTFunction) -> AssemblyASTFunction {
    let name = f.name;

    let instructions = generate_instructions(f.body);

    return AssemblyASTFunction { name, instructions };
}

fn generate_instructions(statement: ASTStatement) -> Vec<AssemblyASTInstruction> {
    let mut instructions: Vec<AssemblyASTInstruction> = Vec::new();

    match statement {
        ASTStatement::Return(expr) => {
            let src = match expr {
                crate::parser::ASTExpression::Constant(c) => c,
            };

            instructions.push(AssemblyASTInstruction::Mov {
                src: AssemblyASTOperand::Imm(src),
                dst: AssemblyASTOperand::Register,
            });

            instructions.push(AssemblyASTInstruction::Ret);
        }
    }

    return instructions;
}
