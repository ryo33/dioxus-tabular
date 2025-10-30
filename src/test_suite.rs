use dioxus::{
    core::{generation, schedule_update},
    prelude::*,
};
use dioxus_core::NoOpMutations;
use futures::future::FutureExt;
use std::{cell::RefCell, rc::Rc, sync::Arc};

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

/// Generic hook testing function
pub fn test_hook<V: 'static>(
    initialize: impl FnMut() -> V + 'static,
    check: impl FnMut(V, MockProxy) + 'static,
    mut final_check: impl FnMut(MockProxy) + 'static,
) {
    // Mock component and related trait implementations
    #[derive(Props)]
    struct MockAppComponent<I: 'static, C: 'static> {
        hook: Rc<RefCell<I>>,
        check: Rc<RefCell<C>>,
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
            }
        }
    }

    // Mock app component for testing
    fn mock_app<I: FnMut() -> V, C: FnMut(V, MockProxy), V>(
        props: MockAppComponent<I, C>,
    ) -> Element {
        let value = props.hook.borrow_mut()();
        props.check.borrow_mut()(value, MockProxy::new());
        rsx! {
            div {}
        }
    }

    // Set up virtual DOM and run test
    let mut vdom = VirtualDom::new_with_props(
        mock_app,
        MockAppComponent {
            hook: Rc::new(RefCell::new(initialize)),
            check: Rc::new(RefCell::new(check)),
        },
    );

    vdom.rebuild_in_place();
    while vdom.wait_for_work().now_or_never().is_some() {
        vdom.render_immediate(&mut NoOpMutations);
    }

    vdom.in_scope(ScopeId::ROOT, || {
        final_check(MockProxy::new());
    })
}
