use serde::Deserialize;
use tabled::Tabled;

use crate::util::deserialize::null_to_default;

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct Jobs {
    pub jobs: Vec<Job>,
}

#[derive(Tabled, Default, Deserialize)]
#[serde(default)]
pub struct RemoteJob {
    #[serde(deserialize_with = "null_to_default")]
    pub details: String,
    #[serde(deserialize_with = "null_to_default")]
    pub job_id: String,
    #[serde(deserialize_with = "null_to_default")]
    pub status: String,
    #[serde(deserialize_with = "null_to_default")]
    pub step_description: String,
    #[serde(deserialize_with = "null_to_default")]
    pub step_number: u32,
    #[serde(deserialize_with = "null_to_default")]
    pub system_id: u32,
    #[serde(deserialize_with = "null_to_default")]
    pub total_steps: u32,
}

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct Job {
    #[serde(deserialize_with = "null_to_default")]
    pub details: String,
    #[serde(deserialize_with = "null_to_default")]
    pub id: String,
    pub remote_jobs: Vec<RemoteJob>,
    #[serde(deserialize_with = "null_to_default")]
    pub status: String,
    #[serde(deserialize_with = "null_to_default")]
    pub step_description: String,
    #[serde(deserialize_with = "null_to_default")]
    pub step_number: u32,
    #[serde(deserialize_with = "null_to_default")]
    pub total_steps: u32,
    #[serde(deserialize_with = "null_to_default")]
    pub _type: String,
}

impl Tabled for Job {
    const LENGTH: usize = 50;

    fn fields(&self) -> Vec<String> {
        let mut remote_jobs = vec![];
        for j in self.remote_jobs.iter() {
            let tmp = format!(
                "details          {}
job_id           {}
status           {}
step_description {}
step_number      {}
system_id        {}
total_steps      {}
--",
                j.details,
                j.job_id,
                j.status,
                j.step_description,
                j.step_number,
                j.system_id,
                j.total_steps,
            );
            remote_jobs.push(tmp);
        }
        vec![
            self.details.to_owned(),
            self.id.to_owned(),
            self.status.to_owned(),
            self.step_description.to_owned(),
            self.step_number.to_string(),
            self.total_steps.to_string(),
            self._type.to_owned(),
            remote_jobs.join("\n"),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "details".to_string(),
            "id".to_string(),
            "status".to_string(),
            "step_description".to_string(),
            "step_number".to_string(),
            "total_steps".to_string(),
            "type".to_string(),
            "remote_jobs".to_string(),
        ]
    }
}
