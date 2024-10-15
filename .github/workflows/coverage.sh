#!/bin/bash
############ INSTALLATION PARAMETERS ############

passCondition=30    # min percent to be passed
passed=false        # true if all passed
totalCoverage='not initialized'     # all project percentage

############ COVERAGE ############

RED='\033[0;31m'
BLUE='\033[0;34m'
GREEN='\033[0;32m'
GRAY='\033[1;30m'
NC='\033[0m' # No Color

# export CARGO_INCREMENTAL=0
export RUSTFLAGS='-Cinstrument-coverage'
export LLVM_PROFILE_FILE='target/coverage/%p-%m.profraw'

rm -rf ./target/coverage

cargo test --release --no-fail-fast 2>/dev/null

grcov target/coverage -s . --binary-path target/release -o target/coverage --keep-only 'src/*' --output-types html,covdir --ignore 'src/tests/*'

############ REPORT ############

lines=$(jq -r --stream 'select(.[0]|contains(["coveragePercent"])) | "\(.[1]) \t \(.[0]|join("."))"' ./target/coverage/covdir)

# FOR DEBUG
# while IFS= read -r line; do
#     echo "$line"
# done <<< "$lines"
# echo 

regex='([0-9]+(\.[0-9]+)*)[ \t]+([^ \t].+)'
while IFS= read -r line; do
    [[ $line =~ $regex ]]
    percent=${BASH_REMATCH[1]:=""}
    path=${BASH_REMATCH[3]:="${RED} missed ${NC}"}
    path="${path%[. ]coveragePercent}"
    path="${path//children.}"
    path="${path//[[:space:]]}"
    if (( $(echo "$percent >= 30.0" |bc -l) )); then
        echo -e "${GREEN}$(printf %3.2f $percent)${NC} '$path'"
        passed=$($passed && echo true)
    else
        echo -e "${RED}$(printf %3.2f $percent)${NC} '$path'"
        passed=false
    fi
    if [[ $path == 'src' ]]; then
        totalCoverage=$percent
    fi
done <<< "$lines"
if ! $passed; then
    echo -e "TotalCoverage: ${RED}$totalCoverage${NC}"
    echo -e "Coverage passed: ${RED}$passed${NC}"
    exit 1
else
    echo -e "TotalCoverage: ${GREEN}$totalCoverage${NC}"
    echo -e "Coverage passed: ${GREEN}$passed${NC}"
fi
