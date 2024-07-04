use reqwest::Client;

use crate::request::query::{
    CharacterQuery, ProducerQuery, Query, ReleaseQuery, StaffQuery, TagQuery, TraitQuery,
    UListQuery, VnQuery,
};
use crate::request::response::Response;

use crate::format::auth::AuthInfo;
use crate::format::stats::VndbStats;
use crate::format::user::{UserSearch, UserSearchFields};

use crate::format::character::Character;
use crate::format::producer::Producer;
use crate::format::release::Release;
use crate::format::rlist::RListPatch;
use crate::format::staff::Staff;
use crate::format::tag::Tag;
use crate::format::traits::Trait;
use crate::format::ulist::{UList, UListLabels, UListLabelsFieldChoices, UListPatch};
use crate::format::vn::VisualNovel;

const API_ENDPOINT: &str = "https://api.vndb.org/kana";

pub struct VndbApiClient {
    client: Client,
    access_token: String,
}

impl VndbApiClient {
    pub fn new(token: &String) -> Self {
        VndbApiClient {
            client: Client::new(),
            access_token: token.clone(),
        }
    }

    pub async fn get_stats(&self) -> Result<VndbStats, reqwest::Error> {
        let url = format!("{}/stats", API_ENDPOINT);
        let response = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<VndbStats>()
            .await?;
        Ok(response)
    }

    pub async fn get_auth_info(&self) -> Result<AuthInfo, reqwest::Error> {
        let url = format!("{}/authinfo", API_ENDPOINT);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("token {}", self.access_token))
            .send()
            .await?
            .json::<AuthInfo>()
            .await?;
        Ok(response)
    }

    pub async fn get_user(
        &self,
        q: &Vec<String>,
        fields: &UserSearchFields,
    ) -> Result<UserSearch, reqwest::Error> {
        let url = format!(
            "{}/user?q={}&fields={}",
            API_ENDPOINT,
            q.join("&q="),
            fields.to_csv()
        );
        let response = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<UserSearch>()
            .await?;
        Ok(response)
    }

    pub async fn vn_search(
        &self,
        q: &Query<VnQuery>,
    ) -> Result<Response<VisualNovel>, reqwest::Error> {
        let url = format!("{}/vn", API_ENDPOINT);
        let response = self
            .client
            .post(url)
            .json(q)
            .send()
            .await?
            .json::<Response<VisualNovel>>()
            .await?;
        Ok(response)
    }

    pub async fn release_search(
        &self,
        q: &Query<ReleaseQuery>,
    ) -> Result<Response<Release>, reqwest::Error> {
        let url = format!("{}/release", API_ENDPOINT);
        let response = self
            .client
            .post(url)
            .json(q)
            .send()
            .await?
            .json::<Response<Release>>()
            .await?;
        Ok(response)
    }

    pub async fn producer_search(
        &self,
        q: &Query<ProducerQuery>,
    ) -> Result<Response<Producer>, reqwest::Error> {
        let url = format!("{}/producer", API_ENDPOINT);
        let response = self
            .client
            .post(url)
            .json(q)
            .send()
            .await?
            .json::<Response<Producer>>()
            .await?;
        Ok(response)
    }

    pub async fn character_search(
        &self,
        q: &Query<CharacterQuery>,
    ) -> Result<Response<Character>, reqwest::Error> {
        let url = format!("{}/character", API_ENDPOINT);
        let response = self
            .client
            .post(url)
            .json(q)
            .send()
            .await?
            .json::<Response<Character>>()
            .await?;
        Ok(response)
    }

    pub async fn staff_search(
        &self,
        q: &Query<StaffQuery>,
    ) -> Result<Response<Staff>, reqwest::Error> {
        let url = format!("{}/staff", API_ENDPOINT);
        let response = self
            .client
            .post(url)
            .json(q)
            .send()
            .await?
            .json::<Response<Staff>>()
            .await?;
        Ok(response)
    }

    pub async fn tag_search(&self, q: &Query<TagQuery>) -> Result<Response<Tag>, reqwest::Error> {
        let url = format!("{}/tag", API_ENDPOINT);
        let response = self
            .client
            .post(url)
            .json(q)
            .send()
            .await?
            .json::<Response<Tag>>()
            .await?;
        Ok(response)
    }

    pub async fn trait_search(
        &self,
        q: &Query<TraitQuery>,
    ) -> Result<Response<Trait>, reqwest::Error> {
        let url = format!("{}/trait", API_ENDPOINT);
        let response = self
            .client
            .post(url)
            .json(q)
            .send()
            .await?
            .json::<Response<Trait>>()
            .await?;
        Ok(response)
    }

    pub async fn ulist(&self, q: &Query<UListQuery>) -> Result<Response<UList>, reqwest::Error> {
        let url = format!("{}/ulist", API_ENDPOINT);
        let response = self
            .client
            .post(url)
            .json(q)
            .send()
            .await?
            .json::<Response<UList>>()
            .await?;
        Ok(response)
    }

    pub async fn get_ulist_labels(
        &self,
        user: &String,
        fields: &UListLabelsFieldChoices,
    ) -> Result<UListLabels, reqwest::Error> {
        let url = format!(
            "{}/ulist_labels?user={}&fields={}",
            API_ENDPOINT,
            user.clone(),
            fields.to_csv()
        );
        let response = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<UListLabels>()
            .await?;
        Ok(response)
    }

    pub async fn ulist_patch(
        &self,
        vn_id: &String,
        patch: &UListPatch,
    ) -> Result<(), reqwest::Error> {
        let url = format!("{}/ulist/{}", API_ENDPOINT, vn_id);
        let _ = self
            .client
            .patch(&url)
            .header("Authorization", format!("token {}", self.access_token))
            .header("Content-Type", "application/json")
            .json(patch)
            .send()
            .await?;
        Ok(())
    }

    pub async fn rlist_patch(
        &self,
        r_id: &String,
        patch: &RListPatch,
    ) -> Result<(), reqwest::Error> {
        let url = format!("{}/rlist/{}", API_ENDPOINT, r_id);
        let _ = self
            .client
            .patch(&url)
            .header("Authorization", format!("token {}", self.access_token))
            .header("Content-Type", "application/json")
            .json(patch)
            .send()
            .await?;
        Ok(())
    }

    pub async fn ulist_remove(&self, vn_id: &String) -> Result<(), reqwest::Error> {
        let url = format!("{}/ulist/{}", API_ENDPOINT, vn_id);
        let _ = self
            .client
            .delete(&url)
            .header("Authorization", format!("token {}", self.access_token))
            .send()
            .await?;
        Ok(())
    }

    pub async fn rlist_remove(&self, r_id: &String) -> Result<(), reqwest::Error> {
        let url = format!("{}/rlist/{}", API_ENDPOINT, r_id);
        let _ = self
            .client
            .delete(&url)
            .header("Authorization", format!("token {}", self.access_token))
            .send()
            .await?;
        Ok(())
    }
}
