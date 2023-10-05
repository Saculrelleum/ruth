static mut IOTA_COUNTER: i32 = 0;
static mut OP_PUSH: i32 = 0;
static mut OP_PLUS: i32 = 0;
static mut OP_MINUS: i32 = 0;
static mut OP_DUMP: i32 = 0;

fn iota(reset: bool) -> i32 {
    unsafe {
        if reset {
            IOTA_COUNTER = 0;
        }
        let result = IOTA_COUNTER;
        IOTA_COUNTER += 1;
        result
    }
}

fn push(x: i32) -> (i32, i32) {
    let op_push = unsafe { OP_PUSH };
    (op_push, x)
}

fn plus() -> (i32, i32) {
    let op_plus = unsafe { OP_PLUS };
    (op_plus, 0)
}

fn minus() -> (i32, i32) {
    let op_minus = unsafe { OP_MINUS };
    (op_minus, 0)
}

fn dump() -> (i32, i32) {
    let op_dump = unsafe { OP_DUMP };
    (op_dump, 0)
}

fn simulate_program(program: Vec<(i32, i32)>) {
    let mut stack: Vec<i32> = Vec::new();

    for op in program {
        if op.0 == unsafe { OP_PUSH } {
            stack.push(op.1);
        } else if op.0 == unsafe { OP_PLUS } {
            let x = stack.pop().unwrap();
            let y = stack.pop().unwrap();
            stack.push(x + y);
        } else if op.0 == unsafe { OP_MINUS } {
            let x = stack.pop().unwrap();
            let y = stack.pop().unwrap();
            stack.push(y - x);
        } else if op.0 == unsafe { OP_DUMP } {
            let x = stack.pop().unwrap();
            println!("{}", x);
        } else {
            assert!(false, "Unknown operation");
        }
        // Handle other operations as needed
    }
}

fn compile_program(program: Vec<(i32, i32)>) {
    for op in program {
        if op.0 == unsafe { OP_PUSH } {
            // Implement the logic for OP_PUSH
            println!("OP_PUSH with value: {}", op.1);
        }
        // Handle other operations as needed
    }
}

fn main() {
    // Initialize global variables
    unsafe {
        OP_PUSH = iota(false);
        OP_PLUS = iota(false);
        OP_MINUS = iota(false);
        OP_DUMP = iota(false);
    }

    let program = vec![
        push(34),
        push(35),
        plus(), 
        dump(), 
        push(500), 
        push(80), 
        minus(), 
        dump()
    ];

    simulate_program(program);
}
