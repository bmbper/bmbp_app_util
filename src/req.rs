use bmbp_auth::{ BmbpUser};
use bmbp_http_type::{BmbpResp, BmbpRespErr};
use bmbp_rdbc_orm::RdbcOrm;
use salvo::Depot;
use crate::{BMBP_CURRENT_ORM, BMBP_CURRENT_USER};

pub fn parse_user_orm(depot: &mut Depot) -> (Option<&BmbpUser>, Option<&RdbcOrm>) {
    let mut current_user = None;
    if let Ok(user) = depot.get::<BmbpUser>(BMBP_CURRENT_USER) {
        current_user = Some(user);
    }
    let mut current_orm = None;
    if let Ok(orm) = depot.get::<&RdbcOrm>(BMBP_CURRENT_ORM) {
        current_orm = Some(*orm);
    }
    (current_user, current_orm)
}

pub fn parse_user(depot: &mut Depot) -> Option<&BmbpUser> {
    if let Ok(user) = depot.get::<BmbpUser>(BMBP_CURRENT_USER) {
        return Some(user);
    }
    None
}
pub fn parse_orm(depot: &mut Depot) -> BmbpResp<&RdbcOrm> {
    if let Ok(orm) = depot.get::<&RdbcOrm>(BMBP_CURRENT_ORM) {
        return Ok(orm);
    }
    Err(BmbpRespErr::err(Some("DB".to_string()), Some("请先连接数据库".to_string())))
}
pub fn valid_orm(orm: Option<&RdbcOrm>) -> BmbpResp<bool> {
    if orm.is_none() {
        return Err(BmbpRespErr::err(Some("DB".to_string()), Some("请先连接数据库".to_string())));
    }
    Ok(true)
}
pub fn valid_user(user: Option<&BmbpUser>) -> BmbpResp<bool> {
    if user.is_none() {
        return Err(BmbpRespErr::err(Some("AUTH".to_string()), Some("请先登录".to_string())));
    }
    Ok(true)
}