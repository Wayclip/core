#!/usr/bin/env zsh
# Okay this is AI generated because hard...
set -euo pipefail

URL="https://discord.com/api/v10/applications/detectable"
OUT_FILE="games.json"

curl -sSL -f -H "User-Agent: Mozilla/5.0" "$URL" | jq -c '
  ["game", "client", "launcher", "start", "main", "patcher", "updater", "crashreporter", "unitycrashhandler", "unrealcefsubprocess"] as $blacklist |
  [
    .[] | . as $app |
    ($app.third_party_skus // [] | map(select(.distributor == "steam") | .id | tonumber?)[0] // null) as $steam_appid |
    ($app.themes // [] | map(ascii_downcase | gsub("[^a-z0-9]+"; "_") | sub("^_+"; "") | sub("_+$"; ""))) as $tags |
    (
      if $app.icon != null then
        "https://cdn.discordapp.com/app-icons/" + $app.id + "/" + $app.icon + ".png?size=256"
      elif $steam_appid != null then
        "https://shared.cloudflare.steamstatic.com/store_item_assets/steam/apps/" + ($steam_appid|tostring) + "/capsule_231x87.jpg"
      else
        null
      end
    ) as $icon_url |
    (
      if $steam_appid != null then
        "https://shared.cloudflare.steamstatic.com/store_item_assets/steam/apps/" + ($steam_appid|tostring) + "/header.jpg"
      elif $app.cover_image != null then
        "https://cdn.discordapp.com/app-icons/" + $app.id + "/" + $app.cover_image + ".png?size=1024"
      elif $app.icon != null then
        "https://cdn.discordapp.com/app-icons/" + $app.id + "/" + $app.icon + ".png?size=512"
      else
        null
      end
    ) as $cover_url |
    ($app.executables // [])[] |
    select(.is_launcher != true) |
    (.name | gsub("\\\\"; "/") | split("/") | last | ascii_downcase | sub("\\.exe$"; "")) as $exe |
    select(($exe | length) >= 4) |
    select(($blacklist | index($exe)) == null) |
    {
      app_id: $app.id,
      app_name: $app.name,
      exe: $exe,
      steam_appid: $steam_appid,
      tags: $tags,
      icon_url: $icon_url,
      cover_url: $cover_url
    }
  ] |
  group_by(.exe) |
  map(select((map(.app_id) | unique | length) == 1)[0]) |
  group_by(.app_id) |
  map({
    slug: (.[0].app_name | ascii_downcase | gsub("[^a-z0-9]+"; "_") | sub("^_+"; "") | sub("_+$"; "")),
    name: (.[0].app_name | sub("^\\s+"; "") | sub("\\s+$"; "")),
    steam_appid: .[0].steam_appid,
    icon_url: .[0].icon_url,
    cover_url: .[0].cover_url,
    tags: (.[0].tags // []),
    exes: [.[].exe] | unique | sort
  })
' > "$OUT_FILE"

COUNT=$(jq 'length' "$OUT_FILE")
echo "Saved $COUNT games to $OUT_FILE"
