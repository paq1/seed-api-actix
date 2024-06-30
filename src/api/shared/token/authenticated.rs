use actix_web::HttpRequest;

use crate::api::shared::token::JwtTokenService;
use crate::api::shared::token::jwt_claims::JwtClaims;
use crate::core::shared::context::Context;
use crate::core::shared::token::TokenService;
use crate::models::shared::errors::Error;

pub fn authenticated(
    req: &HttpRequest,
    jwt_token_service: &JwtTokenService
) -> Result<Context, Error> {
    let maybe_authorization = req.headers().get("Authorization");
    match maybe_authorization {
        Some(bearer_header_value) => {

            let bearer_str = bearer_header_value
                .to_str()
                .map_err(|err| Error::new(
                        err.to_string(),
                        "00TOKPA".to_string(),
                        vec![],
                        None
                    )
                )?;

            let jwt = *bearer_str
                .split(" ")
                .collect::<Vec<&str>>()
                .get(1)
                .unwrap_or(&"");

            jwt_token_service
                .decode::<JwtClaims>(jwt)
                .map_err(|err| Error::new(
                        err,
                        "".to_string(),
                        vec![],
                        Some(401)
                    )
                )
                .map(|claims| claims.into())
        }
        _ => Err(
            Error::new(
                "Unauthorized, pas de token d'authentification".to_string(),
                "00MTOKE".to_string(),
                vec![],
                Some(401)
            )
        )
    }
}