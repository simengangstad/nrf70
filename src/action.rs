use core::cell::{Cell, RefCell};
use core::future::{poll_fn, Future, Pending};
use core::ptr;
use core::task::{Poll, Waker};

use embassy_sync::waitqueue::WakerRegistration;

use crate::bindings::nrf_wifi_host_rpu_msg_type;
use crate::control::ScanOptions;
use crate::Error;

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Action {
    LoadFirmware(*const [u8]),
    Command((nrf_wifi_host_rpu_msg_type, bool, *const [u8], Option<*mut [u8]>)),
}

#[derive(Clone, Copy)]
enum ActionStateInner {
    Pending(Action),
    Sent { response_buffer: Option<*mut [u8]> },
    Done { result: Result<Option<usize>, Error> },
}

struct Wakers {
    control: WakerRegistration,
    runner: WakerRegistration,
}

impl Wakers {
    const fn new() -> Self {
        Self {
            control: WakerRegistration::new(),
            runner: WakerRegistration::new(),
        }
    }
}

pub struct ActionState {
    state: Cell<ActionStateInner>,
    wakers: RefCell<Wakers>,
}

impl ActionState {
    pub const fn new() -> Self {
        Self {
            state: Cell::new(ActionStateInner::Done { result: Ok(None) }),
            wakers: RefCell::new(Wakers::new()),
        }
    }

    fn wake_control(&self) {
        self.wakers.borrow_mut().control.wake();
    }

    fn register_control(&self, waker: &Waker) {
        self.wakers.borrow_mut().control.register(waker);
    }

    fn wake_runner(&self) {
        self.wakers.borrow_mut().runner.wake();
    }

    fn register_runner(&self, waker: &Waker) {
        self.wakers.borrow_mut().runner.register(waker);
    }

    pub fn wait_complete(&self) -> impl Future<Output = Result<Option<usize>, Error>> + '_ {
        poll_fn(|cx| {
            if let ActionStateInner::Done { result } = self.state.get() {
                Poll::Ready(result)
            } else {
                self.register_control(cx.waker());
                Poll::Pending
            }
        })
    }

    pub fn wait_pending(&self) -> impl Future<Output = Action> + '_ {
        poll_fn(|cx| {
            if let ActionStateInner::Pending(pending) = self.state.get() {
                self.state.set(ActionStateInner::Sent {
                    response_buffer: match pending {
                        Action::Command((_, _, _, response_buffer)) => response_buffer,
                        _ => None,
                    },
                });

                Poll::Ready(pending)
            } else {
                self.register_runner(cx.waker());
                Poll::Pending
            }
        })
    }

    pub fn cancel(&self) {
        self.state.set(ActionStateInner::Done { result: Ok(None) });
    }

    pub async fn issue(&self, action: Action) -> Result<Option<usize>, Error> {
        match self.state.get() {
            ActionStateInner::Done { result: _ } => (),
            _ => return Err(Error::Busy),
        }

        self.state.set(ActionStateInner::Pending(action));

        self.wake_runner();
        self.wait_complete().await
    }

    pub fn respond(&self, result: Result<Option<*const [u8]>, Error>) {
        if let ActionStateInner::Sent { response_buffer } = self.state.get() {
            // Response buffer may be a value (given by the optional) and should be filled under the following conditions:
            //
            // * The result is OK and its optional contains a value
            // * The response buffer has enough space for the result
            fn get_result(
                result: Result<Option<*const [u8]>, Error>,
                response_buffer: Option<*mut [u8]>,
            ) -> Result<Option<usize>, Error> {
                match result {
                    Ok(Some(result_data)) => unsafe {
                        match response_buffer {
                            Some(response_buffer_ptr) => {
                                let result_data_length = result_data.len();
                                let response_buffer: &mut [u8] = &mut *response_buffer_ptr;

                                if response_buffer.len() < result_data_length {
                                    return Err(Error::BufferTooSmall);
                                }

                                let result_data_ptr: &[u8] = &*result_data;

                                ptr::copy_nonoverlapping(
                                    result_data_ptr.as_ptr(),
                                    response_buffer.as_mut_ptr(),
                                    result_data_length,
                                );

                                Ok(Some(result_data_length))
                            }
                            None => Ok(None),
                        }
                    },
                    Ok(None) => Ok(None),
                    Err(e) => Err(e),
                }
            }

            self.state.set(ActionStateInner::Done {
                result: get_result(result, response_buffer),
            });

            self.wake_control();
        } else {
            warn!("Acking action, but no pending action");
        }
    }
}
