use itertools::Itertools;
use microkv::namespace::NamespaceMicroKV;
use std::fmt::{Debug, Formatter};

/// Track block number, based on microkv
/// The tracker will store data to microkv database.
/// The basic keys includes
///
/// name        | required | description
/// ---         | ---      | ---
/// key_raw     | Yes      | tracked key, user input, and all keys based on this
/// key_curt    |          | currently value, <key_raw>.current
/// key_finish  |          | finished value, <key_raw>.finish
/// key_running |          | this track is running, default is null means false, <key_raw>.running
/// key_next    |          | queuing for run, <key_raw>.next
/// key_skipped |          | skipped block number, maybe failed to execute. <key_raw>.skipped
///
/// Have 3 mode
/// 1. Normal
///    Will read next block one by one, read the next number based on <key_raw>.finish
/// 2. Fast mode (Not recommended)
///    Will read next block one by one, read the next number based on <key_raw>.curt, this will not have to wait for the update to <key_raw>.finish
/// 3. Parallel mode (Recommended)
///    Read next number based on <key_raw>.parallel.records last value, the length of <key_raw>.parallel.records will be controlled at <key_raw>.parallel.max default is 256
#[derive(Clone)]
pub struct Tracker {
    microkv: NamespaceMicroKV,
    key_raw: String,
    key_curt: String,
    key_finish: String,
    key_running: String,
    key_next: String,
    key_skipped: String,

    /// fast mode
    key_fast_mode: String,

    /// parallel enable
    key_parallel_enable: String,
    /// max records size, defualt is 256
    key_parallel_max: String,
    /// parallel records
    key_parallel_records: String,
}

impl Debug for Tracker {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Tracker {\n")?;
        f.write_str("  microkv: ***,\n")?;
        f.write_str(&format!("  key_raw: {}\n", self.key_raw))?;
        f.write_str(&format!("  key_curt: {}\n", self.key_curt))?;
        f.write_str(&format!("  key_finish: {}\n", self.key_finish))?;
        f.write_str(&format!("  key_running: {}\n", self.key_running))?;
        f.write_str(&format!("  key_next: {}\n", self.key_next))?;
        f.write_str(&format!("  key_skipped: {}\n", self.key_skipped))?;
        f.write_str(&format!("  key_fast_mode: {}\n", self.key_fast_mode))?;
        f.write_str(&format!(
            "  key_parallel_enable: {}\n",
            self.key_parallel_enable
        ))?;
        f.write_str(&format!("  key_parallel_max: {}\n", self.key_parallel_max))?;
        f.write_str(&format!(
            "  key_parallel_records: {}\n",
            self.key_parallel_records
        ))?;
        f.write_str("}")?;
        Ok(())
    }
}

impl Tracker {
    /// Create a tracker
    pub fn new(microkv: NamespaceMicroKV, key: impl AsRef<str>) -> Self {
        let key = key.as_ref();
        Self {
            microkv,
            key_raw: key.to_string(),
            key_curt: format!("{}.current", key),
            key_finish: format!("{}.finish", key),
            key_running: format!("{}.running", key),
            key_next: format!("{}.next", key),
            key_skipped: format!("{}.skipped", key),
            key_fast_mode: format!("{}.fast_mode", key),

            key_parallel_enable: format!("{}.parallel.enable", key),
            key_parallel_max: format!("{}.parallel.max", key),
            key_parallel_records: format!("{}.parallel.records", key),
        }
    }
}

impl Tracker {
    /// Read bool value by a key
    fn read_bool(&self, key: impl AsRef<str>) -> anyhow::Result<bool> {
        let value = self
            .microkv
            .get(key.as_ref())?
            .unwrap_or(serde_json::Value::Bool(false));
        if value.is_boolean() {
            return Ok(value.as_bool().unwrap_or(false));
        }
        let text = value.to_string().to_lowercase();
        Ok(&text == "true" || &text == "1")
    }

    /// Write Vec<usize> to microkv
    fn write_vec_usize(&self, key: impl AsRef<str>, values: &Vec<usize>) -> anyhow::Result<()> {
        if values.is_empty() {
            return Ok(());
        }
        let store_value: String = values.iter().join(",");
        self.microkv.put(key.as_ref(), &store_value)?;
        Ok(())
    }
}

// Parallel mode
impl Tracker {
    /// Update parallel records, when block finished, will remove by records
    pub fn update_parallel_records(&self, finish: usize) -> anyhow::Result<()> {
        if !self.is_enabled_parallel()? {
            return Ok(());
        }
        let records: Option<String> = self.microkv.get_as(&self.key_parallel_records)?;
        if let Some(records) = records {
            let mut parallel_records = parse_blocks_from_text(records)?;
            parallel_records.retain(|item| *item != finish);
            self.write_vec_usize(&self.key_parallel_records, &parallel_records)?;
        }
        Ok(())
    }

