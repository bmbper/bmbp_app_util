use std::sync::{ LazyLock, RwLock};
use bmbp_rdbc_orm::RdbcOrm;

static BMBP_ORM_CONTEXT: LazyLock<RwLock<Option<&'static RdbcOrm>>> = LazyLock::new(|| {
    RwLock::new(None)
});

pub fn register_app_orm(orm: &'static RdbcOrm) {
    *((&*BMBP_ORM_CONTEXT).write().unwrap()) = Some(orm)
}

pub fn get_app_orm() -> Option<&'static RdbcOrm> {
    if let Some(orm) = (&*BMBP_ORM_CONTEXT).read().unwrap().clone() {
        return Some(orm);
    }
    None
}
