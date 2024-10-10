use crate::{
    get_app_orm, BMBP_CURRENT_ORM, BMBP_CURRENT_USER, BMBP_IGNORE_AUTH_VALID, BMBP_WHITE_URLS,
};
use bmbp_auth::BmbpAuthTokenUtil;
use bmbp_ctx_vars::BMBP_CONTEXT_VARS;
use bmbp_http_type::{BmbpResp, BmbpRespErr};
use salvo::routing::{PathFilter, PathState};
use salvo::{handler, Depot, FlowCtrl, Request, Response};

#[handler]
pub async fn auth_token_middle(
    req: &mut Request,
    depot: &mut Depot,
    _resp: &mut Response,
    ctrl: &mut FlowCtrl,
) -> BmbpResp<()> {
    let white_urls = (&*BMBP_CONTEXT_VARS).get_value::<String>(BMBP_WHITE_URLS.to_string());
    let white_url_list: Vec<String> = white_urls.split(",").map(|s| s.to_string()).collect();
    let req_url = req.uri().path();
    let mut req_path_state = PathState::new(req_url);
    for white_url in white_url_list {
        let white_path = PathFilter::new(white_url);
        if white_path.detect(&mut req_path_state) {
            depot.insert(BMBP_IGNORE_AUTH_VALID, true);
            return Ok(());
        }
    }
    return if let Some(token) = req.header::<String>("Authorization") {
        let token = token.replace("Bearer ", "");
        match BmbpAuthTokenUtil::check_token(token).await {
            Ok(token_user) => {
                if let Some(b) = token_user {
                    if b {
                        Ok(())
                    } else {
                        ctrl.skip_rest();
                        Err(BmbpRespErr::err(
                            Some("AUTH".to_string()),
                            Some("Token校验失败[token无效]".to_string()),
                        ))
                    }
                } else {
                    ctrl.skip_rest();
                    Err(BmbpRespErr::err(
                        Some("AUTH".to_string()),
                        Some("Token校验失败[未取到校验结果]".to_string()),
                    ))
                }
            }
            Err(err) => {
                let msg = if let Some(msg) = err.get_msg() {
                    msg.clone()
                } else {
                    "".to_string()
                };
                ctrl.skip_rest();
                Err(BmbpRespErr::err(
                    Some("AUTH".to_string()),
                    Some(format!("Token校验失败[{}]", msg)),
                ))
            }
        }
    } else {
        ctrl.skip_rest();
        Err(BmbpRespErr::err(
            Some("AUTH".to_string()),
            Some("未登录".to_string()),
        ))
    };
}

#[handler]
pub async fn auth_user_middle(
    req: &mut Request,
    depot: &mut Depot,
    ctrl: &mut FlowCtrl,
) -> BmbpResp<()> {
    if let Some(token) = req.header::<String>("Authorization") {
        let token = token.replace("Bearer ", "");
        match BmbpAuthTokenUtil::get_token_user(token).await {
            Ok(token_user) => {
                if let Some(user) = token_user {
                    depot.insert(BMBP_CURRENT_USER, user);
                }
            }
            Err(err) => {
                let ignore_auth_valid = match depot.get::<bool>(BMBP_IGNORE_AUTH_VALID) {
                    Ok(b) => b.clone(),
                    Err(_) => false,
                };
                let msg = if let Some(msg) = err.get_msg() {
                    msg.clone()
                } else {
                    "获取用户信息失败".to_string()
                };
                // 忽略认证
                return if ignore_auth_valid {
                    Ok(())
                } else {
                    ctrl.skip_rest();
                    Err(BmbpRespErr::err(
                        Some("AUTH".to_string()),
                        Some(msg.to_string()),
                    ))
                };
            }
        }
    }
    Ok(())
}

#[handler]
pub async fn orm_middle(depot: &mut Depot) {
    if let Some(orm) = get_app_orm() {
        depot.insert(BMBP_CURRENT_ORM, orm);
    }
}