    /// Clear all parallel records
    pub fn clear_parallel_records(&self) -> anyhow::Result<()> {
        self.microkv.delete(&self.key_parallel_records)?;
        Ok(())
    }

    /// Set max parallel records
    pub fn set_parallel_max(&self, max: u64) -> anyhow::Result<()> {
        self.microkv.put(&self.key_parallel_max, &max)?;
        Ok(())
    }

    /// Read parallel records
    pub fn parallel_records(&self) -> anyhow::Result<Vec<usize>> {
        let records: Option<String> = self.microkv.get_as(&self.key_parallel_records)?;
        if let Some(record_text) = records {
            parse_blocks_from_text(record_text)
        } else {
            Ok(vec![])
        }
    }

    /// Enable parallel mode
    pub fn enable_parallel(&self) -> anyhow::Result<()> {
        if self.is_enabled_fast_mode()? {
            log::warn!("Parallel mode will replace fast mode");
        }
        self.microkv.put(&self.key_parallel_enable, &true)?;
        Ok(())
    }

    /// Disable parallel mode
    pub fn disable_parallel(&self) -> anyhow::Result<()> {
        self.microkv.put(&self.key_parallel_enable, &false)?;
        Ok(())
    }

    /// The parallel mode is enabled
    pub fn is_enabled_parallel(&self) -> anyhow::Result<bool> {
        self.read_bool(&self.key_parallel_enable)
    }
}

// fast mode
impl Tracker {
    /// Is enabled fast mode, key: *.fast_mode
    pub fn is_enabled_fast_mode(&self) -> anyhow::Result<bool> {
        self.read_bool(&self.key_fast_mode)
    }

    /// Enable fast mode
    /// WARNING: if parallel mode is enabled, there will be return a error
    pub fn enable_fast_mode(&self) -> anyhow::Result<()> {
        if self.is_enabled_parallel()? {
            anyhow::bail!("Parallel mode is enabled, can not enable fast mode.");
        }
        self.microkv
            .put(&self.key_fast_mode, &String::from("true"))?;
        Ok(())
    }
}

// read next
impl Tracker {
    /// Read next value by an entrypoint, key_curt or key_finish
    async fn _next_with_entrypoint(&self, key: impl AsRef<str>) -> anyhow::Result<usize> {
        let next: Option<String> = self.microkv.get_as(&self.key_next)?;
        if next.is_none() {
            let curt: usize = self.microkv.get_as(key.as_ref())?.unwrap_or(0);
            let next = curt + 1;
            self.microkv.put(&self.key_curt, &next)?;
            return Ok(next);
        }
        let mut plan_blocks = parse_blocks_from_text(next.unwrap())?;
        let next = *plan_blocks.first().unwrap_or(&1);
        if !plan_blocks.is_empty() {
            plan_blocks.remove(0);
            self.write_vec_usize(&self.key_next, &plan_blocks)?;
        }
        if plan_blocks.is_empty() {
            self.microkv.delete(&self.key_next)?;
        }
        self.microkv.put(&self.key_curt, &next)?;
        Ok(next)
    }

    /// Read next value use fast mode
    async fn next_fast_mode(&self) -> anyhow::Result<usize> {
        self._next_with_entrypoint(&self.key_curt).await
    }

    /// Read next value use normal mode
    async fn next_serial(&self) -> anyhow::Result<usize> {
        self._next_with_entrypoint(&self.key_finish).await
    }

    /// Read next Value use parallel mode
    async fn next_parallel(&self) -> anyhow::Result<usize> {
        let parallel_max = self
            .microkv
            .get(&self.key_parallel_max)?
            .map(|value| value.as_u64())
            .flatten()
            .unwrap_or(256);

        // if already have parallel records
        let mut parallel_records;
        let mut len;
        loop {
            let records: Option<String> = self.microkv.get_as(&self.key_parallel_records)?;
            // not have parallel records, read from serial
            if records.is_none() {
                let next = self.next_serial().await?;
                self.write_vec_usize(&self.key_parallel_records, &vec![next])?;
                return Ok(next);
            }
            parallel_records = parse_blocks_from_text(records.unwrap())?;
            len = parallel_records.len();
            if len < parallel_max as usize {
                break;
            }
            // The block being executed cannot exceed the maximum
            let secs = 10;
            tokio::time::sleep(std::time::Duration::from_secs(secs)).await;
            log::warn!(
                "The maximum value of parallel execution has been reached, wait {} seconds",
                secs
            );
        }

        let jump: Option<String> = self.microkv.get_as(&self.key_next)?;
        let next_num = if let Some(jumpstr) = jump {
            // jump to queue
            let mut plan_blocks = parse_blocks_from_text(jumpstr)?;
            let next = *plan_blocks.first().unwrap_or(&1);
            if !plan_blocks.is_empty() {
                plan_blocks.remove(0);
                self.write_vec_usize(&self.key_next, &plan_blocks)?;
            }
            if plan_blocks.is_empty() {
                self.microkv.delete(&self.key_next)?;
            }
            next
        } else {
            let last = parallel_records.get(len - 1).unwrap();
            *last + 1
        };

        // save current
        self.microkv.put(&self.key_curt, &next_num)?;
        // save parallel records
        let mut records_save = parallel_records.clone();
        records_save.push(next_num);
        self.write_vec_usize(&self.key_parallel_records, &records_save)?;
        Ok(next_num)
    }
}

