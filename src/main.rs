fn add(op0: u32, op1: u32) -> u32 {
    op0 + op1
}

fn main() {
    let opcode: u8 = 0x02;
    let mut r0: u32 = 10;
    let r1: u32 = 20;

    println!(
        "do-core-1: opcode {} Initial CPU state [R0:{} R1:{}]",
        opcode, r0, r1
    );

    if opcode == 0x02 {
        // ADD r0, r1
        r0 = add(r0, r1);
    } else {
        panic!("Unknown opcode {}", opcode);
    }

    println!(
        "do-core-1: opcode {} Final CPU state [R0:{} R1:{}]",
        opcode, r0, r1
    );
}
