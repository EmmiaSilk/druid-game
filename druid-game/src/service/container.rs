//! Defines the [`ServiceContainer`] and its related errors. 
//! 
//! Note: Don't reference this module's children from this module's path
//! directly! Instead, use its parent module, [`service`](super).

use std::error::Error;
use std::fmt::Display;

use vfc::Vfc;

use super::input::InputManager;
use super::render_context::RenderContext;
use super::asset_loader::AssetLoader;

/// A selecton of services used to run the game.
///  
/// Each service is expected to be implemented and registered by whichever 
/// front-end uses the library.
pub struct ServiceContainer {
    asset_loader: Option<Box<dyn AssetLoader>>,
    render_context: Option<Box<dyn RenderContext>>,
    input_manager: Option<Box<dyn InputManager>>,
    vfc: Option<Box<Vfc>>,
}

impl Default for ServiceContainer {
    fn default() -> ServiceContainer {
        ServiceContainer {
            asset_loader: None,
            render_context: None,
            input_manager: None,
            vfc: None,
        }
    }
}

impl ServiceContainer {
    /// Constructs a `ServiceContainer` with no services registered.
    pub fn new() -> ServiceContainer {
        Self::default()
    }

    /// Registers an [`AssetLoader`] to the service container. If one has 
    /// already been registered, this function instead returns an arror.
    pub fn register_asset_loader(&mut self, asset_loader: Box<dyn AssetLoader>) -> Result<(), AlreadyRegisteredError> {
        if self.asset_loader.is_some() {
            return Err(AlreadyRegisteredError::new());
        }
        self.asset_loader = Some(asset_loader);
        Ok(())
    }
    /// Returns the [`AssetLoader`] that has been registered to the system. 
    /// If one has not yet been registered, this function instead returns 
    /// an error. 
    pub fn asset_loader_mut(&mut self) -> Result<&mut Box<dyn AssetLoader>, NotYetRegisteredError> {
        match &mut self.asset_loader {
            None => Err(NotYetRegisteredError::new()),
            Some(asset_loader) => Ok(asset_loader), 
        }
    }

    /// Registers a [`RenderContext`] to the service container. If one has 
    /// already been registered, this function instead returns an arror.
    pub fn register_render_context(&mut self, render_context: Box<dyn RenderContext>) -> Result<(), AlreadyRegisteredError> {
        if self.render_context.is_some() {
            return Err(AlreadyRegisteredError::new());
        }
        self.render_context = Some(render_context);
        Ok(())
    }
    /// Returns the [`RenderContext`] that has been registered to the system. 
    /// If one has not yet been registered, this function instead returns 
    /// an error. 
    pub fn render_context_mut(&mut self) -> Result<&mut Box<dyn RenderContext>, NotYetRegisteredError> {
        match &mut self.render_context {
            None => Err(NotYetRegisteredError::new()),
            Some(render_context) => Ok(render_context), 
        }
    }

    /// Registers an [`InputManager`] to the service container. If one has 
    /// already been registered, this function instead returns an arror.
    pub fn register_input_manager(&mut self, input_manager: Box<dyn InputManager>) -> Result<(), AlreadyRegisteredError> {
        if self.input_manager.is_some() {
            return Err(AlreadyRegisteredError::new());
        }
        self.input_manager = Some(input_manager);
        Ok(())
    }
    /// Returns the [`InputManager`] that has been registered to the system. 
    /// If one has not yet been registered, this function instead returns 
    /// an error. 
    pub fn input_manager_mut(&mut self) -> Result<&mut Box<dyn InputManager>, NotYetRegisteredError> {
        match &mut self.input_manager {
            None => Err(NotYetRegisteredError::new()),
            Some(input_manager) => Ok(input_manager), 
        }
    }

    /// Registers a [`Vfc`] to the service container. If one has 
    /// already been registered, this function instead returns an arror.
    pub fn register_vfc(&mut self, vfc: Box<Vfc>) -> Result<(), AlreadyRegisteredError> {
        if self.vfc.is_some() {
            return Err(AlreadyRegisteredError::new());
        }
        self.vfc = Some(vfc);
        Ok(())
    }
    /// Returns the [`Vfc`] that has been registered to the system. 
    /// If one has not yet been registered, this function instead returns 
    /// an error. 
    pub fn vfc_mut(&mut self) -> Result<&mut Box<Vfc>, NotYetRegisteredError> {
        match &mut self.vfc {
            None => Err(NotYetRegisteredError::new()),
            Some(vfc) => Ok(vfc), 
        }
    }
}

/// An error that arises when trying to register a service that has already 
/// been registered.
#[derive(Debug)]
pub struct AlreadyRegisteredError {}
impl AlreadyRegisteredError {
    fn new() -> Self {
        AlreadyRegisteredError {}
    }
}
impl Error for AlreadyRegisteredError {}
impl Display for AlreadyRegisteredError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// An error that arrives when requesting a service that has not yet been 
/// registered.
#[derive(Debug)]
pub struct NotYetRegisteredError {}
impl NotYetRegisteredError {
    fn new() -> Self {
        NotYetRegisteredError {}
    }
}
impl Error for NotYetRegisteredError {}
impl Display for NotYetRegisteredError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}