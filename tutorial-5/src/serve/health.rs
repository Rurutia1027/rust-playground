use crate::health;
use crate::health::{HealthCheckable, HealthStatus};
use chrono::{DateTime, Duration, Utc};
use std::sync::RwLock;

pub struct ServeHealth {
    last_cache_update: RwLock<Option<DateTime<Utc>>>,
    started_on: DateTime<Utc>,
}

impl ServeHealth {}

impl HealthCheckable for ServeHealth {
    // Cache is healthy if we have seen an update in the last five minutes, or it has been less
    // than 5 minutes since the server started.
    fn health_status(&self) -> HealthStatus {
        let now = Utc::now();
        let last_update = self
            .last_cache_update
            .read()
            .unwrap()
            .unwrap_or(self.started_on);

        let time_since_last_update = now - last_update;

        if time_since_last_update < Duration::minutes(5) {
            HealthStatus::Healthy
        } else {
            HealthStatus::Unhealthy(Some(format!(
                "cache has not been update in {} seconds",
                time_since_last_update.num_seconds()
            )))
        }
    }
}

// todo: here i want to modify this function into a scheduler
// todo: it schedule 5 seconds fetch data to the

// 所以这个地方它指定的 cluster 或者是 server 端的 health 健康标准到底是什么哦?
// 我希望能够实现的功能是, 每隔 5 秒钟, 采集一次数据, 然后将其写到 db 库中,
// 同时对外提供查询接口增加一个 支持 start-end 时间段的,
// 然后, 将这个时间段的数据健康指标返回回来, 并且前端增加一个跳转页面, 展示查询时间的健康统计指标数据信息.
