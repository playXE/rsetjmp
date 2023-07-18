use rsetjmp::*;
fn main() {
    unsafe {
        let mut env = JumpBuf::new();

        let ret = setjmp(&mut env);

        if ret == 0 {
            println!("Jump");
            longjmp(&mut env, 1);
        } else {
            println!("Exit");
            drop(env);
        }
    }
}
