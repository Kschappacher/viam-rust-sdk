#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveStraightRequest {
    /// Name of a base
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Desired travel distance in millimeters
    #[prost(int64, tag = "2")]
    pub distance_mm: i64,
    /// Desired travel velocity in millimeters/second
    #[prost(double, tag = "3")]
    pub mm_per_sec: f64,
    /// Whether the movement should block all other
    /// movement commands to base until this movement is complete
    #[prost(bool, tag = "4")]
    pub block: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveStraightResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveArcRequest {
    /// Name of a base
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Desired travel distance in millimeters
    #[prost(int64, tag = "2")]
    pub distance_mm: i64,
    /// Desired speed in millimeters per second
    #[prost(double, tag = "3")]
    pub mm_per_sec: f64,
    /// Desired angle in degrees
    #[prost(double, tag = "4")]
    pub angle_deg: f64,
    /// Whether the movement should block all other
    /// movement commands to base until this movement is complete
    #[prost(bool, tag = "5")]
    pub block: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveArcResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpinRequest {
    /// Name of a base
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Desired angle
    #[prost(double, tag = "2")]
    pub angle_deg: f64,
    /// Desired angular velocity
    #[prost(double, tag = "3")]
    pub degs_per_sec: f64,
    /// Whether the movement should block all other
    /// movement commands to base until this movement is complete
    #[prost(bool, tag = "4")]
    pub block: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpinResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopRequest {
    /// Name of a base
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopResponse {}
#[doc = r" Generated client implementations."]
pub mod base_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct BaseServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BaseServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> BaseServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> BaseServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            BaseServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " MoveStraight moves a robot's base in a straight line by a given distance, expressed in millimeters"]
        #[doc = " and a given speed, expressed in millimeters per second "]
        pub async fn move_straight(
            &mut self,
            request: impl tonic::IntoRequest<super::MoveStraightRequest>,
        ) -> Result<tonic::Response<super::MoveStraightResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.component.base.v1.BaseService/MoveStraight",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " MoveArc moves the robot's base in an arc by a given distance, expressed in millimeters,"]
        #[doc = " a given speed, expressed in millimeters per second of movement, and a given angle expressed in degrees"]
        pub async fn move_arc(
            &mut self,
            request: impl tonic::IntoRequest<super::MoveArcRequest>,
        ) -> Result<tonic::Response<super::MoveArcResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.component.base.v1.BaseService/MoveArc",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Spin spins a robot's base by an given angle, expressed in degrees, and a given "]
        #[doc = " angular speed, expressed in degrees per second"]
        pub async fn spin(
            &mut self,
            request: impl tonic::IntoRequest<super::SpinRequest>,
        ) -> Result<tonic::Response<super::SpinResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.component.base.v1.BaseService/Spin",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Stop stops a robot's base "]
        pub async fn stop(
            &mut self,
            request: impl tonic::IntoRequest<super::StopRequest>,
        ) -> Result<tonic::Response<super::StopResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.component.base.v1.BaseService/Stop",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod base_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with BaseServiceServer."]
    #[async_trait]
    pub trait BaseService: Send + Sync + 'static {
        #[doc = " MoveStraight moves a robot's base in a straight line by a given distance, expressed in millimeters"]
        #[doc = " and a given speed, expressed in millimeters per second "]
        async fn move_straight(
            &self,
            request: tonic::Request<super::MoveStraightRequest>,
        ) -> Result<tonic::Response<super::MoveStraightResponse>, tonic::Status>;
        #[doc = " MoveArc moves the robot's base in an arc by a given distance, expressed in millimeters,"]
        #[doc = " a given speed, expressed in millimeters per second of movement, and a given angle expressed in degrees"]
        async fn move_arc(
            &self,
            request: tonic::Request<super::MoveArcRequest>,
        ) -> Result<tonic::Response<super::MoveArcResponse>, tonic::Status>;
        #[doc = " Spin spins a robot's base by an given angle, expressed in degrees, and a given "]
        #[doc = " angular speed, expressed in degrees per second"]
        async fn spin(
            &self,
            request: tonic::Request<super::SpinRequest>,
        ) -> Result<tonic::Response<super::SpinResponse>, tonic::Status>;
        #[doc = " Stop stops a robot's base "]
        async fn stop(
            &self,
            request: tonic::Request<super::StopRequest>,
        ) -> Result<tonic::Response<super::StopResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct BaseServiceServer<T: BaseService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: BaseService> BaseServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        #[doc = r" Enable decompressing requests with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.accept_compression_encodings.enable_gzip();
            self
        }
        #[doc = r" Compress responses with `gzip`, if the client supports it."]
        pub fn send_gzip(mut self) -> Self {
            self.send_compression_encodings.enable_gzip();
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for BaseServiceServer<T>
    where
        T: BaseService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/proto.api.component.base.v1.BaseService/MoveStraight" => {
                    #[allow(non_camel_case_types)]
                    struct MoveStraightSvc<T: BaseService>(pub Arc<T>);
                    impl<T: BaseService> tonic::server::UnaryService<super::MoveStraightRequest>
                        for MoveStraightSvc<T>
                    {
                        type Response = super::MoveStraightResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MoveStraightRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).move_straight(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MoveStraightSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.api.component.base.v1.BaseService/MoveArc" => {
                    #[allow(non_camel_case_types)]
                    struct MoveArcSvc<T: BaseService>(pub Arc<T>);
                    impl<T: BaseService> tonic::server::UnaryService<super::MoveArcRequest> for MoveArcSvc<T> {
                        type Response = super::MoveArcResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MoveArcRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).move_arc(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MoveArcSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.api.component.base.v1.BaseService/Spin" => {
                    #[allow(non_camel_case_types)]
                    struct SpinSvc<T: BaseService>(pub Arc<T>);
                    impl<T: BaseService> tonic::server::UnaryService<super::SpinRequest> for SpinSvc<T> {
                        type Response = super::SpinResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SpinRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).spin(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SpinSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.api.component.base.v1.BaseService/Stop" => {
                    #[allow(non_camel_case_types)]
                    struct StopSvc<T: BaseService>(pub Arc<T>);
                    impl<T: BaseService> tonic::server::UnaryService<super::StopRequest> for StopSvc<T> {
                        type Response = super::StopResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StopRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).stop(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StopSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: BaseService> Clone for BaseServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: BaseService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: BaseService> tonic::transport::NamedService for BaseServiceServer<T> {
        const NAME: &'static str = "proto.api.component.base.v1.BaseService";
    }
}
