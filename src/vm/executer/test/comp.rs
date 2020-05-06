#[test]
fn TestEq() {
    let constant_pool = [
        0x01,
        0x03,
        0x02,0x00,0x00,0x00,
        0x01,0x00,0x00,0x00,0x01,0x00,0x00,0x00,
        0x02,0x00,0x00,0x00,0x01,0x00,0x00,0x00
    ];
    let bytes = [
        0x03, 0x00, 0x00, 0x00, 
        0x6a, 0x00, 0x6d,0x00,0x70,0x00,
        0x00,0x00,0x00,0x00,
        0x00,
        0x00,
        0x00,
        0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,
        
        0x01,0x00,
    // start
    // label : 0
        0x00,0x00,
        0x04,0x00,0x00,0x00,
        0xFF,0x01,0x05,0x00,0x03,0x01,0x00,0x00,0x00,
        0xFF,0x01,0x05,0x00,0x03,0x02,0x00,0x00,0x00,
        0x00,0x02,0x02,0x03,0x00,
        0x00,0x90,0x02,0x00,0x01,
    // end
        0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,
    ];
    reader::Reader::read_constant_pool(constant_pool.as_ptr(), constant_pool.len());
    let mut reader = reader::Reader::new(bytes.as_ptr());
    let func = reader.read_func();
    println!("{}",func);

    let mut state = super::super::state::State::new();
    let mut stack= super::super::stack::Stack::new(Box::new(func));
    state.push_stack(stack);
    println!("===== testing eq =====");
    println!("before execute {:?}",state.stack().stack);
    state.execute();
    println!("after execute {:?}\n",state.stack().stack);
    use super::super::super::super::bin_format::*;
    assert_eq!(*(state.stack().stack.last().unwrap()),super::super::Value(super::super::PrimeValue::Bool(true),super::super::super::super::bin_format::Type::Mono(TAG_BOOL)))
}

#[test]
fn FIB() {
    /*
stack  var     ins
0        n
1        a
2        b
{0x00 0x00}:
3           LOADK [0]
4           LOADK [1]
5           EQ 0 3
            JPE 5 {RETURN_A}
6           EQ 0 4
            JPE 6 {RETURN_B}
7           Sub 0 4
8           ADD 1 2
            FIXTOP 7
            FIXTOP 2
            FIXTOP 8
            TAILCALL
{0x01 0x00}:
            FIXTOP 1
            RETURN
{0x02 0x00}:
            FIXTOP 2
            RETURN
*/
        let constant_pool = [
        0x02,
        0x03,
        0x03,0x00,0x00,0x00,

        0x01,0x00,0x00,0x00,
            0x00,0x00,0x00,0x00,
        0x02,0x00,0x00,0x00,
            0x01,0x00,0x00,0x00,
        0x03,0x00,0x00,0x00,
            0x0F,0x00,0x00,0x00,


        0x11,
        0x01,0x00,0x00,0x00,
        0x01,0x00,0x00,0x00,

        0x03, 0x00, 0x00, 0x00, 
        0x66,0x00, 0x69,0x00, 0x62,0x00,
        0x00,0x00,0x00,0x00,
        0x00,
        0x00,
        0x00,
        0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,
        0x03,0x00,
    // start
    // label : 0
        0x00,0x00,
        0x0d,0x00,0x00,0x00,
        
        0xFF,0x01,0x05,0x00,0x03,0x01,0x00,0x00,0x00,
        0xFF,0x01,0x05,0x00,0x03,0x02,0x00,0x00,0x00,
        0x00,0x02,0x05,0x0a,0x00,
        // if arg(0) == [5]0
        0x00,0x90,0x05,0x00,0x03,
        0x00,0x26,0x05,0x01,0x00,
        // if arg(0) == [6]1
        0x00,0x90,0x06,0x00,0x04,
        0x00,0x26,0x06,0x02,0x00,
        // arg(0) - [6]1
        0x00,0x6a,0x07,0x00,0x04,
        // arg(1) + arg(2)
        0x00,0x69,0x08,0x01,0x02,
        
        0x00,0x4d,0x08,0x00,0x00,
        0x00,0x4d,0x02,0x00,0x00,
        0x00,0x4d,0x09,0x00,0x00,
        // tail call
        0x00,0x23,0x00,0x00,0x00,

    // label : 1
        0x01,0x00,
        0x02,0x00,0x00,0x00,
        0x00,0x4d,0x01,0x00,0x00,
        0x00,0x25,0x00,0x00,0x00,
    // label : 2
        0x02,0x00,
        0x02,0x00,0x00,0x00,
        0x00,0x4d,0x02,0x00,0x00,
        0x00,0x25,0x00,0x00,0x00,
    // end
        0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,

    ];
    let bytes = [
        0x04, 0x00, 0x00, 0x00,
        0x6D,0x00, 0x61,0x00, 0x69,0x00, 0x6E,0x00,
        0x00,0x00,0x00,0x00,
        0x00,
        0x00,
        0x00,
        0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,
        0x01,0x00,
    // start
    // label : 0
        0x00,0x00,
        0x05,0x00,0x00,0x00,
        0x00,0x45,0x00,0x00,0x00,
        0xFF,0x01,0x05,0x00,0x03,0x02,0x00,0x00,0x00,
        0xFF,0x01,0x05,0x00,0x03,0x01,0x00,0x00,0x00,
        0xFF,0x01,0x05,0x00,0x03,0x03,0x00,0x00,0x00,
        0x00,0x22,0x00,0x03,0x00,
    // end
        0x01,0x00,0x00,0x00,
        0x11,0x01,0x00,0x00,0x00,
    
        0x00,0x00,0x00,0x00,
    ];
    
    super::super::super::super::reader::Reader::read_constant_pool(constant_pool.as_ptr(), constant_pool.len());
    let mut reader = super::super::super::super::reader::Reader::new(bytes.as_ptr());
    let func = reader.read_func();
    println!("{}",func);

    let mut state = super::super::state::State::new();
    let mut stack= super::super::stack::Stack::new(Box::new(func));
    state.push_stack(stack);
    println!("===== testing fib =====");
    println!("before execute {:?}",state.stack().stack);
    state.execute();
    println!("after execute {:?}\n",state.stack().stack);
    assert_eq!(*(state.stack().stack.last().unwrap()),super::super::Value(super::super::PrimeValue::Int(610),super::super::super::super::bin_format::Type::Mono(super::super::super::super::TAG_INT)));
    println!("LEMONVM DID IT!!!!!!!!!!! FIBIONACCI!!!!!!!");
}