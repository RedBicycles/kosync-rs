Add routes to match current kosync spec
https://github.com/koreader/koreader/blob/master/plugins/kosync.koplugin/api.json

Add DB code
https://github.com/launchbadge/sqlx
https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md#enable-building-in-offline-mode-with-query

Make it work

Test it

Make it look good
https://docs.rs/eyre/latest/eyre/

Current implementation
https://github.com/koreader/koreader-sync-server
https://github.com/jberlyn/kosync-dotnet/blob/main/Controllers/SyncController.cs

https://github.com/koreader/koreader/tree/master/plugins/kosync.koplugin

docker run --rm --name pg -p 5432:5432  -e POSTGRES_PASSWORD=kosync postgres
sqlx database create
sqlx migrate add users

Add Tracing 
https://tokio.rs/tokio/topics/tracing