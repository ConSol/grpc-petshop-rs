use self::grpc::{
    authentication_server::{Authentication, AuthenticationServer},
    pet_shop_server::{PetShop, PetShopServer},
    BuyPetReply, BuyPetRequest, GetPetsReply, GetPetsRequest, LoginReply, LoginRequest,
    LogoutReply, LogoutRequest, Pet,
};
use crate::PetDb;
use async_trait::async_trait;
use tonic::Result;
use tonic::{Request, Response, Status};

/// contains generated gRPC code
mod grpc {
    tonic::include_proto!("auth.v1");
    tonic::include_proto!("shop.v1");
}

/// creates auth server
pub fn auth() -> AuthenticationServer<AuthenticationService> {
    AuthenticationServer::new(AuthenticationService)
}

/// creates shop server
pub fn shop(db: PetDb) -> PetShopServer<ShopService> {
    PetShopServer::new(ShopService { db })
}

/// top secret user token returned, after login
const TOKEN: &'static str = "secrettoken";

/// meta key used for auth
const AUTH_META: &'static str = "authentication";

#[derive(Clone)]
pub struct AuthenticationService;

#[async_trait]
impl Authentication for AuthenticationService {
    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<LoginReply>> {
        let request = request.into_inner();
        if request.email == "user@email.com" && request.password == "password" {
            Ok(Response::new(LoginReply {
                usertoken: TOKEN.to_owned(),
            }))
        } else {
            Err(Status::unauthenticated("invalid credentials"))
        }
    }

    async fn logout(&self, request: Request<LogoutRequest>) -> Result<Response<LogoutReply>> {
        check_auth_meta(&request)?;

        Ok(Response::new(LogoutReply {}))
    }
}

#[derive(Clone)]
pub struct ShopService {
    db: PetDb,
}

#[async_trait]
impl PetShop for ShopService {
    async fn get_pets(&self, request: Request<GetPetsRequest>) -> Result<Response<GetPetsReply>> {
        check_auth_meta(&request)?;

        let data = self.db.data.lock().await;
        Ok(Response::new(GetPetsReply {
            pets: data
                .iter()
                .map(|pet| Pet {
                    id: pet.id,
                    age: pet.age,
                    name: pet.name.clone(),
                })
                .collect(),
        }))
    }

    async fn buy_pet(&self, request: Request<BuyPetRequest>) -> Result<Response<BuyPetReply>> {
        check_auth_meta(&request)?;

        let mut data = self.db.data.lock().await;
        data.retain(|pet| pet.id != request.get_ref().id);

        Ok(Response::new(BuyPetReply {}))
    }
}

/// checks whether request has correct auth meta set
fn check_auth_meta<T>(request: &Request<T>) -> Result<()> {
    let meta = request.metadata();
    if let Some(authentication) = meta.get(AUTH_META) {
        if authentication == format!("Bearer {TOKEN}").as_str() {
            Ok(())
        } else {
            Err(Status::unauthenticated("bad authorization token"))
        }
    } else {
        Err(Status::unauthenticated("not authorization meta given"))
    }
}
