use crate::user_info::web::web_get_user_id;

mod web;

pub struct UserInfo;

impl UserInfo {
    pub fn get_user_id() -> String {
        #[cfg(target_arch = "wasm32")]
        {
            return web_get_user_id();
        }

        todo!()
    }
}
