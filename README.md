# h23w_01-backend

23 冬ハッカソンチーム 01 バックエンド

## 環境変数

名前 | 値
:-- | :--
`BOT_ACCESS_TOKEN` | traQ BOTのAccess Token
`VERIFICATION_TOKEN` | traQ BOTのVerification Token
`ALLOWED_ORIGINS` | CORSで`Access-Control-Allow-Origin`に含めるOriginのリスト。空白区切り
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
