use wasm_bindgen::prelude::*;
struct memory{
    memory_unit: [u8,4096];
}
struct registers{
    v0: [u8,8];
    v1: [u8,8];
    v2: [u8,8];
    v3: [u8,8];
    v4: [u8,8];
    v5: [u8,8];
    v6: [u8,8];
    v7: [u8,8];
    v8: [u8,8];
    v9: [u8,8];
    va: [u8,8];
    vb: [u8,8];
    vc: [u8,8];
    vd: [u8,8];
    ve: [u8,8];
    vf: [u8,8];
    address_register: [u8,16];
};
struct stack{
    stack_element: Vec<u8>;
}
struct delay_timer{
    timer: u8
}
struct sound_timer{
    timer: u8
}
fn create_memory()->memory{
    let mut memory = memory{
        memory_unit:[u8, 4096] = [];
    };
    for i in (0..512){
        memory.memory_unit[i] = 0;
    }
    for i in (3744..3840){
        memory.memory_unit[i] = 0;
    }
    for i in (3840..4096){
        memory.memory_unit[i] = 0;
    }
    return memory;
}
fn create_register()->registers{
    let mut register = registers{
        v0 = [0,0,0,0,0,0,0],
        v1 = [0,0,0,0,0,0,0],
        v2 = [0,0,0,0,0,0,0],
        v3 = [0,0,0,0,0,0,0],
        v4 = [0,0,0,0,0,0,0],
        v5 = [0,0,0,0,0,0,0],
        v6 = [0,0,0,0,0,0,0],
        v7 = [0,0,0,0,0,0,0],
        v8 = [0,0,0,0,0,0,0],
        v9 = [0,0,0,0,0,0,0],
        va = [0,0,0,0,0,0,0],
        vb = [0,0,0,0,0,0,0],
        vc = [0,0,0,0,0,0,0],
        vd = [0,0,0,0,0,0,0],
        ve = [0,0,0,0,0,0,0],
        vf = [0,0,0,0,0,0,0],
        address_register = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
    }
    return register;
}
fn insert_to_stack(address: [u8,16],stack: &stack){
    for _ in (0..16){
        stack.stack_element.push(address[i]);
    }
}
fn remove_from_stack(stack: &stack){
    for _ in (0..16){
        stack.stack_element.pop();
    }
}
#[wasm_bindgen]
pub fn setup(){
    let mut ram = create_memory();
    let mut registers = create_register();
    let mut stack = stack{
        stack_element = Vec::new()
    };

}