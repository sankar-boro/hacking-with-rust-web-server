use crate::{resource::ResourceService, scope::Scope, service::{AppServiceFactory, HttpServiceFactory, ServiceFactoryWrapper}};

pub struct ServiceConfig {
  pub services:Vec<Box<dyn AppServiceFactory>>,
}

pub struct AppService {
  services: Vec<ResourceService>
}

impl AppService {
  pub fn new() -> Self {
    AppService {
      services: Vec::new(),
    }
  }

  pub fn service(&mut self, service: ResourceService) {
    self.services.push(service);
  }
}

impl ServiceConfig {
  pub fn new() -> Self {
    ServiceConfig {
      services: Vec::new(),
    }
  }
	
	pub fn service<T>(&mut self, factory: T) 
  where 
    T: HttpServiceFactory + 'static
  {
    self.services.push(Box::new(ServiceFactoryWrapper::new(factory)));
  }
}
