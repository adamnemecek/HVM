use std::sync::atomic::{fence, AtomicBool, AtomicUsize, Ordering};

pub struct Barrier {
  pub done: AtomicUsize,
  pub pass: AtomicUsize,
  pub tids: usize,
}

impl Barrier {
  pub fn new(tids: usize) -> Self {
    Self { done: 0.into(), pass: 0.into(), tids }
  }

  pub fn wait(&self, stop: &AtomicUsize) {
    let pass = self.pass.load(Ordering::Relaxed);
    if self.done.fetch_add(1, Ordering::SeqCst) == self.tids - 1 {
      self.done.store(0, Ordering::Relaxed);
      self.pass.store(pass + 1, Ordering::Release);
    } else {
      while stop.load(Ordering::Relaxed) != 0 && self.pass.load(Ordering::Relaxed) == pass {}
      fence(Ordering::Acquire);
    }
  }
}
