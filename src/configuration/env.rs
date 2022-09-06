/*
 * Copyright 2022 ABSA Group Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

pub enum Env {
    Local,
}

impl Env {
    pub fn as_str(&self) -> &'static str {
        match self {
            Env::Local => "localhost",
        }
    }
}

impl TryFrom<String> for Env {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "localhost" => Ok(Self::Local),
            other => Err(format!(
                "{} is not a supported environment. The only supported environment is localhost.",
                other
            )),
        }
    }
}
