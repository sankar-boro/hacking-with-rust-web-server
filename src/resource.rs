use std::{
  pin::Pin, 
  rc::Rc,
  cell::RefCell,
  task::{Context, Poll},
};

use crate::{config::AppService, route::{
    BoxedRouteService, 
    Route, 
    RouteFutureService
  }, service::{
    ServiceRequest, 
    ServiceResponse,
    AppServiceFactory,
  }};
use async_std::task::block_on;
use futures::{Future, FutureExt};
use loony_service::{ServiceFactory, Service};

pub struct Resource {
  prefix: String,
  route: Route,
  route_service: Rc<RefCell<Option<ResourceService>>>
}

impl Resource {
  pub fn new(prefix: String) -> Self {
    Resource {
      prefix,
      route: Route::new(""),
      route_service: Rc::new(RefCell::new(None)),
    }
  }

  pub fn route(mut self, route: Route) -> Self {
    self.route = route;
    self
  }
}

impl ServiceFactory for Resource {
    type Request = ServiceRequest;
    type Response = ServiceResponse;
    type Error = ();
    type Service = ResourceService;
    type Future = CreateResourceService;
    type InitError = ();
    type Config = ();
 
    fn new_service(&self, _: ()) -> Self::Future {
        let mut path = self.prefix.clone();
        path.push_str(&self.route.path);
        let fut = self.route.new_service(());
        CreateResourceService {
          len: path.len() as u16,
          path,
          fut,
        }
    }
}

impl AppServiceFactory for Resource {
  fn register(&mut self, config: &mut AppService) {
    let a = self.new_service(());
    let b = block_on(a).unwrap();
    config.service(b);
  }
}

#[pin_project::pin_project]
pub struct CreateResourceService {
    #[pin]
    pub fut: RouteFutureService,
    pub path: String,
    pub len: u16,
}
pub struct ResourceService {
    pub service: BoxedRouteService,
    pub path: String,
    pub len: u16,
}

impl Service for ResourceService {
    type Request = ServiceRequest;
    type Response = ServiceResponse;
    type Error = ();
    type Future = Pin<Box<dyn Future<Output=Result<ServiceResponse, ()>>>>;

    fn call(&mut self, req: Self::Request) -> Self::Future {
        self.service.call(req).boxed_local()
    }
}

impl Future for CreateResourceService {
    type Output = Result<ResourceService, ()>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.fut.fut.as_mut().poll(cx) {
          Poll::Ready(service) => {
            let a = Poll::Ready(Ok(ResourceService {
                service: service.unwrap(),
                path: self.path.clone(),
                len: self.len,
            }));
            return a;
          },
          Poll::Pending => Poll::Pending
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::extensions::Extensions;
    use crate::request::HttpRequest;
    use crate::route::Route;
    use crate::service::AppServiceFactory;
    use crate::service::ServiceRequest;
    use crate::resource::Resource;
    use loony_service::Service;
    use std::rc::Rc;
    use crate::config::AppService;

    async fn index(_: String) -> String {
        "Hello World!".to_string()
    }

    #[test]
    fn resource() {
      let r = Route::new("/home");
      let r = r.route(index);
      let rs = Resource::new("".to_string());
      let mut rs = rs.route(r);
      let mut a_ser = AppService::new();
      rs.register(&mut a_ser);
      let sr = ServiceRequest(HttpRequest { url: "/home".to_string(), extensions: Rc::new(Extensions::new()) });
      let mut a= rs.route_service.borrow_mut();
      if let Some(mut c) = a.take() {
        c.call(sr);
      }
    }
}