use crate::health::{HealthCheckable, HealthStatus};
use chrono::{DateTime, Duration, TimeDelta, Utc};
use core::time;
use enum_iterator::last;
use std::sync::RwLock;
use std::thread;

pub struct ServeHealth {
    health_duration_in_seconds: RwLock<Option<u32>>,
    last_cache_update: RwLock<Option<DateTime<Utc>>>,
    started_on: DateTime<Utc>,
}

impl ServeHealth {
    pub fn new(started_on: DateTime<Utc>) -> Self {
        Self {
            health_duration_in_seconds: RwLock::new(Some(5)),
            last_cache_update: RwLock::new(None),
            started_on,
        }
    }

    pub fn set_cache_updated(&self) {
        *self.last_cache_update.write().unwrap() = Some(Utc::now())
    }

    pub fn set_health_duration_in_seconds(&self, duration: u32) {
        assert!(duration >= 0);
        *self.health_duration_in_seconds.write().unwrap() = Some(duration);
    }
}

impl HealthCheckable for ServeHealth {
    fn get_since_last_update(&self) -> DateTime<Utc> {
        let last_update = self
            .last_cache_update
            .read()
            .unwrap()
            .unwrap_or(self.started_on);
        last_update
    }

    // Cache is healthy if we have seen an update in the last five minutes, or it has been less
    // than 5 minutes since the server started.
    fn health_status(&self) -> HealthStatus {
        let time_since_last_update = Utc::now() - self.get_since_last_update();
        let health_duration_in_secs = self
            .health_duration_in_seconds
            .read()
            .unwrap()
            .unwrap_or(5 * 60) as i64;

        if time_since_last_update < Duration::seconds(health_duration_in_secs) {
            HealthStatus::Healthy
        } else {
            HealthStatus::Unhealthy(Some(format!(
                "cache has not been update in {} seconds",
                time_since_last_update.num_seconds()
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use chrono::Timelike;

    use super::*;
    #[tokio::test]
    async fn test_create_health_status() {
        let healthy_status = HealthStatus::Healthy;
        let un_healthy_status = HealthStatus::Unhealthy(Option::None);
        assert_eq!(healthy_status, HealthStatus::Healthy);
        assert_eq!(un_healthy_status, HealthStatus::Unhealthy(Option::None));
    }

    #[tokio::test]
    async fn test_health_checkable_logic() {
        let started_on = chrono::Utc::now();
        let server_health = ServeHealth::new(started_on);
        let health_status = server_health.health_status();
        assert_eq!(health_status, HealthStatus::Healthy);

        // here we set the duration threashold to 3 seconds which means
        // if update duration >= 3 seconds, the health status will become UnHealth

        server_health.set_health_duration_in_seconds(3);
        server_health.set_cache_updated();
        // let thread sleep for more than 3 seconds
        thread::sleep(Duration::from_secs(4));
        // here we get the health_stauts and it should be in unhealth status
        let health_status = server_health.health_status();
        assert_ne!(HealthStatus::Healthy, health_status);
        let cache_since_last_update = server_health.get_since_last_update();
        let expected_health_status = HealthStatus::Unhealthy(Some(format!(
            "cache has not been update in {} seconds",
            cache_since_last_update.second()
        )));

        // assert_eq!(expected_health_status, health_status);

        println!(
            "expected_health_status: {:?}, health_status: {:?}",
            expected_health_status, health_status
        );
    }
}

// todo: here i want to modify this function into a scheduler
// todo: it schedule 5 seconds fetch data to the

// 所以这个地方它指定的 cluster 或者是 server 端的 health 健康标准到底是什么哦?
// 我希望能够实现的功能是, 每隔 5 秒钟, 采集一次数据, 然后将其写到 db 库中,
// 同时对外提供查询接口增加一个 支持 start-end 时间段的,
// 然后, 将这个时间段的数据健康指标返回回来, 并且前端增加一个跳转页面, 展示查询时间的健康统计指标数据信息.
