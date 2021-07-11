pub mod prelude;

/// Operator defines a trait that represents a valid operator within the 3AC
/// specification for a given language.
pub trait Operator {}

/// Operand provides a trait that demarcates that something can be used as a
/// Operand or Return value in a given ThreeAddressCode structure.
pub trait Operand {}

/// Defines the output of a `Lower.lower` call that may or may not fail with a
/// corresponding error string attached.
type LowerResult<'a, T> = Result<T, String>;

/// Defines a Lower method for taking a given Input (A), typically a singular
/// ThreeAddressCode or various groupings of ThreeAddressCodes, and lowers
/// this to a lower representation (B) for Target (T).
pub trait Lower<'a, T, A, B> {
    fn lower(&self, input: A) -> LowerResult<'a, B>;
}

/// Represents a three address code representation of an instruction.
#[derive(Debug)]
pub struct ThreeAddressCode<OPER, OPERAND1, OPERAND2, RET> {
    operator: OPER,
    operand_1: OPERAND1,
    operand_2: OPERAND2,
    ret_val: RET,
}

impl<OPER, OPERAND1, OPERAND2, RET> ThreeAddressCode<OPER, OPERAND1, OPERAND2, RET> {
    pub fn new(operator: OPER, operand_1: OPERAND1, operand_2: OPERAND2, ret_val: RET) -> Self {
        Self {
            operator,
            operand_1,
            operand_2,
            ret_val,
        }
    }
}
