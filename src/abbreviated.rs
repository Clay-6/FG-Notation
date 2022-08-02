pub struct Move {
    button: Button,
    motion: Motion,
    modifier: Modifier,
}

pub struct Button(String);

pub enum Motion {
    U,
    D,
    F,
    B,
    DB,
    DF,
    UB,
    UF,
    QCF,
    QCB,
    HCF,
    HCB,
    DP,
    RDP,
    FullCircle,
    Double360,
}

pub enum Modifier {
    Close,
    Far,
    Standing,
    Crouching,
    Jump,
    JumpCancel,
    TigerKnee,
}
