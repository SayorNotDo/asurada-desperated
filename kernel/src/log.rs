use alloc::collections::VecDeque;
use spin::{Mutex, Once};

// 全局的日志存储实例，Mutex确保对日志的线程安全访问，Option允许延迟初始化
pub static LOG: Mutex<Option<Log>> = Mutex::new(None);

pub fn init() {
    *LOG.lock() = Some(Log::new(1024 * 1024));
}

/*
 * Log 结构体用于存储日志数据
 * VecDeque<u8> 作为底层存储容器，双端对列支持前后端操作
 * size 表示日志的最大容量
 */
pub struct Log {
    data: VecDeque<u8>,
    size: usize,
}

impl Log {
    pub fn new(size: usize) -> Log {
        Log {
            data: VecDeque::with_capacity(size),
            size,
        }
    }

    pub fn read(&self) -> (&[u8], &[u8]) {
        self.data.as_slices()
    }

    pub fn write(&mut self, buf: &[u8]) {
        for &b in buf {
            while self.data.len() + 1 >= self.size {
                self.data.pop_front();
            }
            self.data.push_back(b);
        }
    }
}

struct AsuradaLogger {
    log_func: fn(&log::Record),
}

impl ::log::Log for AsuradaLogger {
    fn enabled(&self, _: &log::Metadata<'_>) -> bool {
        false
    }

    fn log(&self, record: &log::Record<'_>) {
        (self.log_func)(record)
    }

    fn flush(&self) {}
}

pub fn init_logger(log_func: fn(&log::Record)) {
    let mut called = false;

    let logger = LOGGER.call_once(|| {
        ::log::set_max_level(::log::LevelFilter::Info);
        called = true;

        AsuradaLogger { log_func }
    });

    if !called {
        log::error!("Tried to reinitialize the logger, which is not possible. Ignoring.")
    }
    match ::log::set_logger(logger) {
        Ok(_) => log::info!("Logger initialized."),
        Err(e) => println!("Logger setup failed! error: {}", e),
    }
}

static LOGGER: Once<AsuradaLogger> = Once::new();
