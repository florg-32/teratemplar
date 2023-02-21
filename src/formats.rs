use anyhow::Result;

pub fn parse_toml(data: String) -> Result<tera::Context> {
    let parsed = toml::from_str::<toml::Table>(data.as_str())?;
    Ok(tera::Context::from_serialize(parsed)?)
}