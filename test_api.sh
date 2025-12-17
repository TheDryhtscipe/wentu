#!/bin/bash

set -euo pipefail

# End-to-end API test for Wentu

API="${API:-http://127.0.0.1:3000}"
WRITE_DELAY="${WRITE_DELAY:-0.8}"

json_or_note() {
  local body="$1"
  if [[ -z "$body" ]]; then
    echo "(empty response)"
  else
    echo "$body" | jq .
  fi
}

call_api() {
  local method="$1"
  local path="$2"
  local payload="${3:-}"

  local curl_cmd=(curl -s -w $'\n%{http_code}' -X "$method" "$API$path")
  if [[ "$method" =~ ^(POST|PUT|PATCH)$ ]]; then
    curl_cmd+=(-H "Content-Type: application/json" -d "$payload")
  fi

  local response
  response=$("${curl_cmd[@]}")

  local status="${response##*$'\n'}"
  local body="${response%$'\n'$status}"

  if [[ -z "$status" ]]; then
    echo "Failed to parse HTTP status for $method $path" >&2
    exit 1
  fi

  if (( status < 200 || status >= 300 )); then
    echo "Request $method $path failed with status $status" >&2
    [[ -n "$body" ]] && echo "$body" >&2
    exit 1
  fi

  printf '%s' "$body"
}

throttle_writes() {
  sleep "$WRITE_DELAY"
}

echo "🧪 Testing Wentu API..."
echo

# 1. Health check
echo "1️⃣  Health check..."
curl -s $API/health
echo
echo

# 2. Create wentu
echo "2️⃣  Creating wentu..."
CREATE=$(call_api POST "/api/wentu" '{
  "title": "Team Offsite",
  "description": "Q1 Planning Session",
  "creator_name": "Alice",
  "date_range_start": "2025-03-15T00:00:00Z",
  "date_range_end": "2025-03-20T00:00:00Z",
  "pref_deadline": "2025-03-13T23:59:59Z",
  "expires_in_days": 7
}')
json_or_note "$CREATE"
SLUG=$(echo "$CREATE" | jq -r '.slug')
CREATOR_KEY=$(echo "$CREATE" | jq -r '.creator_key')
echo "✅ Created: $SLUG"
throttle_writes
echo

# 3. Get wentu
echo "3️⃣  Getting wentu..."
WENTU=$(call_api GET "/api/wentu/$SLUG")
echo "$WENTU" | jq '.title, .creator_name, .status'
DATES=$(echo "$WENTU" | jq -r '.date_options | map(.id) | @csv' | tr -d '"')
IFS=',' read -ra DATE_ARR <<< "$DATES"
DATE1=${DATE_ARR[0]}
DATE2=${DATE_ARR[1]}
echo "✅ Found $(echo $WENTU | jq '.date_options | length') date options"
echo

# 4. Join as participant
echo "4️⃣  Joining as participant..."
PART1=$(call_api POST "/api/wentu/$SLUG/join" '{"name": "Bob"}')
json_or_note "$PART1"
PART1_ID=$(echo "$PART1" | jq -r '.participant_id')
PART1_KEY=$(echo "$PART1" | jq -r '.participant_key')
echo "✅ Joined as: Bob"
throttle_writes
echo

# 5. Submit preferences
echo "5️⃣  Submitting preferences..."
PREF1=$(call_api POST "/api/wentu/$SLUG/preferences" "{
  \"participant_id\": \"$PART1_ID\",
  \"participant_key\": \"$PART1_KEY\",
  \"rankings\": [
    {\"date_option_id\": \"$DATE1\", \"preference_order\": 1},
    {\"date_option_id\": \"$DATE2\", \"preference_order\": 2}
  ]
}")
json_or_note "$PREF1"
echo "✅ Preferences submitted"
throttle_writes
echo

# 6. Another participant
echo "6️⃣  Another participant joins..."
PART2=$(call_api POST "/api/wentu/$SLUG/join" '{"name": "Charlie"}')
PART2_ID=$(echo "$PART2" | jq -r '.participant_id')
PART2_KEY=$(echo "$PART2" | jq -r '.participant_key')
throttle_writes

call_api POST "/api/wentu/$SLUG/preferences" "{
  \"participant_id\": \"$PART2_ID\",
  \"participant_key\": \"$PART2_KEY\",
  \"rankings\": [
    {\"date_option_id\": \"$DATE2\", \"preference_order\": 1},
    {\"date_option_id\": \"$DATE1\", \"preference_order\": 2}
  ]
}" >/dev/null
echo "✅ Charlie joined and voted"
throttle_writes
echo

# 7. Get STV results
echo "7️⃣  Getting STV results..."
RESULTS=$(call_api GET "/api/wentu/$SLUG/stv-results")
echo "$RESULTS" | jq .
echo "✅ STV calculated"
echo

# 8. Close wentu
echo "8️⃣  Closing wentu..."
CLOSE=$(call_api POST "/api/wentu/$SLUG/close" "{\"creator_key\": \"$CREATOR_KEY\"}")
json_or_note "$CLOSE"
echo "✅ Wentu closed"
throttle_writes
echo

# 9. Verify closed
echo "9️⃣  Verifying closed status..."
FINAL=$(call_api GET "/api/wentu/$SLUG")
echo "Status: $(echo $FINAL | jq -r '.status')"
echo "✅ Status is $(echo $FINAL | jq -r '.status')"
echo

echo "🎉 All tests passed!"
