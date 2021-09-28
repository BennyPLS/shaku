//! This module contains trait definitions for provided services and interfaces

use crate::module::ModuleInterface;
use crate::Module;
use std::error::Error;

/// Like [`Component`]s, providers provide a service by implementing an interface.
///
/// Unlike [`Component`], `Provider` represents a temporary service. Examples include a connection
/// to a remote service or pooled database connection. Because only providers can have other
/// providers as dependencies, services which use these provided services must also be `Provider`s
/// (ex. DB repository, service using a DB repository, etc).
///
/// See also the [provider getting started guide].
///
/// ## Generics
/// * `M` - The module. This is used to require certain things about the module
///   that this provider is used in, such as requiring dependencies via
///   [`HasComponent`]/[`HasProvider`].
/// * `I` - The trait/interface which this provider implements.
///
/// [`Component`]: trait.Component.html
/// [`HasComponent`]: trait.HasComponent.html
/// [`HasProvider`]: trait.HasProvider.html
/// [provider getting started guide]: guide/provider/index.html
pub trait Provider<M: Module, I: ?Sized + 'static>: 'static {
    /// Provides the service, possibly resolving other components/providers
    /// to do so.
    fn provide(module: &M) -> Result<Box<I>, Box<dyn Error>>;
}

/// The type signature of [`Provider::provide`]. This is used when overriding a
/// provider via [`ModuleBuilder::with_provider_override`]
///
/// [`Provider::provide`]: trait.Provider.html#tymethod.provide
/// [`ModuleBuilder::with_provider_override`]: struct.ModuleBuilder.html#method.with_provider_override
#[cfg(not(feature = "thread_safe"))]
pub type ProviderFn<M, I> = Box<dyn (Fn(&M) -> Result<Box<I>, Box<dyn Error>>)>;
/// The type signature of [`Provider::provide`]. This is used when overriding a
/// provider via [`ModuleBuilder::with_provider_override`]
///
/// [`Provider::provide`]: trait.Provider.html#tymethod.provide
/// [`ModuleBuilder::with_provider_override`]: struct.ModuleBuilder.html#method.with_provider_override
#[cfg(feature = "thread_safe")]
pub type ProviderFn<M, I> = Box<dyn (Fn(&M) -> Result<Box<I>, Box<dyn Error>>) + Send + Sync>;

/// Indicates that a module contains a provider which implements the interface.
pub trait HasProvider<I: ?Sized>: ModuleInterface {
    /// Create a service using the provider registered with the interface `I`.
    /// Each call will create a new instance of the service.
    ///
    /// # Examples
    /// ```
    /// # use shaku::{module, HasProvider, Provider};
    /// # use std::sync::Arc;
    /// #
    /// # trait Foo {}
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
    /// # fn main() {
    /// # let module = TestModule::builder().build();
    /// #
    /// let foo: Box<dyn Foo> = module.provide().unwrap();
    /// # }
    /// ```
    fn provide(&self) -> Result<Box<I>, Box<dyn Error>>;
}
