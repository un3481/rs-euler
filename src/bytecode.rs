
pub type Operation = (u8, isize);
pub type ByteCode = Box<[Operation]>;

pub enum Code {
    StackPush,
    StackPop,
    ScopeSet,
    ScopeGet,
    LineBreak,
    BoolAnd,
    BoolOr,
    MathAdd,
    MathSub,
    MathMul,
    MathDiv,
    IfStart,
    IfElse,
    LoopStart,
    LoopContinue,
    LoopBreak,
    FunStart,
    FunEnd,
    FunCall,
    ErrorRaise,
}
