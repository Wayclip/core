#!/usr/bin/env zsh
set -euo pipefail

URL="https://discord.com/api/v10/applications/detectable"
OUT_FILE="games.json"
TMP_FILE="${OUT_FILE}.tmp"

trap 'rm -f "$TMP_FILE"' EXIT INT TERM

curl -sSL -f --compressed \
  -H "Accept: application/json" \
  -H "User-Agent: Wayclip-core/0.0.1 (+https://github.com/Wayclip/core)" \
  "$URL" | jq -c '
  ["game", "client", "launcher", "start", "main", "patcher", "updater", "crashreporter",
   "unitycrashhandler", "unitycrashhandler64", "unrealcefsubprocess",
   "helper", "service", "server", "editor", "dev", "debug", "sandbox",
   "cef", "chrome", "gpu", "renderer", "zygote", "broker", "sandboxbox",
   "installer", "setup", "bootstrapper", "bootstrap", "runtime", "redist",
   "dotnet", "dxweb", "vcredist", "prereq",
   "run", "play", "app", "build", "test", "exec", "launch", "runner", "starter",
   "default", "command", "cleaner", "remove", "application", "appdata", "app_main",
   "autorun", "autostarter", "frontend", "standalone", "standalonewindows64",
   "java", "javaw", "dolphin", "aseprite", "krita", "mudlet", "busybox64",
   "windows", "windows64", "windows-64bit", "utilities", "downloader"] as $blacklist |

  ["unity", "unreal", "engine", "launcher", "patcher", "updater",
   "installer", "setup", "bootstrap", "crash", "report", "helper",
   "service", "server", "editor", "debug", "cef", "chrome",
   "gpu", "renderer", "zygote", "broker", "sandbox", "runtime",
   "redist", "vcredist", "dxweb", "dotnet", "prereq",
   "config", "setting", "install", "benchmark", "manager", "tool",
   "util", "test", "loader", "physx", "emulator", "sample",
   "example", "prototype", "preprocessor"] as $pattern_blacklist |

  [
    .[] | select(. != null and type == "object" and .name != null and (.name | length) > 0) | . as $app |
    ($app.id | tostring) as $app_id |
    ($app.icon // $app.icon_hash // null) as $icon_hash |
    ($app.cover_image // $app.cover_image_hash // null) as $cover_image_hash |
    ($app.third_party_skus // [] | map(
      select(.distributor == "steam" and (.id != null or .sku != null)) |
      (.id // .sku | tostring | tonumber?)
    )[0] // null) as $steam_appid |
    ($app.themes // $app.tags // [] | map(
      (if type == "object" then (.name // .slug // "") elif type == "string" then . else "" end) |
      select(length > 0) |
      ascii_downcase | gsub("[^a-z0-9]+"; "_") | sub("^_+"; "") | sub("_+$"; "")
    )) as $tags |
    (
      if $icon_hash != null then
        "https://cdn.discordapp.com/app-icons/" + $app_id + "/" + $icon_hash + ".png?size=256"
      else
        null
      end
    ) as $icon_url |
    (
      if $steam_appid != null then
        "https://shared.cloudflare.steamstatic.com/store_item_assets/steam/apps/" + ($steam_appid|tostring) + "/header.jpg"
      elif $cover_image_hash != null then
        "https://cdn.discordapp.com/app-icons/" + $app_id + "/" + $cover_image_hash + ".png?size=1024"
      elif $icon_hash != null then
        "https://cdn.discordapp.com/app-icons/" + $app_id + "/" + $icon_hash + ".png?size=512"
      else
        null
      end
    ) as $cover_url |
    (
      $app.name | ascii_downcase | gsub("[^a-z0-9]+"; "_") | sub("^_+"; "") | sub("_+$"; "")
    ) as $norm_name |
    ($app.executables // [])[] |
    select(. != null and type == "object" and .is_launcher != true) |
    select(.name != null and (.name | type == "string") and (.name | length) > 0) |
    (.name | gsub("\\\\"; "/") | sub("/+$"; "") | split("/") | last | ascii_downcase | sub("^>+"; "") | sub("\\.exe$"; "") | sub("^\\s+"; "") | sub("\\s+$"; "")) as $exe |
    (
      $exe | gsub("[^a-z0-9]+"; "_") | sub("^_+"; "") | sub("_+$"; "")
    ) as $norm_exe |
    (
      ($norm_exe == $norm_name) or
      (($norm_exe | gsub("_"; "")) == ($norm_name | gsub("_"; "")))
    ) as $is_game_name |
    select(
      (($exe | length) >= 3) and
      (($blacklist | index($exe)) == null) and
      (([$pattern_blacklist[] | . as $p | $exe | test($p; "i")] | any) == false) and
      (($exe | test("^(client|helper|service|launcher|patcher|updater)"; "i")) == false) and
      ($is_game_name or true)
    ) |
    {
      app_id: $app_id,
      app_name: $app.name,
      exe: $exe,
      steam_appid: $steam_appid,
      tags: $tags,
      icon_url: $icon_url,
      cover_url: $cover_url
    }
  ] |
  group_by(.exe) |
  map(sort_by([ (if .icon_url then 1 else 0 end), (if .cover_url then 1 else 0 end), (if .steam_appid then 1 else 0 end), .app_name ]) | .[-1]) |
  group_by(.app_id) |
  map({
    slug: (.[0].app_name | ascii_downcase | gsub("[^a-z0-9]+"; "_") | sub("^_+"; "") | sub("_+$"; "")),
    name: (.[0].app_name | sub("^\\s+"; "") | sub("\\s+$"; "")),
    steam_appid: .[0].steam_appid,
    icon_url: .[0].icon_url,
    cover_url: .[0].cover_url,
    tags: (.[0].tags // []),
    exes: [.[].exe] | unique | sort
  }) |
  group_by(.slug) |
  map(
    sort_by([ (if .icon_url then 1 else 0 end), (if .cover_url then 1 else 0 end), (if .steam_appid then 1 else 0 end), (.exes | length), .name ]) as $entries |
    $entries[-1] as $best |
    $best + {
      steam_appid: ($best.steam_appid // ([$entries[].steam_appid | select(. != null)][0] // null)),
      icon_url: ($best.icon_url // ([$entries[].icon_url | select(. != null)][0] // null)),
      cover_url: ($best.cover_url // ([$entries[].cover_url | select(. != null)][0] // null)),
      tags: ([$entries[].tags[]?] | unique | sort),
      exes: ([$entries[].exes[]?] | unique | sort)
    }
  ) |
  sort_by(.name)
' > "$TMP_FILE"

mv "$TMP_FILE" "$OUT_FILE"
trap - EXIT

jq -r '
  "Saved \(length) games to \(input_filename)",
  "  with icon:         \(map(select(.icon_url != null)) | length)",
  "  with cover:        \(map(select(.cover_url != null)) | length)",
  "  with steam appid:  \(map(select(.steam_appid != null)) | length)",
  "  without exes:      \(map(select((.exes | length) == 0)) | length)"
' "$OUT_FILE"
