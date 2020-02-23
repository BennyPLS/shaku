use crate::container::ComponentMap;
use crate::module::{HasComponent, Module};
use crate::provider::ProviderFn;
use crate::ContainerBuilder;
use crate::Interface;
use crate::Provider;
use crate::{HasProvider, ProvidedInterface};
use std::error::Error;
use std::sync::Arc;

/// Resolves services associated with a [`Module`]. A `Container` is built by a
/// [`ContainerBuilder`], or through the shortcut [`Container::default`]
///
/// [`Module`]: trait.Module.html
/// [`ContainerBuilder`]: struct.ContainerBuilder.html
/// [`Container::default`]: struct.Container.html#method.default
pub struct Container<M: Module> {
    pub(crate) module: M,
    pub(crate) provider_overrides: ComponentMap,
}

impl<M: Module> Default for Container<M> {
    /// Build a default container. Same as `ContainerBuilder::new().build()`.
    fn default() -> Self {
        ContainerBuilder::new().build()
    }
}

impl<M: Module> Container<M> {
    /// Get a reference to the component registered with the interface `I`. The ownership of
    /// the component is shared via `Arc`.
    ///
    /// # Example
    /// ```
    /// # use shaku::{module, Component, Container, Interface};
    /// # use std::sync::Arc;
    /// #
    /// # trait Foo: Interface {}
    /// #
    /// # #[derive(Component)]
    /// # #[shaku(interface = Foo)]
    /// # struct FooImpl;
    /// # impl Foo for FooImpl {}
    /// #
    /// # module! {
    /// #     TestModule {
    /// #         components = [FooImpl],
    /// #         providers = []
    /// #     }
    /// # }
    /// #
    /// # let container = Container::<TestModule>::default();
    /// #
    /// let foo: Arc<dyn Foo> = container.resolve::<dyn Foo>();
    /// ```
    pub fn resolve<I: Interface + ?Sized>(&self) -> Arc<I>
    where
        M: HasComponent<I>,
    {
        Arc::clone(self.module.get_ref())
    }

    /// Create a service using the provider registered with the interface `I`.
    /// Each call will create a new instance of the service.
    ///
    /// # Errors
    /// Returns a [Error::ResolveError] if the provider failed while creating
    /// the service.
    ///
    /// [Error::ResolveError]: enum.Error.html
    ///
    /// # Examples
    /// ```
    /// # use shaku::{module, Container, ProvidedInterface, Provider};
    /// # use std::sync::Arc;
    /// #
    /// # trait Foo: ProvidedInterface {}
    /// #
    /// # #[derive(Provider)]
    /// # #[shaku(interface = Foo)]
    /// # struct FooImpl;
    /// # impl Foo for FooImpl {}
    /// #
    /// # module! {
    /// #     TestModule {
    /// #         components = [],
    /// #         providers = [FooImpl]
    /// #     }
    /// # }
    /// #
    /// # let container = Container::<TestModule>::default();
    /// #
    /// let foo: Box<dyn Foo> = container.provide::<dyn Foo>().unwrap();
    /// ```
    pub fn provide<I: ProvidedInterface + ?Sized>(&self) -> Result<Box<I>, Box<dyn Error + 'static>>
    where
        M: HasProvider<I>,
    {
        self.provider_overrides
            .get::<ProviderFn<M, I>>()
            .map(|provider_fn| provider_fn(self))
            .unwrap_or_else(|| M::Impl::provide(self))
    }

    /// Get a reference to the component registered with the interface `I`.
    ///
    /// # Example
    /// ```
    /// # use shaku::{module, Component, Container, Interface};
    /// # use std::sync::Arc;
    /// #
    /// # trait Foo: Interface {}
    /// #
    /// # #[derive(Component)]
    /// # #[shaku(interface = Foo)]
    /// # struct FooImpl;
    /// # impl Foo for FooImpl {}
    /// #
    /// # module! {
    /// #     TestModule {
    /// #         components = [FooImpl],
    /// #         providers = []
    /// #     }
    /// # }
    /// #
    /// # let container = Container::<TestModule>::default();
    /// #
    /// let foo: &dyn Foo = container.resolve_ref::<dyn Foo>();
    /// ```
    pub fn resolve_ref<I: Interface + ?Sized>(&self) -> &I
    where
        M: HasComponent<I>,
    {
        Arc::as_ref(self.module.get_ref())
    }

    /// Get a mutable reference to the component registered with the interface `I`.
    ///
    /// If the component is jointly owned (an `Arc<I>` reference to the component exists outside of
    /// this container), then `None` will be returned as it is unsafe to take a
    /// mutable reference without exclusive ownership of the component.
    ///
    /// # Example
    /// ```
    /// # use shaku::{module, Component, Container, Interface};
    /// # use std::sync::Arc;
    /// #
    /// # trait Foo: Interface {}
    /// #
    /// # #[derive(Component)]
    /// # #[shaku(interface = Foo)]
    /// # struct FooImpl;
    /// # impl Foo for FooImpl {}
    /// #
    /// # module! {
    /// #     TestModule {
    /// #         components = [FooImpl],
    /// #         providers = []
    /// #     }
    /// # }
    /// #
    /// # let mut container = Container::<TestModule>::default();
    /// #
    /// let foo: &mut dyn Foo = container.resolve_mut::<dyn Foo>().unwrap();
    /// ```
    pub fn resolve_mut<I: Interface + ?Sized>(&mut self) -> Option<&mut I>
    where
        M: HasComponent<I>,
    {
        Arc::get_mut(self.module.get_mut())
    }
}
