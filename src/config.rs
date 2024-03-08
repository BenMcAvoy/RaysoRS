// Example URL: https://ray.so/#background=true&darkMode=true&padding=32&theme=candy&language=rust&code=cHJpbnRsbiEoIkhlbGxvLCB3b3JsZCIpOwo

use base64::{engine::general_purpose::URL_SAFE, Engine as _};

#[derive(Debug)]
pub struct RaysoConfig {
    background: bool,
    dark_mode: bool,
    padding: i32,
    theme: String,
    language: String,
    code: String,
    title: String,
}

#[derive(Default, Debug)]
pub struct RaysoConfigBuilder {
    background: Option<bool>,
    dark_mode: Option<bool>,
    padding: Option<i32>,
    theme: Option<String>,
    language: Option<String>,
    code: Option<String>,
    title: Option<String>,
}

impl RaysoConfigBuilder {
    pub fn background(mut self, background: bool) -> RaysoConfigBuilder {
        self.background = Some(background);
        self
    }

    pub fn dark_mode(mut self, dark_mode: bool) -> RaysoConfigBuilder {
        self.dark_mode = Some(dark_mode);
        self
    }

    pub fn padding(mut self, padding: i32) -> RaysoConfigBuilder {
        self.padding = Some(padding);
        self
    }

    pub fn theme(mut self, theme: String) -> RaysoConfigBuilder {
        self.theme = Some(theme);
        self
    }

    pub fn language(mut self, language: String) -> RaysoConfigBuilder {
        self.language = Some(language);
        self
    }

    pub fn code(mut self, code: String) -> RaysoConfigBuilder {
        let encoded = URL_SAFE.encode(code.as_bytes());
        self.code = Some(encoded);
        self
    }

    pub fn title(mut self, title: String) -> RaysoConfigBuilder {
        self.title = Some(title);
        self
    }

    pub fn build(self) -> RaysoConfig {
        let encoded = URL_SAFE.encode("fn main() { println!(\"Hello, world\"); }".as_bytes());

        RaysoConfig {
            background: self.background.unwrap_or(true),
            dark_mode: self.dark_mode.unwrap_or(true),
            padding: self.padding.unwrap_or(32),
            theme: self.theme.unwrap_or("candy".to_string()),
            language: self.language.unwrap_or("rust".to_string()),
            code: self.code.unwrap_or(encoded.to_string()),
            title: self.title.unwrap_or("Hello, rayso-rs".to_string()),
        }
    }
}

impl RaysoConfig {
    pub fn builder() -> RaysoConfigBuilder {
        RaysoConfigBuilder::default()
    }

    pub fn to_url(&self) -> String {
        let title = urlencoding::encode(&self.title);

        format!(
            "https://ray.so/#background={}&darkMode={}&padding={}&theme={}&language={}&code={}&title={}",
            self.background,
            self.dark_mode,
            self.padding,
            self.theme,
            self.language,
            self.code,
            title
        )
    }
}
