program token_17685.aleo;

record Token:
    owner as address.private;
    balance as u64.private;


function mint:
    input r0 as u64.private;
    cast self.caller r0 into r1 as Token.record;
    output r1 as Token.record;


function transfer_private:
    input r0 as address.private;
    input r1 as u64.private;
    input r2 as Token.record;
    sub r2.balance r1 into r3;
    cast self.caller r1 into r4 as Token.record;
    cast r0 r1 into r5 as Token.record;
    output r4 as Token.record;
    output r5 as Token.record;
