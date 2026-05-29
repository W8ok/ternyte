use std::cell::RefCell;
use std::time::Instant;

thread_local! {
    static LAST_PRINT: RefCell<Instant> = RefCell::new(Instant::now());
    static FRAME_COUNT: RefCell<u32> = const { RefCell::new(0) };
}

pub fn tick() {
    FRAME_COUNT.with(|count| {
        let mut count = count.borrow_mut();
        *count += 1;

        LAST_PRINT.with(|last| {
            if last.borrow().elapsed().as_secs_f32() >= 1.0 {
                println!(
                    "FPS: {} | Frame time: {:.2}ms",
                    *count,
                    1000.0 / *count as f32
                );
                *count = 0;
                *last.borrow_mut() = Instant::now();
            }
        });
    });
}
