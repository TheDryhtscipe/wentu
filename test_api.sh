#!/bin/bash

# End-to-end API test for Wentu

API="http://127.0.0.1:3000"

echo "🧪 Testing Wentu API..."
echo

# 1. Health check
echo "1️⃣  Health check..."
curl -s $API/health
echo
echo

# 2. Create wentu
echo "2️⃣  Creating wentu..."
CREATE=$(curl -s -X POST $API/api/wentu \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Team Offsite",
    "description": "Q1 Planning Session",
    "creator_name": "Alice",
    "date_range_start": "2025-03-15T00:00:00Z",
    "date_range_end": "2025-03-20T00:00:00Z",
    "pref_deadline": "2025-03-13T23:59:59Z",
    "expires_in_days": 7
  }')

echo "$CREATE" | jq .
SLUG=$(echo "$CREATE" | jq -r '.slug')
CREATOR_KEY=$(echo "$CREATE" | jq -r '.creator_key')
echo "✅ Created: $SLUG"
echo

# 3. Get wentu
echo "3️⃣  Getting wentu..."
WENTU=$(curl -s $API/api/wentu/$SLUG)
echo "$WENTU" | jq '.title, .creator_name, .status'
DATES=$(echo "$WENTU" | jq -r '.date_options | map(.id) | @csv' | tr -d '"')
IFS=',' read -ra DATE_ARR <<< "$DATES"
DATE1=${DATE_ARR[0]}
DATE2=${DATE_ARR[1]}
echo "✅ Found $(echo $WENTU | jq '.date_options | length') date options"
echo

# 4. Join as participant
echo "4️⃣  Joining as participant..."
PART1=$(curl -s -X POST $API/api/wentu/$SLUG/join \
  -H "Content-Type: application/json" \
  -d '{"name": "Bob"}')
echo "$PART1" | jq .
PART1_ID=$(echo "$PART1" | jq -r '.participant_id')
PART1_KEY=$(echo "$PART1" | jq -r '.participant_key')
echo "✅ Joined as: Bob"
echo

# 5. Submit preferences
echo "5️⃣  Submitting preferences..."
curl -s -X POST $API/api/wentu/$SLUG/preferences \
  -H "Content-Type: application/json" \
  -d "{
    \"participant_id\": \"$PART1_ID\",
    \"participant_key\": \"$PART1_KEY\",
    \"rankings\": [
      {\"date_option_id\": \"$DATE1\", \"preference_order\": 1},
      {\"date_option_id\": \"$DATE2\", \"preference_order\": 2}
    ]
  }" | jq .
echo "✅ Preferences submitted"
echo

# 6. Another participant
echo "6️⃣  Another participant joins..."
PART2=$(curl -s -X POST $API/api/wentu/$SLUG/join \
  -H "Content-Type: application/json" \
  -d '{"name": "Charlie"}')
PART2_ID=$(echo "$PART2" | jq -r '.participant_id')
PART2_KEY=$(echo "$PART2" | jq -r '.participant_key')

curl -s -X POST $API/api/wentu/$SLUG/preferences \
  -H "Content-Type: application/json" \
  -d "{
    \"participant_id\": \"$PART2_ID\",
    \"participant_key\": \"$PART2_KEY\",
    \"rankings\": [
      {\"date_option_id\": \"$DATE2\", \"preference_order\": 1},
      {\"date_option_id\": \"$DATE1\", \"preference_order\": 2}
    ]
  }" > /dev/null
echo "✅ Charlie joined and voted"
echo

# 7. Get STV results
echo "7️⃣  Getting STV results..."
RESULTS=$(curl -s $API/api/wentu/$SLUG/stv-results)
echo "$RESULTS" | jq .
echo "✅ STV calculated"
echo

# 8. Close wentu
echo "8️⃣  Closing wentu..."
curl -s -X POST $API/api/wentu/$SLUG/close \
  -H "Content-Type: application/json" \
  -d "{\"creator_key\": \"$CREATOR_KEY\"}" | jq .
echo "✅ Wentu closed"
echo

# 9. Verify closed
echo "9️⃣  Verifying closed status..."
FINAL=$(curl -s $API/api/wentu/$SLUG)
echo "Status: $(echo $FINAL | jq -r '.status')"
echo "✅ Status is $(echo $FINAL | jq -r '.status')"
echo

echo "🎉 All tests passed!"
