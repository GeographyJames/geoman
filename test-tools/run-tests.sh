#!/bin/bash
# Run OGC API Features test suite and show quick summary
# Usage: ./run-tests.sh [-q|--quiet] [-o|--open] [-c|--config <config-file>] [-n|--name <test-name>]

QUIET=false
OPEN_BROWSER=false
CONFIG_FILE="test-tools/test-run-props.xml"
TEST_NAME="root"

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -q|--quiet)
            QUIET=true
            shift
            ;;
        -o|--open)
            OPEN_BROWSER=true
            shift
            ;;
        -c|--config)
            CONFIG_FILE="$2"
            shift 2
            ;;
        -n|--name)
            TEST_NAME="$2"
            shift 2
            ;;
        *)
            shift
            ;;
    esac
done

OUTPUT_DIR="test-tools/testng-${TEST_NAME}"

echo "🧪 Running OGC API Features tests: $TEST_NAME"
echo "📋 Config: $CONFIG_FILE"
echo "📁 Output: $OUTPUT_DIR"
echo ""

# Remove previous test output
rm -rf "$OUTPUT_DIR"

# Run the test suite
SEVERE_DETECTED=false

if [ "$QUIET" = true ]; then
    while IFS= read -r line; do
        if echo "$line" | grep -qE "WARN|ERROR|SEVERE"; then
            echo "$line"
        fi
        if echo "$line" | grep -q "SEVERE"; then
            echo ""
            echo "❌ SEVERE error detected! Exiting..."
            SEVERE_DETECTED=true
            break
        fi
    done < <(java -jar test-tools/ets-ogcapi-features10-1.10-SNAPSHOT-aio.jar \
      -h true \
      -o "$OUTPUT_DIR" \
      "$CONFIG_FILE" 2>&1)
else
    while IFS= read -r line; do
        echo "$line"
        if echo "$line" | grep -q "SEVERE"; then
            echo ""
            echo "❌ SEVERE error detected! Exiting..."
            SEVERE_DETECTED=true
            break
        fi
    done < <(java -jar test-tools/ets-ogcapi-features10-1.10-SNAPSHOT-aio.jar \
      -h true \
      -o "$OUTPUT_DIR" \
      "$CONFIG_FILE" 2>&1)
fi

# Exit early if SEVERE error was detected
if [ "$SEVERE_DETECTED" = true ]; then
    exit 1
fi

echo ""
echo "Testing completed."
echo ""
# Find the results file
RESULTS=$(find "$OUTPUT_DIR" -name "testng-results.xml" 2>/dev/null | head -1)

if [ -f "$RESULTS" ]; then
    echo ""
    echo "📊 Test Results Summary ($TEST_NAME)"
    echo "======================="

    # Parse the XML for summary stats
    TOTAL=$(grep -oP 'total="\K[0-9]+' "$RESULTS" | head -1)
    PASSED=$(grep -oP 'passed="\K[0-9]+' "$RESULTS" | head -1)
    FAILED=$(grep -oP 'failed="\K[0-9]+' "$RESULTS" | head -1)
    SKIPPED=$(grep -oP 'skipped="\K[0-9]+' "$RESULTS" | head -1)

    echo "✅ Passed:  $PASSED"
    echo "❌ Failed:  $FAILED"
    echo "⏭️  Skipped: $SKIPPED"
    echo "📝 Total:   $TOTAL"
    echo ""

    # Show failed tests with reasons
    if [ "$FAILED" -gt 0 ]; then
        echo "Failed Tests:"
        echo "-------------"

        # Extract failed test names and their error messages
        grep -A 30 'status="FAIL"' "$RESULTS" | \
            grep -E 'test-method.*name=|<message>|AssertionError' | \
            sed 's/<message>//g; s/<\/message>//g; s/.*name="\([^"]*\)".*/\n❌ \1/g' | \
            grep -v '^$' | \
            head -30
        echo ""
    fi

    # Show report location
    HTML_REPORT=$(find "$OUTPUT_DIR" -name "index.html" 2>/dev/null | head -1)
    echo "📄 Full report: $HTML_REPORT"

    # Open in browser if requested
    if [ "$OPEN_BROWSER" = true ] && [ -f "$HTML_REPORT" ]; then
        echo ""
        echo "🌐 Opening report in browser..."
        xdg-open "$HTML_REPORT" 2>/dev/null || open "$HTML_REPORT" 2>/dev/null || echo "⚠️  Could not open browser automatically"
    fi
else
    echo "⚠️  No results file found"
fi
