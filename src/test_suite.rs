use dioxus::{
    core::{generation, schedule_update},
    prelude::*,
};
use dioxus_core::NoOpMutations;
use futures::future::FutureExt;
use std::{
    cell::RefCell,
    panic::{AssertUnwindSafe, catch_unwind},
    rc::Rc,
    sync::Arc,
};

/// MockProxy tracks rendering generations and allows manual rerunning
#[allow(dead_code)]
pub struct MockProxy {
    rerender: Arc<dyn Fn()>,
    pub generation: usize,
}

#[allow(dead_code)]
impl MockProxy {
    pub fn new() -> Self {
        let generation = generation();
        let rerender = schedule_update();
        Self {
            rerender,
            generation,
        }
    }

    pub fn rerun(&mut self) {
        (self.rerender)();
    }
}

impl Default for MockProxy {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple hook testing function for one-time execution without VirtualDom overhead.
///
/// This is useful for testing hooks that don't require re-rendering or generation tracking.
/// The hook is executed once in a minimal Dioxus context, and assertions can be made
/// on the returned value directly.
///
/// # Example
///
/// ```ignore
/// test_hook_simple(|| {
///     let context = TableContext::use_table_context((Column1, Column2));
///     let rows = Signal::new(vec![data]);
///     context.data.swap_columns(0, 1);
///
///     let mut exporter = MockExporter::new();
///     context.serialize(rows.into(), &mut exporter).unwrap();
///     exporter
/// }, |exporter| {
///     assert_eq!(exporter.headers.len(), 2);
/// });
/// ```
pub fn test_hook_simple<V: 'static, F, A>(hook: F, assert: A)
where
    F: FnMut() -> V + 'static,
    A: FnOnce(V) + 'static,
{
    let result = Rc::new(RefCell::new(None));
    let result_clone = result.clone();

    let assert_cell = Rc::new(RefCell::new(Some(assert)));
    let assert_clone = assert_cell.clone();

    test_hook(
        hook,
        move |value, proxy| {
            // Only store the result on generation 0
            if proxy.generation == 0 {
                *result_clone.borrow_mut() = Some(value);
            }
        },
        move |proxy| {
            // Verify we're at generation 1 (one render has completed)
            assert_eq!(
                proxy.generation, 1,
                "test_hook_simple should complete at generation 1"
            );

            // Extract the result and run assertions
            if let Some(value) = result.borrow_mut().take() {
                if let Some(assert_fn) = assert_clone.borrow_mut().take() {
                    assert_fn(value);
                }
            } else {
                panic!("Hook did not produce a value");
            }
        },
    );
}

/// Generic hook testing function
pub fn test_hook<V: 'static>(
    initialize: impl FnMut() -> V + 'static,
    check: impl FnMut(V, MockProxy) + 'static,
    mut final_check: impl FnMut(MockProxy) + 'static,
) {
    // Store any panic that occurs in the check closure
    let panic_payload: Rc<RefCell<Option<Box<dyn std::any::Any + Send>>>> =
        Rc::new(RefCell::new(None));
    let panic_payload_clone = panic_payload.clone();

    // Mock component and related trait implementations
    #[derive(Props)]
    struct MockAppComponent<I: 'static, C: 'static> {
        hook: Rc<RefCell<I>>,
        check: Rc<RefCell<C>>,
        panic_payload: Rc<RefCell<Option<Box<dyn std::any::Any + Send>>>>,
    }

    impl<I, C> PartialEq for MockAppComponent<I, C> {
        fn eq(&self, _: &Self) -> bool {
            true
        }
    }

    impl<I, C> Clone for MockAppComponent<I, C> {
        fn clone(&self) -> Self {
            Self {
                hook: self.hook.clone(),
                check: self.check.clone(),
                panic_payload: self.panic_payload.clone(),
            }
        }
    }

    // Mock app component for testing
    fn mock_app<I: FnMut() -> V, C: FnMut(V, MockProxy), V>(
        props: MockAppComponent<I, C>,
    ) -> Element {
        let value = props.hook.borrow_mut()();

        // Catch any panic in the check closure
        let result = catch_unwind(AssertUnwindSafe(|| {
            props.check.borrow_mut()(value, MockProxy::new());
        }));

        // Store the panic payload if one occurred
        if let Err(panic) = result {
            *props.panic_payload.borrow_mut() = Some(panic);
        }

        rsx! {
            div {}
        }
    }

    // Set up virtual DOM and run test, catching any panics
    let test_result = catch_unwind(AssertUnwindSafe(|| {
        let mut vdom = VirtualDom::new_with_props(
            mock_app,
            MockAppComponent {
                hook: Rc::new(RefCell::new(initialize)),
                check: Rc::new(RefCell::new(check)),
                panic_payload: panic_payload_clone,
            },
        );

        vdom.rebuild_in_place();
        while vdom.wait_for_work().now_or_never().is_some() {
            vdom.render_immediate(&mut NoOpMutations);
        }

        vdom.in_scope(ScopeId::ROOT, || {
            final_check(MockProxy::new());
        })
    }));

    // Check if a panic occurred in the check closure
    if let Some(panic) = panic_payload.borrow_mut().take() {
        std::panic::resume_unwind(panic);
    }

    // If a panic occurred in final_check, re-panic to fail the test
    if let Err(panic_info) = test_result {
        std::panic::resume_unwind(panic_info);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hook_simple_basic() {
        test_hook_simple(
            || {
                let value = use_signal(|| 42);
                value()
            },
            |result| {
                assert_eq!(result, 42);
            },
        );
    }

    #[test]
    #[should_panic(expected = "left == right")]
    fn test_hook_simple_with_failing_assertion() {
        test_hook_simple(
            || {
                let value = use_signal(|| 42);
                value()
            },
            |result| {
                assert_eq!(result, 100); // This will fail
            },
        );
    }

    #[test]
    #[should_panic(expected = "This should fail the test!")]
    fn test_panic_in_final_check() {
        test_hook(
            || 42,
            |val, _| {
                assert_eq!(val, 42);
            },
            |_| {
                panic!("This should fail the test!");
            },
        );
    }

    #[test]
    #[should_panic(expected = "left == right")]
    fn test_panic_in_check() {
        test_hook(
            || 42,
            |val, _| {
                assert_eq!(val, 100); // This will fail
            },
            |_| {},
        );
    }
}
