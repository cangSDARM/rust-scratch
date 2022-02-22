use std::{
    mem::MaybeUninit,
    sync::{Mutex, Once},
    thread,
};

#[derive(Debug)]
struct Config {
    conf: String,
}

#[allow(dead_code)]
pub fn singleton() -> &'static Mutex<Config> {
    // 用MaybeUninit创建一块 未被初始化的内存
    static mut CONFIG: MaybeUninit<Mutex<Config>> = MaybeUninit::uninit();
    static ONCE: Once = Once::new();

    // 使用static Once，底层只会调用一次
    ONCE.call_once(|| unsafe {
        //指向MaybeUninit的内存地址，写入
        CONFIG.as_mut_ptr().write(Mutex::new(Config {
            conf: "test".to_string(),
        }));
    });
    //转成指针(size可以确定，规避编译限制)导出
    unsafe { &*CONFIG.as_ptr() }
}

// 状态转移的参考[./state_monad.rs]