impl Tracker {
    /// Track key is running, key: *.running
    pub fn is_running(&self) -> anyhow::Result<bool> {
        self.read_bool(&self.key_running)
    }

    pub fn stop_running(&self) -> anyhow::Result<()> {
        self.microkv
            .put(&self.key_running, &String::from("false"))?;
        Ok(())
    }

    pub fn start_running(&self) -> anyhow::Result<()> {
        self.microkv.put(&self.key_running, &String::from("true"))?;
        Ok(())
    }

    /// Update current value to finish value
    pub fn reset_current(&self) -> anyhow::Result<()> {
        let finish: Option<usize> = self.microkv.get_as(&self.key_finish)?;
        if let Some(finish) = finish {
            self.microkv.put(&self.key_curt, &finish)?;
        }
        Ok(())
    }

    pub fn flush_current(&self, to: usize) -> anyhow::Result<()> {
        self.microkv.put(&self.key_curt, &to)?;
        Ok(())
    }

    /// Read next value
    pub async fn next(&self) -> anyhow::Result<usize> {
        let is_running = self.is_running()?;
        if !is_running {
            loop {
                let secs = 3;
                log::warn!(
                    "The track [{}] isn't running (value is not `true`), wait {} seconds check again.",
                    &self.key_running,
                    secs
                );
                tokio::time::sleep(std::time::Duration::from_secs(secs)).await;
                if self.is_running()? {
                    break;
                }
            }
        }
        if self.is_enabled_parallel()? {
            return self.next_parallel().await;
        }
        if self.is_enabled_fast_mode()? {
            return self.next_fast_mode().await;
        }
        self.next_serial().await
    }

    /// When finished work, call this flush value.
    pub fn finish(&self, block: usize) -> anyhow::Result<()> {
        self.microkv.put(&self.key_finish, &block)?;
        self.update_parallel_records(block)?;
        Ok(())
    }

    pub fn skip(&self, block: usize) -> anyhow::Result<()> {
        let skipped: Option<String> = self.microkv.get_as(&self.key_skipped)?;
        match skipped {
            Some(v) => self
                .microkv
                .put(&self.key_skipped, &format!("{},{}", v, block))?,
            None => self.microkv.put(&self.key_skipped, &format!("{}", block))?,
        }
        self.finish(block)?;
        Ok(())
    }

    pub fn retry_skipped(&self) -> anyhow::Result<()> {
        let skipped: Option<String> = self.microkv.get_as(&self.key_skipped)?;
        if let Some(v) = skipped {
            let blocks = parse_blocks_from_text(v)?;
            self.jump_the_queue(blocks)?;
            self.microkv.delete(&self.key_skipped)?;
        }
        Ok(())
    }

    pub fn jump_the_queue(&self, mut blocks: Vec<usize>) -> anyhow::Result<()> {
        let curt: Option<usize> = self.microkv.get_as(&self.key_curt)?;
        if let Some(v) = curt {
            blocks.push(v);
        }
        let value: String = blocks.iter().join(",");
        self.microkv.put(&self.key_next, &value)?;
        Ok(())
    }

    pub fn queue(&self, blocks: Vec<usize>) -> anyhow::Result<()> {
        let value: String = blocks.iter().join(",");
        self.microkv.put(&self.key_next, &value)?;
        Ok(())
    }
}

fn parse_blocks_from_text(text: String) -> anyhow::Result<Vec<usize>> {
    let text = text.trim();
    let mut blocks = vec![];
    if text.starts_with('[') && text.ends_with(']') {
        blocks = serde_json::from_str(text)?;
    } else {
        let arrs = text.split(',').collect::<Vec<&str>>();
        for item in arrs {
            let item = item.trim();
            if let Ok(v) = item.parse::<usize>() {
                blocks.push(v);
            }
        }
    }
    Ok(blocks)
}
