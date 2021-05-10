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
	pub fn unwrap(self) -> T {
		if self.err == 1 {
			let a = &self.msg;
			panic!("{}", self.msg.unwrap_or("Response error".to_string()));
		}
		self.data.unwrap()
	}
	pub fn unwrap_or(self, def: T) -> T {
		if self.err == 1 {
			return def;
		}
		self.data.unwrap_or(def)
	}
	pub fn ok_or<E>(self, err: E) -> Result<T, E> {
		if self.err == 1 {
			return Err(err);
		}
		self.data.ok_or(err)
	}
	pub fn ok_or_else<E, F: FnOnce(Option<String>) -> E>(self, err: F) -> Result<T, E> {
		if self.err == 1 {
			return Err(err(self.msg));
		}
		self.data.ok_or_else(|| err(None))
	}
	pub fn safe_ok_or<E>(self, err: E) -> Result<Option<T>, E> {
		if self.err == 1 {
			return Err(err);
		}
		Ok(self.data)
	}
	pub fn safe_ok_or_else<E, F: FnOnce(Option<String>) -> E>(
		self,
		err: F,
	) -> Result<Option<T>, E> {
		if self.err == 1 {
			return Err(err(self.msg));
		}
		Ok(self.data)
	}
}

impl<T: Serialize> Resp<T> {
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
