#!/bin/bash
# Run OGC API Features test suite and show quick summary
# Usage: ./run-tests.sh [-q|--quiet]

QUIET=false
if [[ "$1" == "-q" || "$1" == "--quiet" ]]; then
    QUIET=true
fi

echo "🧪 Running OGC API Features tests..."
echo ""

# Remove previous test output
rm -rf test-tools/testng

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
      -o test-tools/ \
      test-tools/test-run-props.xml 2>&1)
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
      -o test-tools/ \
      test-tools/test-run-props.xml 2>&1)
fi

# Exit early if SEVERE error was detected
if [ "$SEVERE_DETECTED" = true ]; then
    exit 1
fi

echo ""
echo "Testing completed."
echo ""
# Find the results file
RESULTS=$(find test-tools/testng -name "testng-results.xml" 2>/dev/null | head -1)

if [ -f "$RESULTS" ]; then
    echo ""
    echo "📊 Test Results Summary"
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
    HTML_REPORT=$(find test-tools/testng -name "index.html" 2>/dev/null | head -1)
    echo "📄 Full report: $HTML_REPORT"
else
    echo "⚠️  No results file found"
fi
