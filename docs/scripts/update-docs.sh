#!/bin/bash

# Update Code of Conduct
cd "./docs/code-of-conduct/" || exit
echo "Updating Code of Conduct..."
git pull
cd "../../" || exit
cp "./docs/code-of-conduct/EN.md" "./CODE_OF_CONDUCT.md"
echo "Code of Conduct succesfully updated.."

