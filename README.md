# h23w_01-backend

23 冬ハッカソンチーム 01 バックエンド

## 環境変数

DB設定

名前 | 値
:-- | :--
`MYSQL_USER` | MySQLのユーザー名。Dockerイメージの`MYSQL_USER`と対応
`MYSQL_PASSWORD` | MySQLのユーザー パスワード。Dockerイメージの`MYSQL_PASSWORD`と対応
`MYSQL_HOSTNAME` | MySQLサーバーのホスト名。`localhost`など
`MYSQL_PORT` | MySQLサーバーのポート。`3306`など
`MYSQL_DATABASE` | MySQLサーバーのデータベース名。Dockerイメージの`MYSQL_DATABASE`と対応
`MIGRATION` | アプリ起動時に行うMigrationの設定。`up`, `down`, `refresh`, `none`のいずれか。デフォルトは`none`で何も行わない

※ `MYSQL_*`の環境変数が見つからなければ`NS_MARIADB_*`の環境変数も探索される。(NeoShowcase対応)

traQ BOTの設定

名前 | 値
:-- | :--
`BOT_ACCESS_TOKEN` | traQ BOTのAccess Token
`VERIFICATION_TOKEN` | traQ BOTのVerification Token

その他

名前 | 値
:-- | :--
`ALLOWED_ORIGINS` | CORSで`Access-Control-Allow-Origin`に含めるOriginのリスト。空白区切り
`ALLOW_CREDENTIALS` | (optional)CORSで`Access-Control-Allow-Credentials`に含める値。`true`または`false`, デフォルトは`true`
`ALLOWED_METHODS` | (optional)CORSで`Access-Control-Allow-Methods`に含めるHTTPメソッドのリスト。空白区切り
`ALLOWED_HEADERS` | (optional)CORSで`Access-Control-Allow-Headers`に含めるHTTPヘッダのリスト。空白区切り
`CHECK_AUTH` | 主要なエンドポイントで`X-Forwarded-User`によるユーザーの確認を行うかどうか。`true`または`false`

値の例は[`.env.dev`](./.env.dev)を参照

## docker compose

```sh
docker compose --env-file .env.dev up -d
```

このコマンドでバックエンドアプリが立ち上がる。ポートとコンテナの対応は以下の通り

- MariaDB `:3306`
- Adminer `:8080`
- アプリ `:8000`

## migration の手順

- up

```sh
DATABASE_URL=mysql://... sea-orm-cli migrate up
```

- down

```sh
DATABASE_URL=mysql://... sea-orm-cli migrate down
```
