# <img alt="Hyperledger Aries logo" src="docs/aries-logo.png" width="45px" /> Aries Out-of-Band URL shortener
Shortener for URL-formatted Out-of-Band Aries messages in compliance with [Aries RFC0434](https://github.com/hyperledger/aries-rfcs/tree/main/features/0434-outofband).

## Quickstart
Run redis:
```
docker run --name redis -p 6379:6379 -d redis
```
Run shortener with localhost config:
```
APP_CONFIG=localhost cargo run
```
To run unit and integration tests:
```
APP_CONFIG=localhost cargo test --features "unit_test integration_test"
```

## API
This service listens on two separate preconfigured ports and exposes two distinct APIs on each:
### Internal API
`POST /api/internal/shorten-link`

Shortens given Out-of-Band message.

#### Body
  * **msg**
    * *description*: message to shorten
    * *required*: true
    * *type*: string
  * **base_url**
    * *description*: custom base URL to use for the shortened message
    * *default*: value preconfigured in `APPLICATION::SHORT_URL_BASE` config field
    * *required*: false
    * *type*: string
  * **expire_in_secs**
    * *description*: expiration time of the shortened URL in seconds
    * *default*: no expiration
    * *required*: false
    * *type*: positive integer

### External API
`GET /{msg_hash}`

Returns the content of the shortened message hashing to `msg_hash`, if it is not-expired and exists.

If a non-expired message is found and the request header does not contain `Content-Type: application/json`, a 308 Permanent Redirect is returned with Location header containing the message JSON encoded in base64.

If a non-expired message is found and the request header contains `Content-Type: application/json`, shortened message in the JSON format is returned.

If a non-expired message is not found, a 404 Not Found is returned.

---
    Copyright 2022 ABSA Group Limited
    
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at
    
        http://www.apache.org/licenses/LICENSE-2.0
    
    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
