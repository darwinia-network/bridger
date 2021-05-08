use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct Resp<T>
where
	T: Serialize,
{
	err: i8,
	msg: Option<String>,
	data: Option<T>,
}

impl<T: Serialize> Resp<T> {
	pub fn to_json(&self) -> String {
		serde_json::to_string(&self).unwrap_or_else(|e| {
			let defresp: Resp<String> = Resp::err_with_msg(stringify!("{:?}", e));
			serde_json::to_string(&defresp).unwrap()
		})
	}
	pub fn ok() -> Resp<T> {
		Self {
			err: 0,
			msg: Some("ok".to_string()),
			data: None,
		}
	}
	pub fn ok_with_data(data: T) -> Resp<T> {
		Self {
			err: 0,
			msg: Some("ok".to_string()),
			data: Some(data),
		}
	}
	pub fn err() -> Resp<T> {
		Self {
			err: 1,
			msg: None,
			data: None,
		}
	}
	pub fn err_with_msg<S: AsRef<str>>(msg: S) -> Resp<T> {
		Self {
			err: 1,
			msg: Some(msg.as_ref().to_string()),
			data: None,
		}
	}
}
