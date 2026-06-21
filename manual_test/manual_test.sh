#!/usr/bin/env bash
SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
cd $SCRIPT_DIR/../
## Initialize testing files and vars:
cp "$SCRIPT_DIR/file_a.yaml-tmpl" "$SCRIPT_DIR/file_a.yaml";
cp "$SCRIPT_DIR/file_b.ini-tmpl" "$SCRIPT_DIR/file_b.ini";
echo "$SCRIPT_DIR/file_c.txt" > "$SCRIPT_DIR/crabby_targets"
echo "$SCRIPT_DIR/file_d.txt" >> "$SCRIPT_DIR/crabby_targets"
cp "$SCRIPT_DIR/file_a.yaml-tmpl" "$SCRIPT_DIR/file_c.txt";
cp "$SCRIPT_DIR/file_a.yaml-tmpl" "$SCRIPT_DIR/file_d.txt";
cp "$SCRIPT_DIR/file_e.txt-tmpl" "$SCRIPT_DIR/file_e.txt";
FILE_A_CONTENTS_INITIAL=$(cat "$SCRIPT_DIR/file_a.yaml");
FILE_B_CONTENTS_INITIAL=$(cat "$SCRIPT_DIR/file_b.ini");
FILE_C_CONTENTS_INITIAL=$(cat "$SCRIPT_DIR/file_c.txt");
FILE_D_CONTENTS_INITIAL=$(cat "$SCRIPT_DIR/file_d.txt");
FILE_E_CONTENTS_INITIAL=$(cat "$SCRIPT_DIR/file_e.txt");

## Run with vars set before the command.
CRABBYFIX=QWERTY_ \
CRABBYWAIT=5 \
CRABBYWAITCOUNT=3 \
CRABBYGETS="$SCRIPT_DIR/file_a.yaml,$SCRIPT_DIR/file_b.ini" \
CRABBYGETS_FILE="$SCRIPT_DIR/crabby_targets" \
QWERTY_MAIN_SETTING_FOO="supercalifragilisticexpialidocious" \
QWERTY_MAIN_SETTING_SECRET_FILE="$SCRIPT_DIR/main_secret.txt" \
./target/release/crabbyfig

## Now check if the contents changed
FILE_A_CONTENTS_POST=$(cat "$SCRIPT_DIR/file_a.yaml");
FILE_B_CONTENTS_POST=$(cat "$SCRIPT_DIR/file_b.ini");
FILE_C_CONTENTS_POST=$(cat "$SCRIPT_DIR/file_c.txt");
FILE_D_CONTENTS_POST=$(cat "$SCRIPT_DIR/file_d.txt");
if [[ "$FILE_A_CONTENTS_INITIAL" == "$FILE_A_CONTENTS_POST" ]]; then
  echo "ERROR: file_a.yaml contents did not change.";
  echo "Initial: "
  echo "$FILE_A_CONTENTS_INITIAL" 
  echo "Post: " 
  echo "$FILE_A_CONTENTS_POST"
else
  echo "SUCCESS: file_a.yaml contents changed."
  echo "Changed: " 
  echo "$FILE_A_CONTENTS_POST"
fi

if [[ "$FILE_B_CONTENTS_INITIAL" == "$FILE_B_CONTENTS_POST" ]]; then
  echo "ERROR: file_b.ini contents did not change.";
  echo "Initial: "
  echo "$FILE_B_CONTENTS_INITIAL" 
  echo "Post: " 
  echo "$FILE_B_CONTENTS_POST"
else
  echo "SUCCESS: file_b.ini contents changed."
  echo "Changed: " 
  echo "$FILE_B_CONTENTS_POST"
fi

if [[ "$FILE_C_CONTENTS_INITIAL" == "$FILE_C_CONTENTS_POST" ]]; then
  echo "ERROR: file_c.txt contents did not change.";
  echo "Initial: "
  echo "$FILE_C_CONTENTS_INITIAL" 
  echo "Post: " 
  echo "$FILE_C_CONTENTS_POST"
else
  echo "SUCCESS: file_c.txt contents changed."
  echo "Changed: " 
  echo "$FILE_C_CONTENTS_POST"
fi

if [[ "$FILE_D_CONTENTS_INITIAL" == "$FILE_D_CONTENTS_POST" ]]; then
  echo "ERROR: file_d.txt contents did not change.";
  echo "Initial: "
  echo "$FILE_D_CONTENTS_INITIAL" 
  echo "Post: " 
  echo "$FILE_D_CONTENTS_POST"
else
  echo "SUCCESS: file_d.txt contents changed."
  echo "Changed: "
  echo "$FILE_D_CONTENTS_POST"
fi

## Run for file_e with multiple prefixes.

CRABBYFIX="QWERTY_,QWERTY2_" \
CRABBYWAIT=5 \
CRABBYWAITCOUNT=3 \
CRABBYGETS="$SCRIPT_DIR/file_e.txt" \
QWERTY2_NAME="Mary" \
QWERTY_MAIN_SETTING_FOO="supercalifragilisticexpialidocious" \
./target/release/crabbyfig

FILE_E_CONTENTS_POST=$(cat "$SCRIPT_DIR/file_e.txt");

if [[ "$FILE_E_CONTENTS_INITIAL" == "$FILE_E_CONTENTS_POST" ]]; then
  echo "ERROR: file_e.txt contents did not change.";
  echo "Initial: "
  echo "$FILE_E_CONTENTS_INITIAL" 
  echo "Post: " 
  echo "$FILE_E_CONTENTS_POST"
else
  echo "SUCCESS: file_e.txt contents changed."
  echo "Changed: "
  echo "$FILE_E_CONTENTS_POST"
fi